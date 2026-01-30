use crate::run_parser_str;

#[test]
fn test_parse() {
    let foo = run_parser_str("
    if fun == true {\n
        println(\"Hello, world!\");\n
    }
    ");
    assert!(foo.is_ok())
}