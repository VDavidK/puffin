use crate::run_parser_str;

#[test]
fn test_parse() {
    let foo = run_parser_str("-++++");
    assert!(foo.is_ok())
}