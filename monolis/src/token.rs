use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// '('
    LPAREN,
    /// ')'
    RPAREN,
    /// \'
    QUOTE,
    /// .
    DOT,

    NUMBER {
        val: f64,
    },

    SYMBOL {
        buf: String,
    },

    OTHER {
        buf: String,
    },
}

impl Token {
    pub fn new(s: Vec<char>) -> Self {
        let buf = s.iter().collect::<String>();
        match buf.parse::<f64>() {
            Ok(n) => Self::NUMBER { val: n },
            Err(_) => {
                if is_symboltoken(s) {
                    Self::SYMBOL { buf }
                } else {
                    Self::OTHER { buf }
                }
            }
        }
    }

    /// symbolとして許可されている記号を返す
    pub fn symbol_chars() -> Iter<'static, char> {
        static CHARS: [char; 9] = ['!', '?', '+', '-', '*', '/', '=', '<', '>'];
        CHARS.iter()
    }

    pub fn separete_chars() -> Iter<'static, char> {
        static CHARS: [char; 2] = ['(', ')'];
        CHARS.iter()
    }

    /// '(',  ')', '\'', '.' のいずれかの場合、それに対応したTokenを返す
    pub fn find(c: &char) -> Option<Token> {
        match c {
            &'(' => Some(Token::LPAREN),
            &')' => Some(Token::RPAREN),
            &'\'' => Some(Token::QUOTE),
            &'.' => Some(Token::DOT),
            _ => None,
        }
    }
}

fn is_symboltoken(s: Vec<char>) -> bool {
    match s.first() {
        None => false,
        Some(c) => {
            if c.is_ascii_digit() {
                false
            } else {
                s.iter()
                    .find(|c| !(c.is_ascii_alphabetic() || c.is_ascii_digit() || is_symch(c)))
                    .is_none()
            }
        }
    }
}

fn is_symch(c: &char) -> bool {
    Token::symbol_chars().find(|sym| *sym == c).is_some()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_token_new_with_num() {
        let mut tables = HashMap::new();
        tables.insert("1", Token::NUMBER { val: 1.0 });
        tables.insert("10", Token::NUMBER { val: 10.0 });
        tables.insert("2", Token::NUMBER { val: 2.0 });

        tables.insert("-1", Token::NUMBER { val: -1.0 });
        for (key, val) in tables {
            let result = Token::new(key.chars().collect());
            assert_eq!(result, val, "case: {} failed", key);
        }
    }

    #[test]
    fn test_token_new_with_symbol() {
        let mut tables = HashMap::new();
        tables.insert("a", Token::SYMBOL { buf: "a".into() });

        tables.insert("a-1", Token::SYMBOL { buf: "a-1".into() });

        tables.insert("a!", Token::SYMBOL { buf: "a!".into() });

        tables.insert("!a", Token::SYMBOL { buf: "!a".into() });
        for (key, val) in tables {
            let result = Token::new(key.chars().collect());
            assert_eq!(result, val, "case: {} failed", key);
        }
    }

    #[test]
    fn test_is_symbol_token() {
        let mut tables = HashMap::new();
        tables.insert("a", true);
        tables.insert("a-1", true);
        tables.insert("a-b", true);
        for (key, val) in tables {
            let result = is_symboltoken(key.chars().collect());
            assert_eq!(result, val, "case: {} failed", key);
        }
    }

    #[test]
    fn test_is_other_token() {
        let mut tables = HashMap::new();
        tables.insert("\"a\"", false);
        tables.insert("\"Hello, World\"", false);
        for (key, val) in tables {
            let result = is_symboltoken(key.chars().collect());
            assert_eq!(result, val, "case: {} failed", key);
        }
    }
}
