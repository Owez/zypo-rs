use zypo_lib::parser::ast::*;
use zypo_lib::parser::ast_result;

/// Tests a simple while loop with expression inside.
/// 
/// This checks valid input.
#[test]
fn while_loop_basic() {
    let input_str = "fun test_while() { while(1 == 1) {} }";
    let expected = vec![Function {
        ident: "test_while".to_string(),
        params: vec![],
        body: vec![StatementNode::WhileLoop(WhileLoop {
            condition: ExpressionNode::BinOp(
                Box::new(ExpressionNode::Constant(Constant::Int(1))),
                BinOp::IsEqual,
                Box::new(ExpressionNode::Constant(Constant::Int(1))),
            ),
            body: vec![],
        })],
        docs: None,
        return_type: VarType::Void,
    }];

    assert_eq!(ast_result(input_str), expected);
}

/// Tests a while loop with a simple body (variable declaration).
/// 
/// This checks valid input.
#[test]
fn while_loop_body() {
    let input_str = "fun test_while() { while(2 == 2) { var x: int = 1; } }";
    let expected = vec![Function {
        ident: "test_while".to_string(),
        params: vec![],
        body: vec![StatementNode::WhileLoop(WhileLoop {
            condition: ExpressionNode::BinOp(
                Box::new(ExpressionNode::Constant(Constant::Int(2))),
                BinOp::IsEqual,
                Box::new(ExpressionNode::Constant(Constant::Int(2))),
            ),
            body: vec![StatementNode::Variable(Variable {
                ident: "x".to_string(),
                ty: VarType::Int,
                body: Box::new(ExpressionNode::Constant(Constant::Int(1))),
            })],
        })],
        docs: None,
        return_type: VarType::Void,
    }];

    assert_eq!(ast_result(input_str), expected);
}
