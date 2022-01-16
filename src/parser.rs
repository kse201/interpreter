use crate::{lexer::Lexer, token::Token};
/// 式
#[derive(Debug)]
pub enum Expr {
    /// 数字
    Num(f64),
}

/// 構文解析器
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    /// 構文木を返す
    /// # Examples
    ///
    /// ```
    /// # use rs_ruby::{lexer::Lexer, parser::Parser};
    ///  let lexer = Lexer::new("1".chars().collect());
    ///  let mut parser = Parser::new(lexer);
    ///  assert_eq!(format!("{:?}", parser.parse()), r#"Some(Num(1.0))"#);
    /// ```
    pub fn parse(&mut self) -> Option<Expr> {
        let expr = self.lexer.token().and_then(|token| match token {
            Token::Num(n) => Some(Expr::Num(n)),
            _ => unimplemented!(),
        });
        return expr;
    }
}

#[cfg(test)]
mod tests {
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
}
