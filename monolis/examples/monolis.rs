use monolis::*;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
fn main() {
    env_logger::init();
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
        let genv = Env::default();
        let lenv = Default::default();
        initsubr(Rc::clone(&genv));
        initfsubr(Rc::clone(&genv));

        let lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(lexer);

        let expr = parser.parse();
        match expr {
            Ok(tree) => println!("{}", eval(tree, genv, lenv)),
            Err(e) => println!("{:?}", e),
        };
    }
}
