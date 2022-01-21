use std::rc::Rc;

use monolis::{
    parser::{Cell, Env, Sexp},
    *,
};

#[test]
fn test_eval_plus() {
    assert_eval(num!(6), "(+ (+ 1 2) 3)");
    assert_eval(num!(6), "(+ 1 2 3)");
}

#[test]
fn test_eval_minus() {
    assert_eval(num!(0), "(- (- 2 1) 3)");
    assert_eval(num!(-6), "(- 3 2 1)");
}

#[test]
fn test_eval_multi() {
    assert_eval(num!(24), "(* (* 2 3) 4)");
}

#[test]
fn test_eval_div() {
    assert_eval(num!(1), "(/ (/ 8 4) 2)")
}

#[test]
fn test_eval_equal() {
    assert_eval(nil!(), "(= 2 1)");
    assert_eval(num!(2), "(= 2 2)");
}

#[test]
fn test_eval_eq() {
    assert_eval(nil!(), "(eq 'a 'b)");
    assert_eval(sym!("a"), "(eq 'a 'a)");
}

#[test]
fn test_eval_if() {
    assert_eval(num!(1), "(if (= 1 1) 1 2)");
    assert_eval(num!(2), "(if (= 2 1) 1 2)");
}

#[test]
fn test_eval_fizzbuzz() {
    assert_eval(
        num!(1),
        r#"
(defun fizzbuzz (x)
  (if (and
        (= 0 (mod x 3))
        (= 0 (mod x 5)))
    (print "fizzbuzz")
    (if (= 0 (mod x 5))
      (print "buzz")
      (if (= 0 (mod x 3))
        (print "fizz")
        (print x)))))
"#,
    );
}

fn assert_eval(expect: Sexp, input: &str) {
    let input = String::from(input);
    let lexer = Lexer::new(input.chars().collect());
    let tree = Parser::new(lexer).parse().unwrap();

    let genv = Env::default();
    let lenv = Default::default();
    monolis::subr::initsubr(Rc::clone(&genv));
    monolis::fsubr::initfsubr(Rc::clone(&genv));

    assert_eq!(expect, eval(tree, genv, lenv).unwrap());
}
