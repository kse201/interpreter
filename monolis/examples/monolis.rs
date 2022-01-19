use monolis::{eval, initsubr, Env, Lexer, Parser};
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
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
        let genv = Env::default();
        let lenv = Default::default();
        initsubr(Rc::clone(&genv));

        println!("{}", eval(expr, genv, lenv));
    }
}
