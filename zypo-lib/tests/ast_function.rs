use zypo_lib::parser::ast::*;
use zypo_lib::parser::ast_result;

/// Tests for the correct output of a basic function decleration with no
/// defined body.
///
/// This checks valid input.
#[test]
fn basic_function() {
    let input_str = "fun hello_there(hi: str) {}";
    let expected = vec![Function {
        ident: "hello_there".to_string(),
        params: vec![Parameter {
            ident: "hi".to_string(),
            ty: VarType::Str,
        }],
        docs: None,
        body: vec![],
        return_type: VarType::Void,
    }];

    assert_eq!(ast_result(input_str), expected);
}

/// Tests a function with a specified return type and a `return` statement inside
/// of the body.
///
/// This checks valid input.
#[test]
fn body_return() {
    let input_str = "fun return_function() -> int { return 5; }";
    let expected = vec![Function {
        ident: "return_function".to_string(),
        params: vec![],
        body: vec![StatementNode::Return(ExpressionNode::Constant(
            Constant::Int(5),
        ))],
        docs: None,
        return_type: VarType::Int,
    }];

    assert_eq!(ast_result(input_str), expected);
}

/// Tests the documentation properties of a function with the `---` syntax.
///
/// This checks valid input.
#[test]
fn docstrings() {
    let input_str = "--- This is a docstring test!\nfun doc_test() {}";
    let expected = vec![Function {
        ident: "doc_test".to_string(),
        params: vec![],
        body: vec![],
        docs: Some("This is a docstring test!".to_string()),
        return_type: VarType::Void,
    }];

    assert_eq!(ast_result(input_str), expected);
}
