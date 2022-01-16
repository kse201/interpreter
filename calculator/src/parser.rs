use crate::{lexer::Lexer, token::Token};
/// 式
#[derive(Debug)]
pub enum Expr {
    /// 数字
    Num(f64),

    /// 前置演算子式
    PrefixExpr { operator: Token, right: Box<Expr> },

    /// 中間演算子
    InfixExpr {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// 構文解析器
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.token();
        let peek = lexer.token();
        Self {
            lexer,
            current,
            peek,
        }
    }

    /// 構文木を返す
    /// # Examples
    ///
    /// ```
    /// # use calculator::{lexer::Lexer, parser::Parser};
    ///  let lexer = Lexer::new("1".chars().collect());
    ///  let mut parser = Parser::new(lexer);
    ///  assert_eq!(format!("{:?}", parser.parse()), r#"Some(Num(1.0))"#);
    /// ```
    ///
    /// ```
    /// # use calculator::{lexer::Lexer, parser::Parser};
    ///  let lexer = Lexer::new("-1".chars().collect());
    ///  let mut parser = Parser::new(lexer);
    ///  assert_eq!(format!("{:?}", parser.parse()), r#"Some(PrefixExpr { operator: Minus, right: Num(1.0) })"#);
    /// ```
    ///
    /// ```
    /// # use calculator::{lexer::Lexer, parser::Parser};
    ///  let lexer = Lexer::new("1+1".chars().collect());
    ///  let mut parser = Parser::new(lexer);
    ///  assert_eq!(format!("{:?}", parser.parse()), r#"Some(InfixExpr { operator: Plus, left: Num(1.0), right: Num(1.0) })"#);
    /// ```
    pub fn parse(&mut self) -> Option<Expr> {
        let mut left = self.parse_prefix()?;
        while self.peek.is_some() {
            self.next();
            left = self.parse_infix(left)?;
        }
        return Some(*left);
    }

    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        match self.current? {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                let token = self.current?;
                self.next();
                let right = self.parse_expression()?;
                Some(Box::new(Expr::InfixExpr {
                    operator: token,
                    left,
                    right,
                }))
            }
            _ => None,
        }
        // let cur = self.current;
    }

    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.current? {
            Token::Num(n) => Some(Box::new(Expr::Num(n))),
            Token::Minus => {
                self.next();
                let right = self.parse_expression()?;
                Some(Box::new(Expr::PrefixExpr {
                    operator: Token::Minus,
                    right: Box::new(*right),
                }))
            }
            Token::LeftParen => unimplemented!("unimplemented Left paren"),
            _ => None,
        }
    }

    fn parse_expression(&mut self) -> Option<Box<Expr>> {
        self.parse_prefix()
    }

    fn next(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.token();
    }
}

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::PrefixExpr { operator, right } => match operator {
            Token::Minus => -eval(right),
            _ => unimplemented!("unimplemented ope: {:?}", operator),
        },
        Expr::InfixExpr {
            operator,
            left,
            right,
        } => match operator {
            Token::Plus => eval(left) + eval(right),
            Token::Minus => eval(left) - eval(right),
            Token::Asterisk => eval(left) * eval(right),
            Token::Slash => eval(left) / eval(right),
            _ => unimplemented!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_parser() {
        do_parser("1", r#"Some(Num(1.0))"#);
    }

    #[cfg(test)]
    fn do_parser(input: &str, expect: &str) {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        assert_eq!(format!("{:?}", parser.parse()), expect);
    }

    #[test]
    fn test_eval_leaf() {
        let lexer = Lexer::new("1".chars().collect());
        let mut parser = Parser::new(lexer);
        assert_eq!(eval(&parser.parse().unwrap()), 1_f64);
    }

    #[test]
    fn test_eval_prefix_expr() {
        let lexer = Lexer::new("-1".chars().collect());
        let mut parser = Parser::new(lexer);
        assert_eq!(eval(&parser.parse().unwrap()), -1_f64);
    }

    #[test]
    fn test_eval_infix_expr() {
        let mut tables = HashMap::new();
        tables.insert("1+1", 2.0);
        tables.insert("1+1+1", 3.0);
        tables.insert("1-1", 0.0);
        tables.insert("2*1", 2.0);
        tables.insert("4.1*2", 8.2);

        for (input, expect) in tables {
            let lexer = Lexer::new(input.into());
            let mut parser = Parser::new(lexer);
            assert_eq!(eval(&parser.parse().unwrap()), expect);
        }
    }
}
