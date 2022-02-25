use crate::token::{lookup_ident, Token, TokenType};
pub struct Lexer {
    input: Vec<char>,
    // current potision
    position: usize,
    // next potision
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.chars().collect(),
            position: usize::default(),
            read_position: usize::default(),
            ch: char::default(),
        };
        l.read_char();
        l
    }
    pub fn next_token(&mut self) -> Token {
        use TokenType::*;
        self.skip_whitespace();
        let (token_type, literal) = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    (EQ, "==".into())
                } else {
                    (ASSIGN, self.ch.to_string())
                }
            }
            '(' => (LPAREN, self.ch.to_string()),
            ')' => (RPAREN, self.ch.to_string()),
            ',' => (COMMA, self.ch.to_string()),
            '+' => (PLUS, self.ch.to_string()),
            '-' => (MINUS, self.ch.to_string()),
            '/' => (SLASH, self.ch.to_string()),
            '*' => (ASTERLISK, self.ch.to_string()),
            '{' => (LBRACE, self.ch.to_string()),
            '}' => (RBRACE, self.ch.to_string()),
            ';' => (SEMICOLON, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    (NotEq, "!=".into())
                } else {
                    (BANG, self.ch.to_string())
                }
            }
            '>' => (GT, self.ch.to_string()),
            '<' => (LT, self.ch.to_string()),
            _ => {
                let default = char::default();
                if self.ch == default {
                    (EOF, self.ch.to_string())
                } else if is_letter(self.ch) {
                    let letter = self.read_identifier();
                    let ttype = lookup_ident(letter.clone());
                    return Token::new(ttype, letter);
                } else if is_digit(self.ch) {
                    let letter = self.read_number();
                    return Token::new(INT, letter);
                } else {
                    (ILLIGAL, self.ch.to_string())
                }
            }
        };
        self.read_char();
        Token::new(token_type, literal)
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::default();
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            char::default()
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
    fn read_identifier(&mut self) -> String {
        let mut ident = Vec::new();
        while is_letter(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        String::from_iter(ident)
    }
    fn read_number(&mut self) -> String {
        let mut ident = Vec::new();
        while is_digit(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        String::from_iter(ident)
    }

    pub fn iter(&mut self) -> std::vec::IntoIter<Token> {
        let mut tokens = Vec::new();
        let mut tok = self.next_token();
        while tok.token_type != TokenType::EOF {
            tokens.push(tok);
            tok = self.next_token();
        }
        tokens.into_iter()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{self};

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/+5;
        5 < 10 > 5;
        if ( 5 < 10) {
            return 10;
        } else {
            return 5;
        }

        10 == 10;
        10 != 9;

        "#;

        use token::TokenType::*;
        let mut tests = Vec::new();
        // let five = 5;
        tests.push((LET, "let"));
        tests.push((IDENT, "five"));
        tests.push((ASSIGN, "="));
        tests.push((INT, "5"));
        tests.push((SEMICOLON, ";"));

        // let ten = 10;
        tests.push((LET, "let"));
        tests.push((IDENT, "ten"));
        tests.push((ASSIGN, "="));
        tests.push((INT, "10"));
        tests.push((SEMICOLON, ";"));

        // let add = fn(x, y) { x + y; };
        tests.push((LET, "let"));
        tests.push((IDENT, "add"));
        tests.push((ASSIGN, "="));
        tests.push((FUNCTION, "fn"));
        tests.push((LPAREN, "("));
        tests.push((IDENT, "x"));
        tests.push((COMMA, ","));
        tests.push((IDENT, "y"));
        tests.push((RPAREN, ")"));
        tests.push((LBRACE, "{"));
        tests.push((IDENT, "x"));
        tests.push((PLUS, "+"));
        tests.push((IDENT, "y"));
        tests.push((SEMICOLON, ";"));
        tests.push((RBRACE, "}"));
        tests.push((SEMICOLON, ";"));

        // let result = add(five, ten));
        tests.push((LET, "let"));
        tests.push((IDENT, "result"));
        tests.push((ASSIGN, "="));
        tests.push((IDENT, "add"));
        tests.push((LPAREN, "("));
        tests.push((IDENT, "five"));
        tests.push((COMMA, ","));
        tests.push((IDENT, "ten"));
        tests.push((RPAREN, ")"));
        tests.push((SEMICOLON, ";"));

        // !-/+5;
        tests.push((BANG, "!"));
        tests.push((MINUS, "-"));
        tests.push((SLASH, "/"));
        tests.push((PLUS, "+"));
        tests.push((INT, "5"));
        tests.push((SEMICOLON, ";"));
        // 5 < 10 > 5;
        tests.push((INT, "5"));
        tests.push((LT, "<"));
        tests.push((INT, "10"));
        tests.push((GT, ">"));
        tests.push((INT, "5"));
        tests.push((SEMICOLON, ";"));

        // if ( 5 < 10) { return 10; }
        tests.push((IF, "if"));
        tests.push((LPAREN, "("));
        tests.push((INT, "5"));
        tests.push((LT, "<"));
        tests.push((INT, "10"));
        tests.push((RPAREN, ")"));
        tests.push((LBRACE, "{"));
        tests.push((RETURN, "return"));
        tests.push((INT, "10"));
        tests.push((SEMICOLON, ";"));
        tests.push((RBRACE, "}"));

        // else { return 5; }
        tests.push((ELSE, "else"));
        tests.push((LBRACE, "{"));
        tests.push((RETURN, "return"));
        tests.push((INT, "5"));
        tests.push((SEMICOLON, ";"));
        tests.push((RBRACE, "}"));

        // 10 == 10;
        tests.push((INT, "10"));
        tests.push((EQ, "=="));
        tests.push((INT, "10"));
        tests.push((SEMICOLON, ";"));
        // 10 != 9;
        tests.push((INT, "10"));
        tests.push((NotEq, "!="));
        tests.push((INT, "9"));
        tests.push((SEMICOLON, ";"));

        tests.push((EOF, "\u{0}"));

        let mut l: Lexer = Lexer::new(input);
        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok: token::Token = l.next_token();

            assert_eq!(
                *expected_type, tok.token_type,
                "test[{}] wrong token type",
                i
            );
            assert_eq!(*expected_literal, tok.literal, "test[{}] wrong literal", i);
        }
    }
}
