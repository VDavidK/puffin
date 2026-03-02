use std::error::Error;
use crate::run_parser_str;
use colored::Colorize;
use puffin_ast::declaration::{Declaration};
use puffin_ast::{VarType};
use puffin_ast::expression::{Expression};
use puffin_ast::statement;

#[test]
fn test_component_methods() {
    let foo = run_parser_str(r#"
        component MyComponent(foo, bar, baz) {
            fn foo(one, two, three) {
            }

            @onclick(Lmb)
            fn bar(four, five, six) {
                one = two + three;
            }
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}

#[test]
fn test_if() {
    let foo = run_parser_str(r#"
        component MyComponent(foo, bar, baz) {
            fn foo(one, two, three) {
                if one == two {
                    print("One is two");
                } else if two == three {
                    print("Two is three");
                } else {
                    print("Oh well");
                }
            }
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}

#[test]
fn test_layout() {
    let foo = run_parser_str(r#"
        component MyComponent(foo, bar, baz) {
            layout {
                vbox {
                    text "hello, world!";
                    hbox {
                        button onclick=foo "click me!";
                        button onclick=[foo, bar] "also me!";
                        button onclick(ev)={foo()} "and me!";
                        button onclick(ev)={foo(); bar()} "not me!";
                        button onclick={foo(); bar()} "perhaps me...";
                        button onclick={exit(1)(2)(3)} "quit";
                    }
                }
            }
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}

#[test]
fn test_for() {
    let foo = run_parser_str(r#"
        fn func() {
            for i in foo {
                for j in 0:10 {
                    print("Hello, world!");
                }
            }
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}
#[test]
fn test_return() {
    let foo = run_parser_str(r#"
        fn func() {
            return 1 > 2;
        }
        fn other() {
            return;
        }
        fn other_other() {
            return 1;
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}

#[test]
fn test_comments() {
    let foo = run_parser_str(r#"
        /* This method does stuff */
        fn foo() {
            // TODO: Make this do stuff.
            return;
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok())
}

#[test]
fn test_accessor() {
    let foo = run_parser_str(r#"
        fn foo() {
            foo = bar.baz.qux;
            thing = func().field.other().field;
            x[1] = 2;
        }
    "#);
    if let Err(e) = &foo {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(foo.is_ok());
}

#[test]
fn test_dictionary() {
    let input = r#"
        let dict = {
            foo: "One",
            bar: "Two",
            baz: "Three",
        };
    "#;
    let result = run_parser_str(input);
    if let Err(e) = &result {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(result.is_ok());
    let res = result.unwrap();
    let iter = res.declarations;
    assert_eq!(iter.len(), 1);

    match iter.first().unwrap() {
        Declaration::Var(inner) => {
            assert_eq!(inner.name.lexeme, "dict");
            assert_eq!(inner.var_type, VarType::Let);
            match &*inner.value {
                Expression::Dictionary(expr) => {
                    let entries = &expr.entries;
                    assert_eq!(entries.len(), 3);
                    assert_eq!(entries[0].0.lexeme, "foo");
                    assert_eq!(entries[1].0.lexeme, "bar");
                    assert_eq!(entries[2].0.lexeme, "baz");
                    let mut exprs = vec![];
                    for (_, expr) in entries {
                        if let Expression::Literal(e) = expr {
                            exprs.push(e);
                        } else {
                            assert!(false, "Mismatched types (expected LiteralExpression)")
                        }
                    }
                    assert_eq!(exprs[0].token.lexeme, "\"One\"");
                    assert_eq!(exprs[1].token.lexeme, "\"Two\"");
                    assert_eq!(exprs[2].token.lexeme, "\"Three\"");
                },
                _ => assert!(false, "Mismatched types (expected DictionaryExpression)"),
            }
        },
        _ => assert!(false, "Mismatched types (expected VarDeclaration)")
    }
}