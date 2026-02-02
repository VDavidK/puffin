use crate::run_parser_str;
use colored::Colorize;

#[test]
fn test_parse() {
    let foo = run_parser_str("
    if fun == true {\n
    }
    ");
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}