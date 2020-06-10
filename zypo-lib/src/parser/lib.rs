use crate::parser::ast::*;
use crate::parser::grammar;

/// Gets the abstract syntax tree generated from the parser of `zypo-lib`. This
/// function will panic is parsing fails and is intended for developers aiming
/// to implament the parser into code generation.
///
/// Please see [Function] for more infomation of what this will immidiatly
/// return.
///
/// # Examples
///
/// Basic function AST:
///
/// ```rust
/// use zypo_lib::parser::ast_result;
/// use zypo_lib::parser::ast::*;
///
/// fn main() {
///     let input_str = "fun hello_there(hi: str) {}";
///     let expected = vec![Function {
///         ident: "hello_there".to_string(),
///         params: vec![Parameter {
///             ident: "hi".to_string(),
///             ty: VarType::Str,
///         }],
///         docs: None,
///         body: vec![],
///         return_type: VarType::Void,
///     }];
///     assert_eq!(ast_result(input_str), expected);
/// }
/// ```
///
/// While loop:
///
/// ```rust
/// use zypo_lib::parser::ast::*;
/// use zypo_lib::parser::ast_result;
///
/// fn main() {
///     let input_str = "fun test_while() { while(1 == 1) {} }";
///     let expected = vec![Function {
///         ident: "test_while".to_string(),
///         params: vec![],
///         body: vec![StatementNode::WhileLoop(WhileLoop {
///             condition: ExpressionNode::BinOp(
///                 Box::new(ExpressionNode::Constant(Constant::Int(1))),
///                 BinOp::IsEqual,
///                 Box::new(ExpressionNode::Constant(Constant::Int(1))),
///             ),
///             body: vec![],
///         })],
///         docs: None,
///         return_type: VarType::Void,
///     }];
///
///     assert_eq!(ast_result(input_str), expected);
/// }
/// ```
pub fn ast_result(input: &str) -> Vec<Function> {
    grammar::GrammarParser::new().parse(input).unwrap()
}
