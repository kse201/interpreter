pub mod lexer;
pub mod parser;
pub mod token;

pub type Lexer = lexer::Lexer;
pub type Parser = parser::Parser<Lexer>;
use log::*;

use anyhow::anyhow;
use anyhow::Result;
use std::rc::Rc;
pub type Env = parser::Env;

use parser::{Cell, Sexp};

pub fn eval(sexp: Sexp, genv: Env, lenv: Env) -> Result<Sexp> {
    debug!("eval {:?}", sexp);
    match sexp.as_ref() {
        Cell::NUMBER { .. } => Ok(sexp.clone()),
        Cell::SYMBOL { name } => {
            let sym = find_sym(name.to_string(), genv);
            if sym.is_value() {
                Ok(sym)
            } else {
                debug!("No symbol {:?}", sexp.as_ref());
                Err(anyhow!("No symbol {:?}", sexp.as_ref()))
            }
        }
        Cell::CONS { .. } => {
            if sexp.car().is_symbol() && (Some("quote".to_string()) == sexp.car().name()) {
                Ok(sexp.cadr())
            } else if sexp.car().is_number() {
                panic!("Arg Error")
            } else if is_subrp(*sexp.car(), Rc::clone(&genv)) {
                apply(
                    &sexp.car(),
                    evlis(&sexp.cdr(), Rc::clone(&genv))?,
                    Rc::clone(&genv),
                )
            } else if is_fsubrp(*sexp.car(), Rc::clone(&genv)) {
                apply(&sexp.car(), sexp.cdr(), Rc::clone(&genv))
            } else if sexp.car().is_function() {
                unimplemented!()
            } else {
                unreachable!()
            }
        }
        _ => unimplemented!(),
    }
}

fn evlis(sexp: &Sexp, env: Env) -> Result<Sexp> {
    debug!("evlis(sexp: {:?})", sexp);
    if sexp.is_nil() {
        Ok(Cell::nil())
    } else {
        Ok(Cell::cons(
            eval(sexp.car(), Rc::clone(&env), Default::default())?,
            evlis(&sexp.cdr(), env)?,
        ))
    }
}

fn apply(func: &Sexp, args: Sexp, env: Env) -> Result<Sexp> {
    debug!("apply: func: {:?}, args: {:?}", func, args);
    let sym = find_sym(func.name().unwrap(), Rc::clone(&env));
    if sym.is_nil() {
        panic!()
    } else {
        match sym.as_ref() {
            Cell::SUBR { subr } => subr(args),
            Cell::FSUBR { fsubr } => fsubr(args, env),
            Cell::FUNC => {
                unimplemented!()
            }
            _ => unreachable!(),
        }
    }
}

fn is_subrp(sym: Cell, env: Env) -> bool {
    match sym.name() {
        Some(name) => find_sym(name, env).is_subr(),
        None => false,
    }
}

fn is_fsubrp(sym: Cell, env: Env) -> bool {
    match sym.name() {
        Some(name) => find_sym(name, env).is_fsubr(),
        None => false,
    }
}

fn f_plus(args: Sexp) -> Result<Sexp> {
    let mut res = 0.0;
    let mut curr = args;
    while curr.is_value() {
        let car = curr.car();
        let arg = match *car {
            Cell::NUMBER { val } => val,
            _ => return Err(anyhow!(format!("unexpected {}", car))),
        };
        curr = curr.cdr();
        res += arg;
    }
    Ok(Cell::number(res))
}

fn f_setq(args: Sexp, env: Env) -> Result<Sexp> {
    println!("f_setq: args: {:?}", args);
    let args1 = args.car();
    println!("args1: {:?}", args1);
    let val = eval(args.cadr(), Rc::clone(&env), Env::default())?;
    println!("val: {:?}", val);
    if let Cell::SYMBOL { name } = *args1 {
        bind_sym(name, val.clone(), env);
    }
    Ok(val)
}

fn find_sym(name: String, env: Env) -> Sexp {
    match env.borrow().get(&name) {
        Some(sexp) => sexp.clone(),
        None => Cell::nil(),
    }
}

fn bind_sym(name: String, val: Sexp, env: Env) {
    debug!("bind_sym:");
    env.borrow_mut().insert(name.into(), val);
}

fn assocsym(sym: Sexp, val: Sexp, list: Sexp) -> Sexp {
    Cell::cons(Cell::cons(sym, val), list)
}

fn assoc(sym: Sexp, list: Sexp) -> Sexp {
    match list.as_ref() {
        Cell::NIL => list,
        _ => {
            if list.caar() == sym {
                list.car()
            } else {
                assoc(sym, list.cdr())
            }
        }
    }
}

pub fn initsubr(env: Env) {
    bind_sym("+".into(), Cell::subr(f_plus), env);
}

pub fn initfsubr(env: Env) {
    bind_sym("setq".into(), Cell::fsubr(f_setq), env);
}

#[cfg(test)]
mod tests {

    use crate::parser::Parser;

    use super::*;

    // #[test]
    // fn test_eval_plus() {
    // let env = bind_sym(Cell::symbol("+".into()), Cell::subr(f_plus), Cell::nil());

    // let lexer = Lexer::new("(+ 1 2)".chars().collect());
    // let tree = Parser::new(lexer).parse();
    // assert_eq!(Cell::number(3.0), eval(tree, env.clone(), Cell::nil()));

    // let lexer = Lexer::new("(+ 1 (+ 2 3))".chars().collect());
    // let tree = Parser::new(lexer).parse();
    // assert_eq!(Cell::number(6.0), eval(tree, env, Cell::nil()));
    // }
    #[test]
    fn test_f_plus_playground() {
        let lexer = Lexer::new("(+ 1 2 3 )".into());
        let mut parser = Parser::new(lexer);

        let tree = parser.parse().unwrap();
        println!("{:?}", tree);
    }

    #[test]
    fn test_is_subr() {
        let env = Env::default();
        initfsubr(Rc::clone(&env));
        initsubr(Rc::clone(&env));

        let setq = Cell::symbol("setq".into());

        assert_eq!(false, is_subrp(*setq.clone(), Rc::clone(&env)));
        assert_eq!(true, is_fsubrp(*setq, env));
    }
}
