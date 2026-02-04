use crate::run_parser_str;
use colored::Colorize;

#[test]
fn test_component_methods() {
    let foo = run_parser_str("
        component MyComponent(foo, bar, baz) {
            fn foo(one, two, three) {
            }

            @onclick(Lmb)
            fn bar(four, five, six) {
                one = two + three;
            }
        }
    ");
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}

#[test]
fn test_if() {
    let foo = run_parser_str("
        component MyComponent(foo, bar, baz) {
            fn foo(one, two, three) {
                if one == two {
                    print(\"One is two\");
                } else if two == three {
                    print(\"Two is three\");
                } else {
                    print(\"Oh well\");
                }
            }
        }
    ");
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}