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

    STRING {
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
                if is_symboltoken(s.clone()) {
                    Self::SYMBOL { buf }
                } else {
                    let first = s.first();
                    let last = s.last();
                    if first.is_some()
                        && last.is_some()
                        && first.unwrap() == &'"'
                        && last.unwrap() == &'"'
                    {
                        Self::STRING {
                            buf: buf.trim_matches('"').into(),
                        }
                    } else {
                        Self::OTHER { buf }
                    }
                }
            }
        }
    }

    /// symbolとして許可されている記号を返す
    pub fn symbol_chars() -> Iter<'static, char> {
        static CHARS: [char; 9] = ['!', '?', '+', '-', '*', '/', '=', '<', '>'];
        CHARS.iter()
    }

    pub fn separate_chars() -> Iter<'static, char> {
        static CHARS: [char; 2] = ['(', ')'];
        CHARS.iter()
    }

    /// '(',  ')', '\'', '.' のいずれかの場合、それに対応したTokenを返す
    pub fn find(c: &char) -> Option<Token> {
        match c {
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            '\'' => Some(Token::QUOTE),
            '.' => Some(Token::DOT),
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
                !s.iter()
                    .any(|c| !(c.is_ascii_alphabetic() || c.is_ascii_digit() || is_symch(c)))
            }
        }
    }
}

fn is_symch(c: &char) -> bool {
    Token::symbol_chars().any(|sym| sym == c)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_token_new() {
        let mut tables = HashMap::new();

        let number_patterns = vec![-1, 0, 1, 2, 10, 100];
        number_patterns.iter().for_each(|p| {
            tables.insert(p.to_string(), Token::NUMBER { val: *p as f64 });
        });

        let symbol_patterns = vec!["a", "!", "+", "aa", "!!", "++", "a!", "a+", "!a", "+a"];
        symbol_patterns.iter().for_each(|p| {
            tables.insert(p.to_string(), Token::SYMBOL { buf: p.to_string() });
        });

        let other_patterns = vec!["\"\"hello, world\""];
        other_patterns.iter().for_each(|p| {
            tables.insert(
                p.to_string(),
                Token::STRING {
                    buf: "hello, world".into(),
                },
            );
        });

        for (key, val) in tables {
            let result = Token::new(key.chars().collect());
            assert_eq!(val, result, "case: {} failed", key);
        }
    }
}

pub trait Tokenize {
    fn token(&mut self) -> Option<Token>;
}
