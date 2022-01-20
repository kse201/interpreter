use super::token::{Token, Tokenize};
use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::fmt;
use std::{cell::RefCell, rc::Rc};

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
    SUBR {
        name: String,
        subr: fn(Sexp) -> Result<Sexp>,
    },

    /// 引数を評価しない組み込み関数
    FSUBR {
        name: String,
        fsubr: fn(Sexp, Env) -> Result<Sexp>,
    },

    /// 関数
    FUNC,
}

#[macro_export]
macro_rules! nil {
    () => {
        Cell::nil()
    };
}

#[macro_export]
macro_rules! num {
    ($num:expr) => {
        Cell::number($num.into())
    };
}

#[macro_export]
macro_rules! sym {
    ($name:expr) => {
        Cell::symbol($name.to_string())
    };
}

#[macro_export]
macro_rules! cons {
    ($car:expr, $cdr:expr) => {
        Cell::cons($car, $cdr)
    };
}

/// S式
pub type Sexp = Box<Cell>;
pub type Env = Rc<RefCell<HashMap<String, Sexp>>>;

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
            Self::SUBR { name, .. } => write!(f, "<subr: {}>", name),
            Self::FSUBR { name, .. } => write!(f, "<fsubr: {}>", name),
            Self::FUNC { .. } => write!(f, "<function>"),
            Self::NIL { .. } => f.write_str("nil"),
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

    pub fn subr(name: String, func: fn(Sexp) -> Result<Sexp>) -> Sexp {
        Box::new(Self::SUBR { name, subr: func })
    }

    pub fn fsubr(name: String, func: fn(Sexp, Env) -> Result<Sexp>) -> Sexp {
        Box::new(Self::FSUBR { name, fsubr: func })
    }

    fn fmt_list(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CONS { car, cdr } => {
                if car.is_nil() {
                    return write!(f, ")");
                }
                if cdr.is_value() && !cdr.is_list() {
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
            _ => nil!(),
        }
    }

    pub fn cdr(&self) -> Sexp {
        match self {
            Cell::CONS { cdr, .. } => cdr.clone(),
            _ => nil!(),
        }
    }

    pub fn cadr(&self) -> Sexp {
        self.cdr().car()
    }

    pub fn caar(&self) -> Sexp {
        self.car().car()
    }

    pub fn set_car(&mut self, cell: Sexp) {
        if let Cell::CONS { car, .. } = self {
            *car = cell;
        }
    }

    pub fn set_cdr(&mut self, cell: Sexp) {
        if let Cell::CONS { cdr, .. } = self {
            *cdr = cell;
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::SYMBOL { name } => Some(name.to_string()),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Cell::NIL)
    }

    pub fn is_value(&self) -> bool {
        !self.is_nil()
    }

    fn is_atom(&self) -> bool {
        self.is_number() || self.is_symbol()
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Cell::SYMBOL { .. })
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Cell::NUMBER { .. })
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Cell::CONS { .. })
    }

    pub fn is_subr(&self) -> bool {
        matches!(self, Cell::SUBR { .. })
    }

    pub fn is_fsubr(&self) -> bool {
        matches!(self, Cell::FSUBR { .. })
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Cell::FUNC { .. })
    }

    pub fn iter(&self) -> std::vec::IntoIter<&Cell> {
        let v = match self {
            Cell::CONS { car, cdr } => {
                let mut v = vec![];
                if car.is_value() {
                    v.push(car.as_ref());
                }
                if cdr.is_value() {
                    v.push(cdr.as_ref());
                }
                v
            }
            _ => Vec::new(),
        };
        v.into_iter()
    }
    pub fn iter_mut(&mut self) -> std::vec::IntoIter<&mut Cell> {
        let v = match self {
            Cell::CONS { car, cdr } => {
                if car == cdr {
                    panic!()
                }
                let mut v = vec![];
                if car.is_value() {
                    v.push(car.as_mut());
                }
                if cdr.is_value() {
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

    pub fn parse(&mut self) -> Result<Sexp> {
        match self.current() {
            Some(Token::NUMBER { val }) => Ok(num!(*val)),
            Some(Token::SYMBOL { buf }) => Ok(sym!(buf.to_string())),
            Some(Token::QUOTE) => {
                self.next();
                Ok(cons!(sym!("quote"), cons!(self.parse()?, nil!())))
            }
            Some(Token::LPAREN) => self.parse_list(),
            Some(Token::RPAREN) => Err(anyhow!("Un expected token ')'")),
            None => Ok(nil!()),
            _ => Err(anyhow!("parse error {:?}", self.current())),
        }
    }

    fn parse_list(&mut self) -> Result<Sexp> {
        self.next();
        // read list
        match self.current() {
            Some(Token::RPAREN) => Ok(nil!()),
            Some(Token::DOT) => {
                self.next();
                let cdr = self.parse()?;
                if cdr.is_atom() {
                    self.next();
                }
                Ok(cdr)
            }
            Some(_) => {
                let car = self.parse()?;
                let cdr = self.parse_list()?;
                Ok(cons!(car, cdr))
            }
            None => Ok(nil!()),
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
        let tree = Parser::new(lexer).parse().unwrap();
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
        let tree = Parser::new(lexer).parse().unwrap();
        assert_eq!("(+ 1 (+ 2 3))", format!("{}", tree));
    }
}
