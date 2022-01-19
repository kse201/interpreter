use super::token::{Token, Tokenize};
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
pub type Sexp = Box<Cell>;

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

    pub fn subr(func: fn(Sexp) -> Sexp) -> Sexp {
        Box::new(Self::SUBR { subr: func })
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

    pub fn set_car(&mut self, cell: Sexp) {
        match self {
            Cell::CONS { car, .. } => {
                *car = cell;
            }
            _ => {}
        }
    }

    pub fn set_cdr(&mut self, cell: Sexp) {
        match self {
            Cell::CONS { cdr, .. } => {
                *cdr = cell;
            }
            _ => {}
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::SYMBOL { name } => Some(name.to_string()),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Cell::NIL => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_nil()
    }

    fn is_atom(&self) -> bool {
        self.is_number() || self.is_symbol()
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Cell::SYMBOL { .. } => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Cell::NUMBER { .. } => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Cell::CONS { .. } => true,
            _ => false,
        }
    }

    pub fn is_subr(&self) -> bool {
        match self {
            Cell::SUBR { .. } => true,
            _ => false,
        }
    }

    pub fn is_fsubr(&self) -> bool {
        match self {
            Cell::FSUBR { .. } => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
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

pub struct Parser<T: Tokenize> {
    lexer: T,
    current: Option<Token>,
}

impl<T: Tokenize> Parser<T> {
    pub fn new(mut lexer: T) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, Tokenize};

    struct MockLexer {
        tokens: Vec<Token>,
        position: usize,
    }

    impl MockLexer {
        pub fn new(tokens: Vec<Token>) -> Self {
            Self {
                tokens,
                position: 0,
            }
        }
    }

    impl Tokenize for MockLexer {
        fn token(&mut self) -> Option<Token> {
            let token = self.tokens.get(self.position)?;
            self.position += 1;
            Some(token.clone())
        }
    }

    #[test]
    fn test_parse_tree() {
        let tokens = vec![
            Token::LPAREN,
            Token::new("+".chars().collect()),
            Token::new("1".chars().collect()),
            Token::new("2".chars().collect()),
            Token::RPAREN,
        ];
        let lexer = MockLexer::new(tokens);
        let tree = Parser::new(lexer).parse();
        assert_eq!("(+ 1 2)", format!("{}", tree));
    }

    #[test]
    fn test_eval_plus() {
        let tokens = vec![
            Token::LPAREN,
            Token::new("+".chars().collect()),
            Token::new("1".chars().collect()),
            Token::LPAREN,
            Token::new("+".chars().collect()),
            Token::new("2".chars().collect()),
            Token::new("3".chars().collect()),
            Token::RPAREN,
            Token::RPAREN,
        ];
        let lexer = MockLexer::new(tokens);
        let tree = Parser::new(lexer).parse();
        assert_eq!("(+ 1 (+ 2 3))", format!("{}", tree));
    }
}
