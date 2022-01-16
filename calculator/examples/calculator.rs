use calculator::lexer::Lexer;
use calculator::parser::{eval, Parser};
use std::io;
use std::io::prelude::*;
fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .ok()
            .expect("failed to read line");

        if code == "exit\n" {
            break;
        }

        let lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(lexer);

        let expr = parser.parse();

        if let Some(expr) = expr {
            println!("{}", eval(&expr));
        }
    }
}
