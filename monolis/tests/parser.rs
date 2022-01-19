use monolis::lexer::Lexer;
use monolis::parser::Parser;

#[test]
fn test_parsed_tree_print_as_is() {
    let lexer = Lexer::new("(+ (+ 1 2) 3)".chars().collect());
    let tree = Parser::new(lexer).parse().unwrap();
    assert_eq!("(+ (+ 1 2) 3)", format!("{}", tree),);
    tree.iter().for_each(|f| println!("{:?} ", f));

    let lexer = Lexer::new("(+   (+ 1  2) 3)".chars().collect());
    let tree = Parser::new(lexer).parse().unwrap();
    assert_eq!("(+ (+ 1 2) 3)", format!("{}", tree),);
    tree.iter().for_each(|f| println!("{:?} ", f));
}

#[test]
fn test_parse_with_quote() {
    let lexer = Lexer::new("'1".chars().collect());
    // let lexer = Lexer::new("'1".chars().collect());
    let tree = Parser::new(lexer).parse().unwrap();
    assert_eq!("(quote 1)", format!("{}", tree),);
}
