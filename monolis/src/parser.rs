use super::lexer::Lexer;
use super::token::Token;

#[derive(Debug, Clone)]
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
    CONS { car: Box<Cell>, cdr: Box<Cell> },

    /// 組み込み関数
    SUBR,

    /// 引数を評価しない組み込み関数
    FSUBR,

    /// 関数
    FUNC,
}

use std::fmt;
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
    pub fn nil() -> Box<Cell> {
        Box::new(Cell::NIL)
    }

    pub fn number(val: f64) -> Box<Cell> {
        Box::new(Self::NUMBER { val })
    }

    pub fn symbol(name: String) -> Box<Cell> {
        Box::new(Self::SYMBOL { name })
    }

    pub fn cons(car: Box<Cell>, cdr: Box<Cell>) -> Box<Cell> {
        Box::new(Self::CONS { car, cdr })
    }

    fn fmt_list(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CONS { car, cdr } => {
                if car.is_nil() {
                    return write!(f, ")");
                }
                if cdr.is_some() && !cdr.is_list() {
                    write!(f, "{} . {})", "car", "cdr")
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

    pub fn car(&self) -> Box<Cell> {
        match self {
            Cell::CONS { car, .. } => car.clone(),
            _ => Cell::nil(),
        }
    }

    pub fn cdr(&self) -> Box<Cell> {
        match self {
            Cell::CONS { cdr, .. } => cdr.clone(),
            _ => Cell::nil(),
        }
    }

    pub fn cadr(&self) -> Box<Cell> {
        self.cdr().car()
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

type Sexp = Cell;

pub struct Parser {
    lexer: Lexer,
    current: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.token();
        let peek = lexer.token();
        Parser {
            lexer,
            current,
            peek,
        }
    }

    fn next(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.token();
    }

    pub fn parse(&mut self) -> Box<Sexp> {
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
            _ => unimplemented!(),
        }
    }

    fn parse_list(&mut self) -> Box<Sexp> {
        self.next();
        // read list
        match self.current() {
            Some(Token::RPAREN) => Cell::nil(),
            Some(Token::DOT) => {
                todo!()
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

pub fn eval(sexp: &Sexp) -> Cell {
    match sexp {
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
                return *sexp.cadr();
            } else if sexp.car().is_number() {
                panic!("Arg Error")
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

// fn evlis(sexp: &Sexp) -> Cell {
// Cell::CONS {
// car: eval(sexp.car()),
// cdr: evlis(sexp.cdr()),
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let lexer = Lexer::new("(+ (+ 1 2) 3)".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!("(+ (+ 1 2) 3)", format!("{}", tree),);
    }

    #[test]
    fn test_parser_quote() {
        let lexer = Lexer::new("'1".chars().collect());
        let tree = Parser::new(lexer).parse();
        assert_eq!("(quote 1)", format!("{}", tree),);
    }
}
