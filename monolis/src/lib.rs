pub mod lexer;
pub mod parser;
pub mod token;

pub type Lexer = lexer::Lexer;
pub type Parser = parser::Parser<Lexer>;

use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
pub type Env = Rc<RefCell<HashMap<String, Sexp>>>;

use parser::{Cell, Sexp};

pub fn eval(sexp: Sexp, genv: Env, lenv: Env) -> Sexp {
    match sexp.as_ref() {
        Cell::NUMBER { .. } => sexp.clone(),
        Cell::SYMBOL { name } => {
            todo!()
            // let cell = find_sym(name);
            // match ref {
            // Some(e) => { return cell ;}
            // None=> { panic!("eval error")}
            // }
        }
        Cell::CONS { .. } => {
            if sexp.car().is_symbol() && (Some("quote".to_string()) == sexp.car().name()) {
                sexp.cadr()
            } else if sexp.car().is_number() {
                panic!("Arg Error")
            } else if is_subrp(*sexp.car(), Rc::clone(&genv)) {
                apply(
                    &sexp.car(),
                    evlis(&sexp.cdr(), Rc::clone(&genv)),
                    Rc::clone(&genv),
                )
            } else if sexp.car().is_fsubr() {
                unimplemented!()
            } else if sexp.car().is_function() {
                unimplemented!()
            } else {
                unreachable!()
            }
        }
        _ => unimplemented!(),
    }
}

fn evlis(sexp: &Sexp, env: Env) -> Sexp {
    if sexp.is_nil() {
        Cell::nil()
    } else {
        Cell::cons(
            eval(sexp.car(), Rc::clone(&env), Default::default()),
            evlis(&sexp.cdr(), env),
        )
    }
}

fn apply(func: &Sexp, args: Sexp, env: Env) -> Sexp {
    let sym = find_sym(func.name().unwrap(), env);
    if sym.is_nil() {
        panic!()
    } else {
        match sym.as_ref() {
            Cell::SUBR { subr } => subr(args),
            Cell::FSUBR => {
                unimplemented!()
            }
            Cell::FUNC => {
                unimplemented!()
            }
            _ => unreachable!(),
        }
    }
}

fn is_subrp(sym: Cell, env: Env) -> bool {
    sym.name()
        .map(|name| find_sym(name, env).is_subr())
        .is_some()
}

fn f_plus(args: Sexp) -> Sexp {
    let mut res = 0.0;
    let mut curr = args;
    while curr.is_some() {
        let car = curr.car();
        let arg = match *car {
            Cell::NUMBER { val } => val,
            _ => panic!("{:?}", *car),
        };
        curr = curr.cdr();
        res += arg;
    }
    Cell::number(res)
}

fn find_sym(name: String, env: Env) -> Sexp {
    match env.borrow().get(&name) {
        Some(sexp) => sexp.clone(),
        None => Cell::nil(),
    }
}

fn bind_sym(name: String, val: Sexp, env: Env) {
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

#[cfg(test)]
mod tests {

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
}
