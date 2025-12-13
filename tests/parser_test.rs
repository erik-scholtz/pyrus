use pyrus::ast::{BinaryOp, Expression, Statement, UnaryOp};
use pyrus::parser::parse;

#[test]
fn test_parse_empty_document() {
    let source = "document { }";
    let ast = parse(source);
    assert!(ast.document.is_some());
    assert!(ast.template.is_none());
    assert!(ast.style.is_none());

    let doc = ast.document.unwrap();
    assert_eq!(doc.statements.len(), 0);
}

#[test]
fn test_parse_empty_template() {
    let source = "template { }";
    let ast = parse(source);
    assert!(ast.template.is_some());
    assert!(ast.document.is_none());
    assert!(ast.style.is_none());

    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 0);
}

#[test]
fn test_parse_empty_style() {
    let source = "style { }";
    let ast = parse(source);
    assert!(ast.style.is_some());
    assert!(ast.template.is_none());
    assert!(ast.document.is_none());
}

#[test]
fn test_parse_all_blocks() {
    let source = "template { } document { } style { }";
    let ast = parse(source);
    assert!(ast.template.is_some());
    assert!(ast.document.is_some());
    assert!(ast.style.is_some());
}

#[test]
fn test_parse_variable_assignment() {
    let source = "template { let x = \"hello\" }";
    let ast = parse(source);
    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 1);

    match &template.statements[0] {
        Statement::VarAssign { name, value } => {
            assert_eq!(name, "x");
            match value {
                Expression::StringLiteral(s) => assert_eq!(s, "hello"),
                _ => panic!("Expected StringLiteral expression"),
            }
        }
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_const_assignment() {
    let source = "template { const PI = \"3.14\" }";
    let ast = parse(source);
    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 1);

    match &template.statements[0] {
        Statement::ConstAssign { name, value } => {
            assert_eq!(name, "PI");
            match value {
                Expression::StringLiteral(s) => assert_eq!(s, "3.14"),
                _ => panic!("Expected StringLiteral expression"),
            }
        }
        _ => panic!("Expected ConstAssign statement"),
    }
}

#[test]
fn test_parse_unary_negation() {
    let source = "template { let x = - 42 }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { name, value } => {
            assert_eq!(name, "x");
            match value {
                Expression::Unary {
                    operator,
                    expression,
                } => {
                    match operator {
                        UnaryOp::Negate => {}
                        _ => panic!("Expected Negate operator"),
                    }
                    // Inner expression should be parsed
                }
                _ => panic!("Expected Unary expression"),
            }
        }
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_binary_addition() {
    let source = "template { let sum = x + y }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { name, value } => {
            assert_eq!(name, "sum");
            match value {
                Expression::Binary {
                    left,
                    operator,
                    right,
                } => {
                    match operator {
                        BinaryOp::Add => {}
                        _ => panic!("Expected Add operator"),
                    }
                    match (&**left, &**right) {
                        (Expression::Identifier(l), Expression::Identifier(r)) => {
                            assert_eq!(l, "x");
                            assert_eq!(r, "y");
                        }
                        _ => panic!("Expected identifier expressions"),
                    }
                }
                _ => panic!("Expected Binary expression"),
            }
        }
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_binary_subtraction() {
    let source = "template { let diff = a - b }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => match value {
            Expression::Binary { operator, .. } => match operator {
                BinaryOp::Subtract => {}
                _ => panic!("Expected Subtract operator"),
            },
            _ => panic!("Expected Binary expression"),
        },
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_binary_multiplication() {
    let source = "template { let product = a * b }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => match value {
            Expression::Binary { operator, .. } => match operator {
                BinaryOp::Multiply => {}
                _ => panic!("Expected Multiply operator"),
            },
            _ => panic!("Expected Binary expression"),
        },
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_binary_division() {
    let source = "template { let quotient = a / b }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => match value {
            Expression::Binary { operator, .. } => match operator {
                BinaryOp::Divide => {}
                _ => panic!("Expected Divide operator"),
            },
            _ => panic!("Expected Binary expression"),
        },
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_binary_equals() {
    let source = "template { let result = a = b }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => match value {
            Expression::Binary { operator, .. } => match operator {
                BinaryOp::Equals => {}
                _ => panic!("Expected Equals operator"),
            },
            _ => panic!("Expected Binary expression"),
        },
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_string_literal() {
    let source = "template { let msg = \"Hello, World!\" }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => match value {
            Expression::StringLiteral(s) => assert_eq!(s, "Hello, World!"),
            _ => panic!("Expected StringLiteral expression"),
        },
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_integer_literal() {
    let source = "template { let num = 42 }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => {
            match value {
                Expression::StringLiteral(s) => {
                    // Currently parsed as StringLiteral, should be NumberLiteral
                    assert_eq!(s, "42");
                }
                _ => panic!("Expected StringLiteral expression (for now)"),
            }
        }
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_float_literal() {
    let source = "template { let pi = 3.14 }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => {
            match value {
                Expression::StringLiteral(s) => {
                    // Currently parsed as StringLiteral
                    assert_eq!(s, "3.14");
                }
                _ => panic!("Expected StringLiteral expression (for now)"),
            }
        }
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_return_statement() {
    let source = "template { return \"done\" }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::Return(expr) => match expr {
            Expression::StringLiteral(s) => assert_eq!(s, "done"),
            _ => panic!("Expected StringLiteral in return"),
        },
        _ => panic!("Expected Return statement"),
    }
}

