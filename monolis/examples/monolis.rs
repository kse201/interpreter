use monolis::*;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
fn main() {
    env_logger::init();
    let genv = Env::default();
    let lenv = Default::default();
    monolis::subr::initsubr(Rc::clone(&genv));
    monolis::fsubr::initfsubr(Rc::clone(&genv));

    let mut count = 0;
    loop {
        count += 1;
        print!("monolis(main):{:>03}> ", count);
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
        match expr {
            Ok(tree) => match eval(tree, Rc::clone(&genv), Rc::clone(&lenv)) {
                Ok(result) => println!("=> {}", result),
                Err(e) => println!("eval error: {}", e),
            },
            Err(e) => println!("parse error: {}", e),
        }
    }
}
