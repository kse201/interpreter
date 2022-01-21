use super::bind_sym;
use crate::parser::Env;
use crate::parser::{Cell, Sexp};
use crate::{nil, num};
use anyhow::{anyhow, Result};
use log::*;
use std::rc::Rc;

#[macro_export]
macro_rules! initsubr {
    ($keyword:expr, $subr:expr, $env:expr) => {
        bind_sym(
            $keyword.into(),
            Cell::subr($keyword.into(), $subr),
            Rc::clone(&$env),
        );
    };
}

pub fn initsubr(env: Env) {
    initsubr!("+", f_plus, env);
    initsubr!("-", f_minus, env);
    initsubr!("*", f_mult, env);
    initsubr!("/", f_div, env);
    initsubr!("mod", f_mod, env);
    initsubr!("and", f_and, env);
    initsubr!("or", f_or, env);
    initsubr!("=", f_eq, env);
    initsubr!("eq", f_eq, env);
    initsubr!("print", f_print, env);
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
    Ok(num!(res))
}

fn f_minus(args: Sexp) -> Result<Sexp> {
    let mut res = 0.0;
    let mut curr = args;
    while curr.is_value() {
        let car = curr.car();
        let arg = match *car {
            Cell::NUMBER { val } => val,
            _ => return Err(anyhow!(format!("unexpected {}", car))),
        };
        curr = curr.cdr();
        res -= arg;
    }
    Ok(num!(res))
}

fn f_mult(args: Sexp) -> Result<Sexp> {
    let mut res = 1.0;
    let mut curr = args;
    while curr.is_value() {
        let car = curr.car();
        let arg = match *car {
            Cell::NUMBER { val } => val,
            _ => return Err(anyhow!(format!("unexpected {}", car))),
        };
        curr = curr.cdr();
        res *= arg;
    }
    Ok(num!(res))
}

fn f_div(args: Sexp) -> Result<Sexp> {
    let mut res = match *args.car() {
        Cell::NUMBER { val } => val,
        _ => return Err(anyhow!(format!("unexpected {}", *args.car()))),
    };
    let mut arglist = args.cdr();
    while arglist.is_value() {
        let arg = match *arglist.car() {
            Cell::NUMBER { val } => val,
            _ => return Err(anyhow!(format!("unexpected {}", args.car()))),
        };
        arglist = arglist.cdr();
        res /= arg;
    }
    Ok(num!(res))
}

fn f_mod(args: Sexp) -> Result<Sexp> {
    let arg1 = args.car();
    let arg2 = args.cadr();

    if let Some(num1) = arg1.val() {
        if let Some(num2) = arg2.val() {
            return Ok(num!(num1 % num2));
        }
    }
    Ok(nil!())
}

fn f_and(args: Sexp) -> Result<Sexp> {
    let mut arglist = args;
    while arglist.cdr().is_value() {
        debug!("f_and arglist: {:?}", arglist);
        let num1 = arglist.car();
        let num2 = arglist.cadr();
        if num1 != num2 {
            return Ok(nil!());
        }
        arglist = arglist.cdr();
    }
    Ok(Cell::symbol("t".into()))
}

fn f_or(args: Sexp) -> Result<Sexp> {
    let mut arglist = args;
    while arglist.cdr().is_value() {
        debug!("f_or arglist: {:?}", arglist);
        let num1 = arglist.car();
        let num2 = arglist.cadr();
        if num1 == num2 {
            return Ok(Cell::symbol("t".into()));
        }
        arglist = arglist.cdr();
    }
    Ok(nil!())
}

fn f_eq(args: Sexp) -> Result<Sexp> {
    let num1 = args.car();
    let num2 = args.cadr();

    if num1 == num2 {
        Ok(Cell::symbol("t".into()))
    } else {
        Ok(nil!())
    }
}

fn f_print(args: Sexp) -> Result<Sexp> {
    let mut curr = args;
    while curr.is_value() {
        let car = curr.car();
        print!("{} ", car);
        curr = curr.cdr();
    }
    println!("");
    Ok(nil!())
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_f_plus_playground() {
        let lexer = Lexer::new("(+ 1 2 3 )".into());
        let mut parser = Parser::new(lexer);

        let tree = parser.parse().unwrap();
        println!("{:?}", tree);
    }
}
