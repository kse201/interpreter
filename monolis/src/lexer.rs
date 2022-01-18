use super::token::Token;

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

    pub fn token(&mut self) -> Option<Token> {
        // 空白を無視する
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        // '(',  ')', '\'', '.' のいずれかの場合、それに対応したTokenを返す
        let curr = self.curr()?;
        let result = Token::find(curr);
        if let Some(_) = result {
            self.next();
            return result;
        }

        // 連続する文字列を取得し、それに対応したTokenを返す
        let mut buf = vec![*curr];
        while self.is_peek_some_chars() {
            buf.push(*self.peek().unwrap());
            self.next();
        }

        self.next();
        return Some(Token::new(buf));
    }

    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn is_peek_some_chars(&mut self) -> bool {
        let peek = self.peek();
        peek.is_some()
            && !peek.unwrap().is_whitespace()
            && Token::separete_chars()
                .find(|c| *c == peek.unwrap())
                .is_none()
    }

    fn next(&mut self) {
        self.position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_with_number() {
        let mut lexer = Lexer::new("(+ 1 1)".into());
        assert_eq!(lexer.token(), Some(Token::LPAREN));
        assert_eq!(
            lexer.token(),
            Some(Token::SYMBOL {
                buf: "+".to_string(),
            })
        );

        assert_eq!(lexer.token(), Some(Token::NUMBER { val: 1.0 }));

        assert_eq!(lexer.token(), Some(Token::NUMBER { val: 1.0 }));

        assert_eq!(lexer.token(), Some(Token::RPAREN));
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn test_lexer_with_other() {
        let mut lexer = Lexer::new("(+ 'a 1)".into());
        assert_eq!(lexer.token(), Some(Token::LPAREN));
        assert_eq!(
            lexer.token(),
            Some(Token::SYMBOL {
                buf: "+".to_string(),
            })
        );

        assert_eq!(lexer.token(), Some(Token::QUOTE));

        assert_eq!(
            lexer.token(),
            Some(Token::SYMBOL {
                buf: "a".to_string(),
            })
        );

        assert_eq!(lexer.token(), Some(Token::NUMBER { val: 1.0 }));

        assert_eq!(lexer.token(), Some(Token::RPAREN));
        assert_eq!(lexer.token(), None);
    }
}
