use crate::token::Token;
/// 字句解析機
#[derive(Debug)]
pub struct Lexer {
    /// 入力文字列
    input: Vec<char>,
    /// 解析中のindex
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        Self { input, position: 0 }
    }

    /// # Examples
    /// ```
    /// # use calculator::lexer::*;
    /// # use calculator::token::*;
    /// let mut lexer = Lexer::new("1 * (2 + 1)".into());
    /// assert_eq!(lexer.token(), Some(Token::Num(1_f64)));
    /// assert_eq!(lexer.token(), Some(Token::Asterisk));
    /// assert_eq!(lexer.token(), Some(Token::LeftParen));
    /// assert_eq!(lexer.token(), Some(Token::Num(2_f64)));
    /// assert_eq!(lexer.token(), Some(Token::Plus));
    /// assert_eq!(lexer.token(), Some(Token::Num(1_f64)));
    /// assert_eq!(lexer.token(), Some(Token::RightParen));
    /// assert_eq!(lexer.token(), None);
    /// ```
    pub fn token(&mut self) -> Option<Token> {
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
        let current = self.curr()?;
        let result = if current.is_ascii_digit() {
            // 連続する数値を取得する
            let mut digit = vec![*current];
            while self.is_peek_digit() {
                digit.push(*self.peek().unwrap());
                self.next();
            }

            let s = String::from_iter(digit);
            Some(Token::Num(s.parse().unwrap()))
        } else if current.is_ascii_alphabetic() {
            // 連続する文字を取得する
            let mut alphabets = vec![*current];
            while self.is_peek_alphabet() {
                alphabets.push(*self.peek().unwrap());
                self.next();
            }

            let s = String::from_iter(alphabets);
            Token::find(s)
        } else {
            // 記号
            let s = String::from_iter(vec![*current]);
            Token::find(s)
        };

        self.next();
        return result;
    }

    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn is_peek_digit(&mut self) -> bool {
        let peek = self.peek();
        match peek {
            Some(p) => p.is_ascii_digit() || p == &'.',
            None => false,
        }
    }

    fn is_peek_alphabet(&mut self) -> bool {
        let peek = self.peek();
        match peek {
            Some(p) => p.is_ascii_alphabetic(),
            None => false,
        }
    }

    fn next(&mut self) {
        self.position += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    /// 四則演算の字句解析
    fn test_lexer_operator() {
        let mut tables = HashMap::new();
        tables.insert("+", Token::Plus);
        tables.insert("-", Token::Minus);
        tables.insert("*", Token::Asterisk);
        tables.insert("/", Token::Slash);
        for (key, val) in tables {
            let mut lexer = Lexer::new(key.into());
            assert_eq!(lexer.token(), Some(val));
            assert_eq!(lexer.token(), None);
        }
    }

    #[test]
    /// 1桁の整数の字句解析
    fn test_lexer_single_digit_integer() {
        for i in 0..9 {
            let mut lexer = Lexer::new(i.to_string());
            assert_eq!(lexer.token(), Some(Token::Num(f64::from(i))));
            assert_eq!(lexer.token(), None);
        }
    }

    #[test]
    /// 1桁以上の整数の字句解析
    fn test_lexer_some_digit_integer() {
        let mut tables = HashMap::new();
        tables.insert("10", Token::Num(10_f64));
        tables.insert("100", Token::Num(100_f64));
        for (input, expect) in tables {
            let mut lexer = Lexer::new(input.into());
            assert_eq!(lexer.token(), Some(expect));
            assert_eq!(lexer.token(), None);
        }
    }

    #[test]
    /// 浮動小数点の字句解析
    fn test_lexer_some_digit_float() {
        let mut tables = HashMap::new();
        tables.insert("1.0", Token::Num(1.0));
        tables.insert("1.1", Token::Num(1.1));
        tables.insert("10.1", Token::Num(10.1));
        for (input, expect) in tables {
            let mut lexer = Lexer::new(input.into());
            assert_eq!(lexer.token(), Some(expect));
            assert_eq!(lexer.token(), None);
        }
    }

    #[test]
    fn test_lexer_basical_expr() {
        let mut lexer = Lexer::new("1 +2".into());
        assert_eq!(lexer.token(), Some(Token::Num(1_f64)));
        assert_eq!(lexer.token(), Some(Token::Plus));
        assert_eq!(lexer.token(), Some(Token::Num(2_f64)));
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn test_lexer_some_expr() {
        let mut lexer = Lexer::new("1 * (2 + 1)".into());
        assert_eq!(lexer.token(), Some(Token::Num(1_f64)));
        assert_eq!(lexer.token(), Some(Token::Asterisk));
        assert_eq!(lexer.token(), Some(Token::LeftParen));
        assert_eq!(lexer.token(), Some(Token::Num(2_f64)));
        assert_eq!(lexer.token(), Some(Token::Plus));
        assert_eq!(lexer.token(), Some(Token::Num(1_f64)));
        assert_eq!(lexer.token(), Some(Token::RightParen));
        assert_eq!(lexer.token(), None);
    }
}
