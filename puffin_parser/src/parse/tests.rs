use std::error::Error;
use crate::run_parser_str;
use colored::Colorize;
use puffin_ast::{VarType};
use puffin_ast::declaration::{ComponentDeclaration, Declaration, MethodDeclaration, VarDeclaration};
use puffin_ast::expression::{BinaryExpression, Expression, LiteralExpression};
use puffin_ast::statement::{AssignStatement, BlockStatement, ExpressionStatement, ReturnStatement, Statement, VariableDeclarationStatement};
use puffin_ast::token::TokenType;
use crate::parse::{ParserError, PuffinParser, Token};
use crate::parse::tests::TestError::{InvalidMethodDeclaration, UnexpectedCastTypeError};

#[derive(thiserror::Error, Debug)]
enum TestError {
    #[error(transparent)]
    ParserError(#[from] ParserError),
    #[error("Unexpected type received. Expected {0}")]
    UnexpectedCastTypeError(String),
    #[error("Invalid method declaration")]
    InvalidMethodDeclaration,
}

fn validate_method_decl(decl: &MethodDeclaration, expected_name: impl AsRef<str>, parameters: Vec<String>) -> Result<&BlockStatement, TestError> {
    assert_eq!(decl.name.lexeme, expected_name.as_ref());
    assert_eq!(decl.parameters.iter().map(|p| p.lexeme.to_owned()).collect::<Vec<String>>(), parameters);
    if let Statement::Block(b) = &*decl.block {
        Ok(b)
    } else {
        assert!(false, "Mismatched types. Expected BlockStatement");
        Err(InvalidMethodDeclaration)
    }
}

fn expect<T, U>(obj: U) -> Result<T, TestError> where U : TryInto<T, Error = ()> {
    let received_type_name = std::any::type_name_of_val(&obj).to_string();
    obj.try_into().map_err(|_| UnexpectedCastTypeError(std::any::type_name::<T>().to_string()))
}

fn expect_ref<'a, T, U>(obj: U) -> Result<&'a T, TestError> where U : TryInto<&'a T, Error = ()> {
    let received_type_name = std::any::type_name_of_val(&obj).to_string();
    obj.try_into().map_err(|_| UnexpectedCastTypeError(std::any::type_name::<T>().to_string()))
}

#[test]
fn test_local_remove_before_merging_please() -> Result<(), TestError> {
    let input = include_str!("../../../local/test.puff");
    let mut parser = PuffinParser::new(input, "test.puff");
    let mut ast = match parser.run() {
        Ok(ast) => ast,
        Err(err) => {
            println!("{}", err.to_string().red());
            return Err(TestError::ParserError(err))
        }
    };
    assert_eq!(ast.component_name, "test");
    Ok(())
}

