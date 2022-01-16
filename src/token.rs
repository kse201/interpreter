use std::slice::Iter;

#[derive(Debug, PartialEq, Clone, Copy)]
/// 字句
///
/// [Ruby JIS](https://kikakurui.com/x3/X3017-2013-01.html)
/// 8.7.2 キーワード
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
    /// =
    Equal,

    /// _LINE̲̲
    /// :  将来の使用のため予約
    _Line,
    /// _ENCODING̲̲
    /// :  将来の使用のため予約
    _Encoding,
    /// _FILE̲̲
    /// :  将来の使用のため予約
    _File,
    // BEGIN
    /// :  将来の使用のため予約
    _Begin,
    /// END
    /// :  将来の使用のため予約
    _End,
    /// alias
    Alias,
    /// and
    An,
    /// begin
    Begin,
    /// break
    Break,
    /// case
    Case,
    /// class
    Class,
    /// def
    Def,
    /// defined?
    Defined,
    /// do
    Do,
    /// else
    Else,
    /// elsif
    Elsif,
    /// end
    End,
    /// ensure
    Ensure,
    /// for
    For,
    /// false
    False,
    /// if
    If,
    /// in
    In,
    /// module
    Module,
    /// next
    Next,
    /// nil
    Nil,
    /// not
    Not,
    /// or
    Or,
    /// redo
    Redo,
    /// rescue
    Rescue,
    /// retry
    Retry,
    /// return
    Return,
    /// self
    Selh,
    /// super
    Super,
    /// then
    Then,
    /// true
    True,
    /// undef
    Undef,
    /// unless
    Unless,
    /// until
    Until,
    /// when
    When,
    /// while
    While,
    /// yield
    Yield,
}

impl Token {
    fn iterator() -> Iter<'static, Token> {
        static TOKENS: [Token; 8] = [
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Equal,
            Token::LeftParen,
            Token::RightParen,
            Token::Def,
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
            Token::Equal => &"=",
            Token::Def => &"def",
            _ => unimplemented!("unimplemented keyword: {:?}", self),
        }
    }

    /// キーワードに対応する字句を返す
    ///
    /// # Examples
    /// ```
    /// # use rs_ruby::token::*;
    /// assert_eq!(Token::find("+".into()), Some(Token::Plus));
    /// assert_eq!(Token::find("1".into()), None);
    /// assert_eq!(Token::find("undefinekeyword".into()), None);
    /// ```
    pub fn find(s: String) -> Option<Token> {
        Self::iterator()
            .find(|token| token.keyword() == s)
            .and_then(|s| Some(*s))
    }
}
