use std::error::Error;
use crate::run_parser_str;
use colored::Colorize;
use puffin_ast::{Ast, VarType};
use puffin_ast::declaration::{ComponentDeclaration, Declaration, ErrorDeclaration, MethodDeclaration, VarDeclaration};
use puffin_ast::expression::{BinaryExpression, DictionaryExpression, Expression, FunctionCallExpression, LiteralExpression};
use puffin_ast::statement::{AssignStatement, BlockStatement, ExpressionStatement, ForStatement, ReturnStatement, Statement, VariableDeclarationStatement};
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
    #[error("Unexpected null when unwrapping")]
    UnexpectedNull,
}

fn validate_method_decl(decl: &MethodDeclaration, expected_name: impl AsRef<str>, parameters: Vec<String>) -> Result<&BlockStatement, TestError> {
    assert_eq!(decl.name.lexeme, expected_name.as_ref());
    assert_eq!(decl.parameters.iter().map(|p| p.lexeme.to_owned()).collect::<Vec<String>>(), parameters);
    expect::<&Statement, &BlockStatement>(decl.block.as_ref())
}
fn get_ast<'a>(mut parser: PuffinParser) -> Result<Ast, TestError> {
    match parser.run() {
        Ok(ast) => Ok(ast),
        Err(err) => {
            println!("{}", err.to_string().red());
            Err(TestError::ParserError(err))
        }
    }
}

