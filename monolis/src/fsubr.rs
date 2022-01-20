use super::bind_sym;
use super::eval;
use crate::parser::Env;
use crate::parser::{Cell, Sexp};
use anyhow::Result;
use std::rc::Rc;

#[macro_export]
macro_rules! initfsubr {
    ($keyword:expr, $fsubr:expr, $env:expr) => {
        bind_sym(
            $keyword.into(),
            Cell::fsubr($keyword.into(), $fsubr),
            Rc::clone(&$env),
        );
    };
}
pub fn initfsubr(env: Env) {
    initfsubr!("setq", f_setq, env);
    initfsubr!("if", f_if, env);
    initfsubr!("defun", f_defun, env);
}

fn f_setq(args: Sexp, env: Env) -> Result<Sexp> {
    let args1 = args.car();
    let val = eval(args.cadr(), Rc::clone(&env), Env::default())?;
    if let Cell::SYMBOL { name } = *args1 {
        bind_sym(name, val.clone(), env);
    }
    Ok(val)
}

fn f_if(args: Sexp, env: Env) -> Result<Sexp> {
    let arg1 = args.car();
    let arg2 = args.cadr();
    let arg3 = args.cdr().cdr().car();

    if eval(arg1, Rc::clone(&env), Env::default())?.is_value() {
        eval(arg2, Rc::clone(&env), Env::default())
    } else {
        eval(arg3, Rc::clone(&env), Env::default())
    }
}

fn f_defun(args: Sexp, env: Env) -> Result<Sexp> {
    let args1 = args.car();
    let args2 = args.cdr();
    if let Cell::SYMBOL { name } = *args1.clone() {
        bind_sym(name, args2, env);
    }
    Ok(args1)
}
