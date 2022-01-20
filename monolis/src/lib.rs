pub mod fsubr;
pub mod lexer;
pub mod parser;
pub mod subr;
pub mod token;

pub type Lexer = lexer::Lexer;
pub type Parser = parser::Parser<Lexer>;
use log::*;

use anyhow::anyhow;
use anyhow::Result;
use std::rc::Rc;
pub type Env = parser::Env;

use parser::{Cell, Sexp};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("No Symbol {0}")]
    Nosymbol(Cell),

    #[error("Unexpected Token {0}")]
    UnexpecteToken(Cell),
}

pub fn eval(sexp: Sexp, genv: Env, lenv: Env) -> Result<Sexp> {
    debug!("eval {:?}", sexp);
    match sexp.as_ref() {
        Cell::NUMBER { .. } => Ok(sexp.clone()),
        Cell::SYMBOL { name } => {
            let sym = find_sym(name.to_string(), genv);
            if sym.is_value() {
                Ok(sym)
            } else {
                Err(ParseError::Nosymbol(*sexp))?
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
                debug!("No symbol {:?}", sexp.car().as_ref());
                Err(anyhow!("No symbol {:?}", sexp.car().as_ref()))
            }
        }
        _ => Ok(sexp),
    }
}

fn evlis(sexp: &Sexp, env: Env) -> Result<Sexp> {
    debug!("evlis(sexp: {:?})", sexp);
    if sexp.is_nil() {
        Ok(nil!())
    } else {
        Ok(cons!(
            eval(sexp.car(), Rc::clone(&env), Default::default())?,
            evlis(&sexp.cdr(), env)?
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
            Cell::SUBR { subr, .. } => subr(args),
            Cell::FSUBR { fsubr, .. } => fsubr(args, env),
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

fn find_sym(name: String, env: Env) -> Sexp {
    match env.borrow().get(&name) {
        Some(sexp) => sexp.clone(),
        None => nil!(),
    }
}

fn bind_sym(name: String, val: Sexp, env: Env) {
    debug!("bind_sym:");
    env.borrow_mut().insert(name, val);
}

fn assocsym(sym: Sexp, val: Sexp, list: Sexp) -> Sexp {
    cons!(cons!(sym, val), list)
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