fn expect<'a, T, U: 'a>(obj: T) -> Result<U, TestError> where U : TryFrom<T, Error = ()> {
    let received_type_name = std::any::type_name_of_val(&obj).to_string();
    U::try_from(obj).map_err(|_| UnexpectedCastTypeError(std::any::type_name::<T>().to_string()))
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
fn test_for() -> Result<(), TestError> {
    let src = r#"
        fn func() {
            for i in foo {
                for j in 0:10 {
                    print("Hello, world!");
                }
            }
        }
    "#;
    let ast = get_ast(PuffinParser::new(src, "<test_for>"))?;
    assert_eq!(ast.declarations.len(), 1);
    let func = expect::<&Declaration, &MethodDeclaration>(&ast.declarations[0])?;
    assert_eq!(func.name.lexeme, "func");
    assert!(func.decorator.is_none());
    assert!(func.parameters.is_empty());
    let outer_block = expect::<&Statement, &BlockStatement>(&func.block)?;
    assert_eq!(outer_block.statements.len(), 1);
    let outer_loop = expect::<&Statement, &ForStatement>(&outer_block.statements[0])?;
    assert_eq!(outer_loop.var_name.lexeme, "i");
    assert!(outer_loop.end_range.is_none());
    let inner_block = expect::<&Statement, &BlockStatement>(&outer_loop.block)?;
    assert_eq!(inner_block.statements.len(), 1);
    let inner_loop = expect::<&Statement, &ForStatement>(&inner_block.statements[0])?;
    assert_eq!(inner_loop.var_name.lexeme, "j");
    let inner_end_range = if let Some(e) = &inner_loop.end_range {
        Ok(expect::<&Expression, &LiteralExpression>(e)?)
    } else {
        Err(TestError::UnexpectedNull)
    }?;
    let inner_iter = expect::<&Expression, &LiteralExpression>(&inner_loop.iterable)?;
    assert_eq!(inner_iter.token.lexeme, "0");
    assert_eq!(inner_end_range.token.lexeme, "10");
    let logic_block = expect::<&Statement, &BlockStatement>(&inner_loop.block)?;
    let call_stat = expect::<&Statement, &ExpressionStatement>(&logic_block.statements[0])?;
    let call_expr = expect::<&Expression, &FunctionCallExpression>(&call_stat.expression)?;
    let callee = expect::<&Expression, &LiteralExpression>(&call_expr.callee)?;
    assert_eq!(call_expr.arguments.len(), 1);
    let arg = expect::<&Expression, &LiteralExpression>(&call_expr.arguments[0])?;
    assert_eq!(arg.token.lexeme, "\"Hello, world!\"");
    assert_eq!(callee.token.lexeme, "print");
    Ok(())
}
#[test]
fn test_return() -> Result<(), TestError> {
    let src = r#"
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
    let ast = get_ast(PuffinParser::new(src, "test_return.puff"))?;
    let func_a = expect::<&Declaration, &MethodDeclaration>(&ast.declarations[0])?;
    let func_b = expect::<&Declaration, &MethodDeclaration>(&ast.declarations[1])?;
    let func_c = expect::<&Declaration, &MethodDeclaration>(&ast.declarations[2])?;
    let func_a_body = validate_method_decl(func_a, "func", vec![])?;
    let func_b_body = validate_method_decl(func_b, "other", vec![])?;
    let func_c_body = validate_method_decl(func_c, "other_other", vec![])?;
    assert_eq!(func_a_body.statements.len(), 1);
    assert_eq!(func_a_body.statements.len(), 1);
    assert_eq!(func_a_body.statements.len(), 1);
    let return_a = expect::<&Statement, &ReturnStatement>(&func_a_body.statements[0])?;
    let return_b = expect::<&Statement, &ReturnStatement>(&func_b_body.statements[0])?;
    let return_c = expect::<&Statement, &ReturnStatement>(&func_c_body.statements[0])?;
    assert_eq!(return_a.expression.is_some(), true);
    assert_eq!(return_b.expression.is_none(), true);
    assert_eq!(return_c.expression.is_some(), true);
    let expr_a = expect::<&Expression, &BinaryExpression>(&(*return_a.expression.as_ref().unwrap()))?;
    let lhs_a = expect::<&Expression, &LiteralExpression>(&(*expr_a.lhs.as_ref()))?;
    let rhs_a = expect::<&Expression, &LiteralExpression>(&(*expr_a.rhs.as_ref()))?;
    assert_eq!(lhs_a.token.lexeme, "1");
    assert_eq!(lhs_a.token.ty, TokenType::Integer);
    assert_eq!(expr_a.op.lexeme, ">");
    assert_eq!(expr_a.op.ty, TokenType::GreaterThan);
    assert_eq!(rhs_a.token.lexeme, "2");
    assert_eq!(rhs_a.token.ty, TokenType::Integer);
    let expr_c = expect::<&Expression, &BinaryExpression>(&(*return_a.expression.as_ref().unwrap()))?;
    let expr_c = expect::<&Expression, &LiteralExpression>(&(*expr_a.lhs.as_ref()))?;
    assert_eq!(expr_c.token.lexeme, "1");
    assert_eq!(expr_c.token.ty, TokenType::Integer);
    Ok(())
}

#[test]
fn test_comments() -> Result<(), TestError> {
    let src = r#"
        /* This method does stuff */
        fn foo() {
            // TODO: Make this do stuff.
            return;
        }
    "#;
    let ast = get_ast(PuffinParser::new(src, "<test_return>"))?;
    let func = expect::<&Declaration, &MethodDeclaration>(&ast.declarations[0])?;
    assert_eq!(func.name.lexeme, "foo");
    assert_eq!(func.parameters.len(), 0);
    assert!(func.decorator.is_none());
    let block = expect::<&Statement, &BlockStatement>(&func.block)?;
    assert_eq!(block.statements.len(), 1);
    let ret = expect::<&Statement, &ReturnStatement>(&block.statements[0])?;
    assert!(ret.expression.is_none());
    Ok(())
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
    let ast = get_ast(PuffinParser::new(input, "<test_dictionary>"))?;
    assert_eq!(ast.component_name, "<test_dictionary>");
    assert_eq!(ast.declarations.len(), 1);
    let inner = expect::<&Declaration, &VarDeclaration>(&ast.declarations[0])?;
    assert_eq!(inner.var_type, VarType::Let);
    assert_eq!(inner.name.lexeme, "dict");
    let dict = expect::<&Expression, &DictionaryExpression>(&inner.value)?;
    assert_eq!(dict.entries.len(), 3);
    assert_eq!(&dict.entries[0].0.lexeme, "foo");
    assert_eq!(&dict.entries[1].0.lexeme, "bar");
    assert_eq!(&dict.entries[2].0.lexeme, "baz");
    let expr_a = expect::<&Expression, &LiteralExpression>(&dict.entries[0].1)?;
    let expr_b = expect::<&Expression, &LiteralExpression>(&dict.entries[1].1)?;
    let expr_c = expect::<&Expression, &LiteralExpression>(&dict.entries[2].1)?;
    assert_eq!(expr_a.token.lexeme, "\"One\"");
    assert_eq!(expr_b.token.lexeme, "\"Two\"");
    assert_eq!(expr_c.token.lexeme, "\"Three\"");
    Ok(())
}

#[test]
fn test_error() -> Result<(), TestError> {
    let src = r#"
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
    let ast = get_ast(PuffinParser::new(src, "<test_error>"))?;
    assert_eq!(ast.component_name, "<test_error>");
    assert_eq!(ast.declarations.len(), 3);
    expect::<&Declaration, &ErrorDeclaration>(&ast.declarations[0])?;
    Ok(())
}

#[test]
fn test_example_program() -> Result<(), TestError> {
    let src = r#"
    require "std";

    use std.git;

    export enum State {
        MainMenu,
        Diff,
        List,
    }

    // State
    let current_state = State.MainMenu;
    let diffs = ["Some", "Diffs"];
    let commits = [];

    // Constants
    const menu_id = "menu";
    const diff_id = "diff";
    const list_id = "list";

    // Utility functions
    fn refresh_diffs() {
        return git.get_diff_list();
    }

    new() {
        diffs = ["Some", "Diffs?"];
    }

    fn refresh_commits() {
        let foo = 1 + 2 * 3 + (8 * -4 + 8);
        return git.get_commit_log();
    }

    export fn set_state(new_state) {
        current_state = new_state;

        match current_state {
            State.Diff => refresh_diffs(),
            State.List => refresh_commits(),
        }
    }

    @onclick(MouseLeft)
    fn click(key, state, el) {
        const new_state = match el.id {
            menu_id => State.MainMenu,
            diff_id => State.Diff,
            list_id => State.List,
        };

        set_state(new_state);
    }

    layout {
        match current_state {
            State.MainMenu => menu_layout,
            State.Diff => diff_layout,
            State.List => ListComponent commits=commits,
            default => main_menu,
        }
    }

    layout menu_layout {
        vbox {
            text "Git Tool";
            button click={set_state(State.Diff)} "Diff";
            button click={set_state(State.List)} "List";

            // button click(ev)={subscriber(any_var)} "Button";
            // button click=subscriber "Button";
            // button click=[sub1, sub2] "Button";

            // Not allowed
            // button click(ev)=subscriber "Button";
            // button click(ev)=[sub1, sub2] "Button";
        }
    }

    layout diff_layout {
        vbox {
            text "Git Tool - Diff";
            for diff in diffs {
                text "${diff}";
            }
        }
    }

    component ListComponent(commits) {
        let component_state = 0;

        @onkey(ArrowUp)
        fn scroll_up(key) {
            if component_state > 0 {
                component_state--;
            }
        }

        @onkey(ArrowDown)
        fn scroll_down(key) {
            if component_state < commits.len() - 1 {
                component_state++;
            }
        }

        layout {
            vbox {
                text "Git Tool - List";

                for i in 0:len(commits) {
                    commit_layout(i, commits[i]);
                }
            }
        }

        layout commit_layout(i, commit) {
            vbox {
                if i == component_state {
                    style text_color=black;
                    style background_color=white;
                } else {
                    style {
                        text_color=white;
                        background_color=black;
                    }
                }

                hbox {
                    text "${commit.who}";
                    text "${commit.when}";
                }

                text "${commit.msg}";
            }
        }
    }

    "#;
    let mut ast = get_ast(PuffinParser::new(src, "test_example_program.puff"))?;
    assert_eq!(ast.component_name, "test_example_program");
    Ok(())
}