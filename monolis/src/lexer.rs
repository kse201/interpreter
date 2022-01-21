use super::token::{Token, Tokenize};

/// 字句解析器
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

    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }

    fn prev(&mut self) -> Option<&char> {
        self.input.get(self.position - 1)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn is_peek_some_chars(&mut self) -> bool {
        if let Some(peek) = self.peek() {
            !peek.is_whitespace() && !Token::separate_chars().any(|c| c == peek)
        } else {
            false
        }
    }

    fn is_string_chars(&mut self) -> bool {
        if let Some('"') = self.curr() {
            if let Some('\\') = self.prev() {
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    fn next(&mut self) {
        self.position += 1;
    }
}

impl Tokenize for Lexer {
    fn token(&mut self) -> Option<Token> {
        // 空白を無視する
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        // '(',  ')', '\'', '.' のいずれかの場合、それに対応したTokenを返す
        let curr = self.curr()?;
        let result = Token::find(curr);
        if result.is_some() {
            self.next();
            return result;
        }

        // 連続する文字列を取得し、それに対応したTokenを返す
        let mut buf = vec![*curr];
        match self.curr() {
            // 文字列
            Some('"') => {
                buf.push(*self.peek().unwrap());
                self.next();
                while self.is_string_chars() {
                    buf.push(*self.peek().unwrap());
                    self.next();
                }
            }
            // それ以外
            _ => {
                while self.is_peek_some_chars() {
                    buf.push(*self.peek().unwrap());
                    self.next();
                }
            }
        }

        self.next();
        Some(Token::new(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(expect: Vec<Token>, input: &str) {
        let mut lexer = Lexer::new(input.into());
        expect.iter().for_each(|e| {
            assert_eq!(Some(e.clone()), lexer.token());
        });
        assert_eq!(None, lexer.token());
    }

    #[test]
    fn test_lexer_with_leaf() {
        assert_tokens(vec![Token::NUMBER { val: 1.0 }], "1");
    }

    #[test]
    fn test_lexer_with_quote() {
        assert_tokens(vec![Token::QUOTE, Token::NUMBER { val: 1.0 }], "'1");
        assert_tokens(vec![Token::QUOTE, Token::SYMBOL { buf: "a".into() }], "'a");
    }

    #[test]
    fn test_lexer_with_cons() {
        assert_tokens(
            vec![
                Token::LPAREN,
                Token::NUMBER { val: 1.0 },
                Token::DOT,
                Token::NUMBER { val: 1.0 },
                Token::RPAREN,
            ],
            "(1 . 1)",
        )
    }

    #[test]
    fn test_lexer_with_string() {
        assert_tokens(
            vec![Token::STRING {
                buf: "string".into(),
            }],
            r#""string""#,
        );
        assert_tokens(
            vec![Token::STRING {
                buf: r#"string with \" double-quote."#.into(),
            }],
            "\"string with \\\" double-quote.\"",
        );
    }

    #[test]
    fn test_lexer_with_a_branch() {
        let mut lexer = Lexer::new("(+ 1 1)".into());
        assert_eq!(Some(Token::LPAREN), lexer.token());
        assert_eq!(Some(Token::SYMBOL { buf: "+".into() }), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::RPAREN), lexer.token());
        assert_eq!(None, lexer.token());
    }

    #[test]
    fn test_lexer_with_some_branch() {
        let mut lexer = Lexer::new("(+ (+ 1 1) 1)".into());
        assert_eq!(Some(Token::LPAREN), lexer.token());
        assert_eq!(Some(Token::SYMBOL { buf: "+".into() }), lexer.token());
        assert_eq!(Some(Token::LPAREN), lexer.token());
        assert_eq!(Some(Token::SYMBOL { buf: "+".into() }), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::RPAREN), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::RPAREN), lexer.token());
        assert_eq!(None, lexer.token());
    }
}
