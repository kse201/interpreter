use super::lexer::Lexer;
use super::token::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
/// セル
pub enum Cell {
    /// nil
    NIL,

    /// Number
    /// 1.0 ...
    NUMBER { val: f64 },

    /// Symbol
    /// quote
    SYMBOL { name: String },

    /// Cons
    CONS { car: Sexp, cdr: Sexp },

    /// 組み込み関数
    SUBR { subr: fn(Sexp) -> Sexp },

    /// 引数を評価しない組み込み関数
    FSUBR,

    /// 関数
    FUNC,
}

/// S式
type Sexp = Box<Cell>;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NUMBER { val } => {
                write!(f, "{}", val)
            }

            Self::SYMBOL { name } => {
                write!(f, "{}", name)
            }
            Self::CONS { .. } => {
                f.write_str("(")?;
                self.fmt_list(f)
            }

            _ => unimplemented!(),
        }
    }
}

impl Cell {
    pub fn nil() -> Sexp {
        Box::new(Cell::NIL)
    }

    pub fn number(val: f64) -> Sexp {
        Box::new(Self::NUMBER { val })
    }

    pub fn symbol(name: String) -> Sexp {
        Box::new(Self::SYMBOL { name })
    }

    pub fn cons(car: Sexp, cdr: Sexp) -> Sexp {
        Box::new(Self::CONS { car, cdr })
    }

    fn fmt_list(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CONS { car, cdr } => {
                if car.is_nil() {
                    return write!(f, ")");
                }
                if cdr.is_some() && !cdr.is_list() {
                    write!(f, "{} . {})", car.as_ref(), cdr.as_ref())
                } else {
                    let r = write!(f, "{}", car.as_ref());
                    if !cdr.is_nil() {
                        r.and(write!(f, " ").and(cdr.as_ref().fmt_list(f)))
                    } else {
                        r.and(write!(f, ")"))
                    }
                }
            }
            _ => todo!(),
        }
    }

    pub fn car(&self) -> Sexp {
        match self {
            Cell::CONS { car, .. } => car.clone(),
            _ => Cell::nil(),
        }
    }

    pub fn cdr(&self) -> Sexp {
        match self {
            Cell::CONS { cdr, .. } => cdr.clone(),
            _ => Cell::nil(),
        }
    }

    pub fn cadr(&self) -> Sexp {
        self.cdr().car()
    }

    pub fn caar(&self) -> Sexp {
        self.car().car()
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::SYMBOL { name } => Some(name.to_string()),
            _ => None,
        }
    }

    fn is_nil(&self) -> bool {
        match self {
            Cell::NIL => true,
            _ => false,
        }
    }

    fn is_some(&self) -> bool {
        !self.is_nil()
    }

    fn is_atom(&self) -> bool {
        self.is_number() || self.is_symbol()
    }

    fn is_symbol(&self) -> bool {
        match self {
            Cell::SYMBOL { .. } => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Cell::NUMBER { .. } => true,
            _ => false,
        }
    }

    fn is_list(&self) -> bool {
        match self {
            Cell::CONS { .. } => true,
            _ => false,
        }
    }

    fn is_subr(&self) -> bool {
        match self {
            Cell::SUBR { .. } => true,
            _ => false,
        }
    }

    fn is_fsubr(&self) -> bool {
        match self {
            Cell::FSUBR { .. } => true,
            _ => false,
        }
    }

    fn is_function(&self) -> bool {
        match self {
            Cell::FUNC { .. } => true,
            _ => false,
        }
    }

    pub fn iter<'a>(&'a self) -> std::vec::IntoIter<&'a Cell> {
        let v = match self {
            Cell::CONS { car, cdr } => {
                let mut v = vec![];
                if car.is_some() {
                    v.push(car.as_ref());
                }
                if cdr.is_some() {
                    v.push(cdr.as_ref());
                }
                v
            }
            _ => Vec::new(),
        };
        v.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> std::vec::IntoIter<&'a mut Cell> {
        let v = match self {
            Cell::CONS { car, cdr } => {
                if car == cdr {
                    panic!()
                }
                let mut v = vec![];
                if car.is_some() {
                    v.push(car.as_mut());
                }
                if cdr.is_some() {
                    v.push(cdr.as_mut());
                }
                v
            }
            _ => Vec::new(),
        };
        v.into_iter()
    }
}

impl IntoIterator for Cell {
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        let v = match self {
            Cell::CONS { car, cdr } => vec![*car, *cdr],
            _ => Vec::new(),
        };
        v.into_iter()
    }
}