/*#[test]
fn test_component_methods() -> Result<(), TestError> {
    let input = r#"
        component MyComponent(foo, bar, baz) {
            fn foo(one, two, three) {
            }

            @onclick(Lmb)
            fn bar(four, five, six) {
                let one = two + three;
            }
        }
    "#;
    let mut parser = PuffinParser::new(input, "<test_return>");
    let result = parser.component_decl();
    if let Err(e) = &result {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(result.is_ok());

    /*let component = expect::<ComponentDeclaration, Declaration>(result.unwrap())?;
    assert_eq!(component.name.lexeme, "MyComponent");
    assert_eq!(component.parameters.len(), 3);
    assert_eq!(component.parameters.iter().map(|t| t.lexeme.as_str()).collect::<Vec<_>>(), vec!["foo", "bar", "baz"]);*/

    let func_a = expect_ref::<MethodDeclaration, &Declaration>(&component.declarations[0])?;
    assert_eq!(func_a.name.lexeme, "foo");
    assert_eq!(func_a.parameters.iter().map(|t| t.lexeme.as_str()).collect::<Vec<_>>(), vec!["one", "two", "three"]);
    let block_a = expect_ref::<BlockStatement, &Statement>(&func_a.block)?;
    assert_eq!(block_a.statements.len(), 0);

    let func_b = expect_ref::<MethodDeclaration, &Declaration>(&component.declarations[1])?;
    assert_eq!(func_b.name.lexeme, "bar");
    assert!(func_b.decorator.is_some());

    let decorator_b = func_b.decorator.as_ref().unwrap();
    assert_eq!(decorator_b.name.lexeme, "onclick");
    assert_eq!(decorator_b.parameters.len(), 1);
    assert_eq!(decorator_b.parameters[0].lexeme, "Lmb");
    assert_eq!(decorator_b.parameters[0].ty, TokenType::Identifier);

    let stats = &expect_ref::<BlockStatement, &Statement>(&func_b.block)?.statements;
    assert_eq!(stats.len(), 1);

    let var_stat = expect_ref::<VariableDeclarationStatement, &Statement>(&stats[0])?;
    assert_eq!(var_stat.name.lexeme, "one");
    assert!(var_stat.catch_block.is_none());
    let val_expr = expect_ref::<BinaryExpression, &Expression>(&var_stat.value)?;
    let lhs = expect_ref::<LiteralExpression, &Expression>(&val_expr.lhs)?;
    let rhs = expect_ref::<LiteralExpression, &Expression>(&val_expr.rhs)?;
    assert_eq!(val_expr.op.lexeme, "+");
    assert_eq!(val_expr.op.ty, TokenType::Plus);
    assert_eq!(lhs.token.ty, TokenType::Identifier);
    assert_eq!(rhs.token.ty, TokenType::Identifier);
    assert_eq!(lhs.token.lexeme, "two");
    assert_eq!(rhs.token.lexeme, "three");
    Ok(())
}*/

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
fn test_return() -> Result<(), TestError> {
    let input = r#"
        fn func() {
            return 1 > 2;
        }
        fn other() {
            return;
        }
        fn other_other() {
            return 1;
        }
    "#;
    let mut parser = PuffinParser::new(input, "<test_return>");
    let result = parser.run();
    if let Err(e) = &result {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(result.is_ok());
    let res = result?;
    let func_a = expect_ref::<MethodDeclaration, &Declaration>(&res.declarations[0])?;
    let func_b = expect_ref::<MethodDeclaration, &Declaration>(&res.declarations[1])?;
    let func_c = expect_ref::<MethodDeclaration, &Declaration>(&res.declarations[2])?;
    let func_a_body = validate_method_decl(func_a, "func", vec![])?;
    let func_b_body = validate_method_decl(func_b, "other", vec![])?;
    let func_c_body = validate_method_decl(func_c, "other_other", vec![])?;
    assert_eq!(func_a_body.statements.len(), 1);
    assert_eq!(func_a_body.statements.len(), 1);
    assert_eq!(func_a_body.statements.len(), 1);
    let return_a = expect_ref::<ReturnStatement, &Statement>(&func_a_body.statements[0])?;
    let return_b = expect_ref::<ReturnStatement, &Statement>(&func_b_body.statements[0])?;
    let return_c = expect_ref::<ReturnStatement, &Statement>(&func_c_body.statements[0])?;
    assert_eq!(return_a.expression.is_some(), true);
    assert_eq!(return_b.expression.is_none(), true);
    assert_eq!(return_c.expression.is_some(), true);
    let expr_a = expect_ref::<BinaryExpression, &Expression>(&(*return_a.expression.as_ref().unwrap()))?;
    let lhs_a = expect_ref::<LiteralExpression, &Expression>(&(*expr_a.lhs.as_ref()))?;
    let rhs_a = expect_ref::<LiteralExpression, &Expression>(&(*expr_a.rhs.as_ref()))?;
    assert_eq!(lhs_a.token.lexeme, "1");
    assert_eq!(lhs_a.token.ty, TokenType::Integer);
    assert_eq!(expr_a.op.lexeme, ">");
    assert_eq!(expr_a.op.ty, TokenType::GreaterThan);
    assert_eq!(rhs_a.token.lexeme, "2");
    assert_eq!(rhs_a.token.ty, TokenType::Integer);
    let expr_c = expect_ref::<BinaryExpression, &Expression>(&(*return_a.expression.as_ref().unwrap()))?;
    let expr_c = expect_ref::<LiteralExpression, &Expression>(&(*expr_a.lhs.as_ref()))?;
    assert_eq!(expr_c.token.lexeme, "1");
    assert_eq!(expr_c.token.ty, TokenType::Integer);
    Ok(())
}

#[test]
fn test_comments() {
    let input = r#"
        /* This method does stuff */
        fn foo() {
            // TODO: Make this do stuff.
            return;
        }
    "#;
    let mut parser = PuffinParser::new(input, "<test_return>");
    let result = parser.run();
    if let Err(e) = &result {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(result.is_ok())
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
fn test_dictionary() -> Result<(), TestError> {
    let input = r#"
        let dict = {
            foo: "One",
            bar: "Two",
            baz: "Three",
        };
    "#;
    let mut parser = PuffinParser::new(input, "<test_dictionary>");
    let result = parser.var_decl(false);
    if let Err(e) = &result {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(result.is_ok());
    let inner = expect::<VarDeclaration, Declaration>(result.unwrap())?;
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
    Ok(())
}

#[test]
fn test_error() {
    let input = r#"
        error {
            SomeError,
            OtherError,
        }

        fn foo() {
            throw SomeError;
        }

        fn main() {
            let a = foo() catch {
                SomeError => 10,
                default => null,
            };

            // Potential feature
            // foo() catch handler;

            foo() catch {
                SomeError => {
                    // Do stuff
                },
                OtherError => return,
                default => raise,
            }
        }
    "#;
    let result = run_parser_str(input);
    if let Err(e) = &result {
        println!("{}", format!("parse error: {:}", e).red());
    }
    assert!(result.is_ok());
}