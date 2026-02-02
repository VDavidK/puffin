use crate::run_parser_str;
use colored::Colorize;

#[test]
fn test_parse() {
    let foo = run_parser_str("
        let test = \"Agreeable Grunt\";
        const foo = 15;

        component MyComponent(foo, bar, baz) {
            let my_let = 10;

            fn foo(one, two, three) {
            }

            @onclick(Lmb)
            fn bar(four, five, six) {
            }
        }
    ");
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}