use crate::parser::ast::*;
use crate::parser::grammar;

/// Gets the abstract syntax tree generated from the parser of `zypo-lib`.
/// 
/// Please see [Function] for more infomation of what this will return if it
/// succeeds parsing.
/// 
/// This function is intended to be a quick shortcut if you don't mind a possible
/// `panic()` occuring upon a failed parse. If you do, you may want to look at
/// directly using:
/// 
/// ```norun
/// match grammar::GrammarParser::new().parse("fun empty() {}") {
///     Ok(_) => println!("It works!"),
///     Err(_) => eprintln!("Nope..")
/// };
/// ```
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
///     let input_str = "fun hello_there(hi) {}";
///     let expected = vec![Function {
///         ident: "hello_there".to_string(),
///         params: vec!["hi".to_string()],
///         docs: None,
///         body: vec![],
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
///     }];
///
///     assert_eq!(ast_result(input_str), expected);
/// }
/// ```
pub fn ast_result(input: &str) -> Vec<Function> {
    grammar::GrammarParser::new().parse(input).unwrap()
}