#[test]
fn test_parse_function_declaration() {
    let source = "template { func add(x, y) { let result = x + y return result } }";
    let ast = parse(source);
    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 1);

    match &template.statements[0] {
        Statement::FunctionDecl { name, params, body } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "x");
            assert_eq!(params[1].name, "y");
            assert!(body.len() > 0);
        }
        _ => panic!("Expected FunctionDecl statement"),
    }
}

#[test]
fn test_parse_function_call_no_args() {
    let source = "document { greet() }";
    let ast = parse(source);
    let doc = ast.document.unwrap();

    match &doc.statements[0] {
        Statement::FunctionCall {
            name,
            args,
            attributes,
        } => {
            assert_eq!(name, "greet");
            assert_eq!(args.len(), 0);
            assert_eq!(attributes.len(), 0);
        }
        _ => panic!("Expected FunctionCall statement"),
    }
}

#[test]
fn test_parse_function_call_with_args() {
    let source = "document { print(\"hello\", \"world\") }";
    let ast = parse(source);
    let doc = ast.document.unwrap();

    match &doc.statements[0] {
        Statement::FunctionCall {
            name,
            args,
            attributes,
        } => {
            assert_eq!(name, "print");
            assert_eq!(args.len(), 2);
            assert_eq!(attributes.len(), 0);
        }
        _ => panic!("Expected FunctionCall statement"),
    }
}

#[test]
fn test_parse_function_call_with_attributes() {
    let source = "document { button(class = \"primary\") }";
    let ast = parse(source);
    let doc = ast.document.unwrap();

    match &doc.statements[0] {
        Statement::FunctionCall {
            name,
            args,
            attributes,
        } => {
            assert_eq!(name, "button");
            assert_eq!(args.len(), 0);
            assert_eq!(attributes.len(), 1);
        }
        _ => panic!("Expected FunctionCall statement"),
    }
}

#[test]
fn test_parse_paragraph_block() {
    let source = "document { p { This is a paragraph } }";
    let ast = parse(source);
    let doc = ast.document.unwrap();
    assert_eq!(doc.statements.len(), 1);

    match &doc.statements[0] {
        Statement::Paragraph { value } => match value {
            Expression::StringLiteral(s) => {
                assert!(s.contains("paragraph"));
            }
            _ => panic!("Expected StringLiteral in paragraph"),
        },
        _ => panic!("Expected Paragraph statement"),
    }
}

#[test]
fn test_parse_default_set() {
    let source = "template { width = 100 }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::DefaultSet { key, value } => {
            assert_eq!(key, "width");
            match value {
                Expression::StringLiteral(s) => assert_eq!(s, "100"),
                _ => panic!("Expected StringLiteral"),
            }
        }
        _ => panic!("Expected DefaultSet statement"),
    }
}

#[test]
fn test_parse_multiple_statements() {
    let source = "template { let x = 1 let y = 2 let z = 3 }";
    let ast = parse(source);
    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 3);
}

#[test]
fn test_parse_mixed_statements() {
    let source = "template { let x = 10 const MAX = 100 width = 50 }";
    let ast = parse(source);
    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 3);

    match &template.statements[0] {
        Statement::VarAssign { .. } => {}
        _ => panic!("Expected VarAssign"),
    }
    match &template.statements[1] {
        Statement::ConstAssign { .. } => {}
        _ => panic!("Expected ConstAssign"),
    }
    match &template.statements[2] {
        Statement::DefaultSet { .. } => {}
        _ => panic!("Expected DefaultSet"),
    }
}

#[test]
fn test_parse_dollar_sign_interpolation() {
    let source = "template { let msg = $ x $ }";
    let ast = parse(source);
    let template = ast.template.unwrap();

    match &template.statements[0] {
        Statement::VarAssign { value, .. } => match value {
            Expression::Identifier(id) => {
                assert_eq!(id, "x");
            }
            _ => panic!("Expected Identifier expression"),
        },
        _ => panic!("Expected VarAssign statement"),
    }
}

#[test]
fn test_parse_nested_template_and_document() {
    let source = "template { func render() { return \"html\" } } document { p { Hello } }";
    let ast = parse(source);
    assert!(ast.template.is_some());
    assert!(ast.document.is_some());

    let template = ast.template.unwrap();
    assert_eq!(template.statements.len(), 1);

    let doc = ast.document.unwrap();
    assert_eq!(doc.statements.len(), 1);
}
