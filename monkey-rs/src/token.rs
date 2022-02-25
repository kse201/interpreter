#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLIGAL,
    EOF,
    // 識別子
    IDENT,
    // 整数
    INT,

    // 演算子
    PLUS,
    MINUS,
    ASTERLISK,
    SLASH,
    // !
    BANG,

    // <
    LT,
    // >
    GT,
    // ==
    EQ,
    // !=
    NotEq,

    // デリミタ
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // fn
    FUNCTION,
    LET,
    RETURN,
    ASSIGN,

    // true
    TRUE,
    // false
    FALSE,
    // if
    IF,
    // else
    ELSE,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

pub fn lookup_ident(ident: String) -> TokenType {
    match ident.as_str() {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "True" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "return" => TokenType::RETURN,
        _ => TokenType::IDENT,
    }
}