pub struct Parser {
    lexer: Lexer,
    current: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.token();
        Parser { lexer, current }
    }

    fn next(&mut self) {
        self.current = self.lexer.token();
    }

    pub fn parse(&mut self) -> Sexp {
        match self.current() {
            Some(Token::NUMBER { val }) => Cell::number(*val),
            Some(Token::SYMBOL { buf }) => Cell::symbol(buf.to_string()),
            Some(Token::QUOTE) => {
                self.next();
                Cell::cons(
                    Cell::symbol("quote".to_string()),
                    Cell::cons(self.parse(), Cell::nil()),
                )
            }
            Some(Token::LPAREN) => self.parse_list(),
            Some(Token::RPAREN) => panic!("parse error"),
            None => Cell::nil(),
            _ => unimplemented!("parse error {:?}", self.current()),
        }
    }

    fn parse_list(&mut self) -> Sexp {
        self.next();
        // read list
        match self.current() {
            Some(Token::RPAREN) => Cell::nil(),
            Some(Token::DOT) => {
                self.next();
                let cdr = self.parse();
                if cdr.is_atom() {
                    self.next();
                }
                cdr
            }
            Some(_) => {
                let car = self.parse();
                let cdr = self.parse_list();
                Cell::cons(car, cdr)
            }
            None => Cell::nil(),
        }
    }

    fn current(&mut self) -> Option<&Token> {
        self.current.as_ref()
    }
}

pub fn eval(sexp: Sexp) -> Sexp {
    match sexp.as_ref() {
        Cell::NUMBER { .. } => sexp.clone(),
        Cell::SYMBOL { name } => {
            todo!()
            // let cell = find_sym(name);
            // match ref {
            // Some(e) => { return cell ;}
            // None=> { panic!("eval error")}
            // }
        }
        Cell::CONS { .. } => {
            if sexp.car().is_symbol() && (Some("quote".to_string()) == sexp.car().name()) {
                sexp.cadr()
            } else if sexp.car().is_number() {
                panic!("Arg Error")
            } else if sexp.car().is_subr() {
                unimplemented!()
            } else if sexp.car().is_fsubr() {
                unimplemented!()
            } else if sexp.car().is_function() {
                unimplemented!()
            } else {
                unreachable!()
            }
        }
        _ => unimplemented!(),
    }
}

fn evlis(sexp: Sexp) -> Sexp {
    if sexp.is_nil() {
        Cell::nil()
    } else {
        Cell::cons(sexp.car(), evlis(sexp.cdr()))
    }
}

fn apply(func: Sexp, args: Sexp, env: Sexp) -> Sexp {
    let sym = find_sym(func.name().unwrap(), env);
    if sym.is_nil() {
        panic!()
    } else {
        match sym.as_ref() {
            Cell::SUBR { subr } => subr(args),
            Cell::FSUBR => {
                unimplemented!()
            }
            Cell::FUNC => {
                unimplemented!()
            }
            _ => unreachable!(),
        }
    }
}

fn f_plus(args: Sexp) -> Sexp {
    let mut res = 0.0;
    let mut curr = args;
    while curr.is_some() {
        let car = curr.car();
        let arg = match *car {
            Cell::NUMBER { val } => val,
            _ => panic!(),
        };
        curr = curr.cdr();
        res += arg;
    }
    Cell::number(res)
}

fn find_sym(name: String, list: Sexp) -> Sexp {
    let addr = assoc(Cell::symbol(name), list);
    addr.cdr()
}

fn assoc(sym: Sexp, list: Sexp) -> Sexp {
    match list.as_ref() {
        Cell::NIL => list,
        _ => {
            if list.caar() == sym {
                list.car()
            } else {
                assoc(sym, list.cdr())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_as_is() {
        let lexer = Lexer::new("(+ (+ 1 2) 3)".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!("(+ (+ 1 2) 3)", format!("{}", tree),);
        tree.iter().for_each(|f| println!("{:?} ", f));

        let lexer = Lexer::new("(+   (+ 1  2) 3)".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!("(+ (+ 1 2) 3)", format!("{}", tree),);
        tree.iter().for_each(|f| println!("{:?} ", f));
    }

    #[test]
    fn test_parse_with_quote() {
        let lexer = Lexer::new("'1".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!("(quote 1)", format!("{}", tree),);
    }

    #[test]
    fn test_f_plus() {
        let lexer = Lexer::new("(1 2 3)".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!(Cell::number(6.0), f_plus(tree));
    }

    #[test]
    fn test_find() {
        let lexer = Lexer::new("((a . 1) (b . 2))".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!(Cell::number(1.0), find_sym("a".to_string(), tree.clone()));
        assert_eq!(Cell::number(2.0), find_sym("b".to_string(), tree));
        let lexer = Lexer::new("(+ 1 2)".chars().collect());
        let tree = Parser::new(lexer).parse();

        let f_plus = Box::new(Cell::SUBR { subr: f_plus });
        let env = Cell::cons(Cell::cons(Cell::symbol("+".into()), f_plus), Cell::nil());
        assert_eq!(Cell::number(3.0), apply(tree.car(), evlis(tree.cdr()), env));
    }

    #[test]
    fn test_apply() {
        let lexer = Lexer::new("(+ 1 2)".chars().collect());
        let tree = Parser::new(lexer).parse();

        let f_plus = Box::new(Cell::SUBR { subr: f_plus });
        let env = Cell::cons(Cell::cons(Cell::symbol("+".into()), f_plus), Cell::nil());
        assert_eq!(Cell::number(3.0), apply(tree.car(), evlis(tree.cdr()), env));
    }
}
