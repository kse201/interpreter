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

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn is_peek_some_chars(&mut self) -> bool {
        let peek = self.peek();
        peek.is_some()
            && !peek.unwrap().is_whitespace()
            && !Token::separate_chars().any(|c| c == peek.unwrap())
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
        while self.is_peek_some_chars() {
            buf.push(*self.peek().unwrap());
            self.next();
        }

        self.next();
        Some(Token::new(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_with_leaf() {
        let mut lexer = Lexer::new("1".into());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(None, lexer.token());
    }

    #[test]
    fn test_lexer_with_quote() {
        let mut lexer = Lexer::new("'1".into());
        assert_eq!(Some(Token::QUOTE), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(None, lexer.token());

        let mut lexer = Lexer::new("'a".into());
        assert_eq!(Some(Token::QUOTE), lexer.token());
        assert_eq!(Some(Token::SYMBOL { buf: "a".into() }), lexer.token());
        assert_eq!(None, lexer.token());
    }

    #[test]
    fn test_lexer_with_cons() {
        let mut lexer = Lexer::new("(1 . 1)".into());
        assert_eq!(Some(Token::LPAREN), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::DOT), lexer.token());
        assert_eq!(Some(Token::NUMBER { val: 1.0 }), lexer.token());
        assert_eq!(Some(Token::RPAREN), lexer.token());
        assert_eq!(None, lexer.token());
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
