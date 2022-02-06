use std::slice::Iter;

#[derive(Debug, PartialEq, Clone, Copy)]
/// 字句
pub enum Token {
    /// 数字
    Num(f64),

    /// \+
    Plus,
    /// \-
    Minus,
    /// \*
    Asterisk,
    /// /
    Slash,
    /// (
    LeftParen,
    /// )
    RightParen,
}

impl Token {
    fn iterator() -> Iter<'static, Token> {
        static TOKENS: [Token; 6] = [
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::LeftParen,
            Token::RightParen,
        ];
        TOKENS.iter()
    }

    fn keyword(&self) -> &str {
        match self {
            Token::Plus => &"+",
            Token::Minus => &"-",
            Token::Asterisk => &"*",
            Token::Slash => &"/",
            Token::LeftParen => &"(",
            Token::RightParen => &")",
            _ => unimplemented!("unimplemented keyword: {:?}", self),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Asterisk | Token::Slash => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }

    /// キーワードに対応する字句を返す
    ///
    /// # Examples
    /// ```
    /// # use calculator::token::*;
    /// assert_eq!(Token::find("+".into()), Some(Token::Plus));
    /// assert_eq!(Token::find("1".into()), None);
    /// assert_eq!(Token::find("u".into()), None);
    /// ```
    pub fn find(s: String) -> Option<Token> {
        Self::iterator()
            .find(|token| token.keyword() == s)
            .and_then(|s| Some(*s))
    }
}

/// 優先順位
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    /// '+', '-'
    SUM,
    /// '*', '/'
    PRODUCT,

    /// 前置詞
    PREFIX,
}
