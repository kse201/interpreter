use std::slice::Iter;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Equal,
    If,
    Class,
}

impl Token {
    pub fn iterator() -> Iter<'static, Token> {
        static TOKENS: [Token; 9] = [
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Equal,
            Token::LeftParen,
            Token::RightParen,
            Token::If,
            Token::Class,
        ];
        TOKENS.iter()
    }

    /// List of key associated to action
    pub fn keyword(&self) -> &str {
        match self {
            Token::Plus => &"+",
            Token::Minus => &"-",
            Token::Asterisk => &"*",
            Token::Slash => &"/",
            Token::LeftParen => &"(",
            Token::RightParen => &")",
            Token::Equal => &"=",
            Token::If => &"if",
            Token::Class => &"class",
            _ => unimplemented!(),
        }
    }

    pub fn find(s: String) -> Option<Token> {
        Self::iterator()
            .find(|token| token.keyword() == s)
            .and_then(|s| Some(*s))
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        Self { input, position: 0 }
    }

    pub fn token(&mut self) -> Option<Token> {
        // 空白をスキップする
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
        let mut tables: HashMap<String, Token> = HashMap::new();
        tables.insert("+".into(), Token::Plus);
        tables.insert("-".into(), Token::Minus);
        tables.insert("*".into(), Token::Asterisk);
        tables.insert("/".into(), Token::Slash);
        tables.insert("=".into(), Token::Equal);
        for (key, val) in tables {
            let mut lexer = Lexer::new(key);
            assert_eq!(lexer.token(), Some(val));
            assert_eq!(lexer.token(), None);
        }
    }

    #[test]
    /// キーワードの字句解析
    fn test_lexer_keyword() {
        let mut tables: HashMap<String, Token> = HashMap::new();
        tables.insert("if".into(), Token::If);
        tables.insert("class".into(), Token::Class);
        for (key, val) in tables {
            let mut lexer = Lexer::new(key);
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
        tables.insert("10".into(), Token::Num(10_f64));
        tables.insert("100".into(), Token::Num(100_f64));
        for (input, expect) in tables {
            let mut lexer = Lexer::new(input);
            assert_eq!(lexer.token(), Some(expect));
            assert_eq!(lexer.token(), None);
        }
    }

    #[test]
    /// 浮動小数点の字句解析
    fn test_lexer_some_digit_float() {
        let mut tables = HashMap::new();
        tables.insert("1.0".into(), Token::Num(1.0));
        tables.insert("1.1".into(), Token::Num(1.1));
        tables.insert("10.1".into(), Token::Num(10.1));
        for (input, expect) in tables {
            let mut lexer = Lexer::new(input);
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
