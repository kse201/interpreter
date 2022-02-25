static PROMPT: &str = ">> ";

use crate::lexer::Lexer;
use std::io;
use std::io::prelude::*;

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .ok()
            .expect("failed to read line");

        if code == "exit\n" {
            break;
        }

        let mut lexer = Lexer::new(code.as_str());
        for tok in lexer.iter() {
            println!("{:?}", tok);
        }
    }
}
