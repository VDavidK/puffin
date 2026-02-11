use crate::run_parser_str;
use colored::Colorize;

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