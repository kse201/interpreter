use super::lexer::Lexer;
use super::token::Token;

#[derive(Debug, Clone)]
pub enum Cell {
    /// Number
    /// 1.0 ...
    NUMBER { val: f64 },

    /// Symbol
    /// quote
    SYMBOL { name: String },

    /// Cons
    CONS {
        car: Option<Box<Cell>>,
        cdr: Option<Box<Cell>>,
    },

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
            Self::CONS { .. } => write!(f, "(").and(self.fmt_list(f)),
            _ => unimplemented!(),
        }
    }
}

impl Cell {
    fn fmt_list(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CONS { car, cdr } => {
                if car.is_none() {
                    return write!(f, ")");
                }
                if cdr.is_some() && !&cdr.as_ref().unwrap().is_list() {
                    write!(f, "{} . {})", "car", "cdr")
                } else {
                    let r = write!(f, "{}", car.as_ref().unwrap());
                    if !cdr.is_none() {
                        r.and(write!(f, " ").and(cdr.as_ref().unwrap().fmt_list(f)))
                    } else {
                        r.and(write!(f, ")"))
                    }
                }
            }
            _ => todo!(),
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            Self::CONS { .. } => true,
            _ => false,
        }
    }

    pub fn car(&self) -> Option<Box<Cell>> {
        match self {
            Cell::CONS { car, .. } => car.clone(),
            _ => None,
        }
    }

    pub fn cdr(&self) -> Option<Box<Cell>> {
        match self {
            Cell::CONS { cdr, .. } => cdr.clone(),
            _ => None,
        }
    }

    pub fn cadr(&self) -> Option<Box<Cell>> {
        self.cdr().and_then(|cdr| cdr.car())
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::SYMBOL { name } => Some(name.to_string()),
            _ => None,
        }
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

    pub fn parse(&mut self) -> Option<Box<Sexp>> {
        match self.current()? {
            Token::NUMBER { val } => Some(Box::new(Cell::NUMBER { val: *val })),
            Token::SYMBOL { buf } => Some(Box::new(Cell::SYMBOL {
                name: buf.to_string(),
            })),
            Token::QUOTE => {
                self.next();
                Some(Box::new(Cell::CONS {
                    car: Some(Box::new(Cell::SYMBOL {
                        name: "quote".to_string(),
                    })),
                    cdr: Some(Box::new(Cell::CONS {
                        car: self.parse(),
                        cdr: None,
                    })),
                }))
            }
            Token::LPAREN => self.parse_list(),
            Token::RPAREN => panic!("parse error"),
            _ => unimplemented!(),
        }
    }

    fn parse_list(&mut self) -> Option<Box<Sexp>> {
        self.next();
        // read list
        match self.current()? {
            Token::RPAREN => None,
            Token::DOT => {
                todo!()
            }
            _ => {
                let car = self.parse();
                let cdr = self.parse_list();
                Some(Box::new(Sexp::CONS { car, cdr }))
            }
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
            if symbolp(sexp.car())
                && (Some("quote".to_string()) == sexp.car().and_then(|car| car.name()))
            {
                return *sexp.cadr().unwrap();
            } else if numberp(sexp.car()) {
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

fn symbolp(p: Option<Box<Cell>>) -> bool {
    match p {
        None => false,
        Some(cell) => match cell.as_ref() {
            Cell::SYMBOL { .. } => true,
            _ => false,
        },
    }
}

fn numberp(p: Option<Box<Cell>>) -> bool {
    match p {
        None => false,
        Some(cell) => match cell.as_ref() {
            Cell::NUMBER { .. } => true,
            _ => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let lexer = Lexer::new("(+ (+ 1 2) 3)".chars().collect());
        let tree = Parser::new(lexer).parse().unwrap();
        assert_eq!("(+ (+ 1 2) 3)", format!("{}", tree),);
    }

    #[test]
    fn test_parser_quote() {
        let lexer = Lexer::new("'1".chars().collect());
        let tree = Parser::new(lexer).parse().unwrap();
        assert_eq!("(quote 1)", format!("{}", tree),);
    }
}
