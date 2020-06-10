//! The AST (abstract-syntax-tree) types with an integrated type checker.

/// The main statement enum, allowing for grouping of statements inside of the
/// AST.
#[derive(Debug, PartialEq)]
pub enum StatementNode {
    Function(Function),         // Function
    WhileLoop(WhileLoop),       // While loop
    Variable(Variable),         // Variable decleration
    If(If),                     // If statement
    Expression(ExpressionNode), // Encapsulates a single expression.
    Return(ExpressionNode),     // Return statement
}

/// The main expression enum, allowing for grouping of all expressions inside
/// of the AST.
#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    /// A binary operation (basic maths) involving 2 recursive
    /// [ExpressionNode]s.
    BinOp(Box<ExpressionNode>, BinOp, Box<ExpressionNode>),

    /// A constant forming from [Constant]
    Constant(Constant),

    /// Pointer to a [Variable]/[Parameter] that may or may not be existant.
    VariablePoint(String),

    /// Function call. Read [FunctionCall] for more info
    FunctionCall(FunctionCall),
}

/// A node for an if statement, giving an expression to evaluate along with a
/// `body` of multiple statements.
///
/// # Syntax example
///
/// ```zypo
/// if(2 + 1 == 3) {
///     --snip--
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: ExpressionNode,
    pub body: Vec<StatementNode>,
}

/// A binary operation type. Having a seperate enum for this avoids repitition
/// and allows for some optimisations downstream.
#[derive(Debug, PartialEq)]
pub enum BinOp {
    /// Addition. Example: `10+14` is `24`
    Add,

    /// Subtraction. Example: `5-3` is `2`
    Sub,

    /// Division. Example: `32/4` is `8`
    Div,

    /// Multiplication. Example: `5*11` is `55`
    Mul,

    /// Power of. Example: (`3^3` or `3**3`) is `81`
    Power,

    /// Modulo. Example: `20 % 2` is `0`
    Mod,

    /// Equals to. Example: `20 == 2` is `false`
    IsEqual,

    /// Not equals to. Example: `20 == 2` is `true`. This is the opposite of
    /// [BinOp::IsEqual]
    NotEqual,

    /// Greater than operator. Example: `5 > 6` is `false`.
    GreaterThan,

    /// Less than operator. Example: `5 < 6` is `true`.
    LessThan,

    /// Similar to [BinOp::LessThan] and [BinOp::IsEqual] combined. If it is
    /// less than `x` OR equal to `x`.
    LessThanOrEqual,

    /// Similar to [BinOp::GreaterThan] and [BinOp::IsEqual] combined. If it is
    /// greater than `x` OR equal to `x`.
    GreaterThanOrEqual,

    /// Increment x by y. Take this syntax for example: `x += y`.
    PlusAssign,

    /// Decrement x by y. Take this syntax for example: `x += y`.
    SubtractAssign,

    /// An `or` / `||` operator for checking expressions that have to include 1
    /// or more as `true` (similar to how [BinOp::IsEqual] works).
    Or,

    /// An `and` / `&&` operator for checking expressions that have to include
    /// all as `true`.
    And,
}

/// A while loop. This goes down into [WhileLoop].
///
/// # Syntax example
///
/// ```zypo
/// while(50 > 49) {
///     // --snip--
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: ExpressionNode,
    pub body: Vec<StatementNode>,
}

/// A function call expression.
///
/// # Syntax example
///
/// ```zypo
/// hello(34, "hi")
/// ```
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub ident: String,
    pub expr_params: Vec<ExpressionNode>,
}

/// All the builtin constant types
#[derive(Debug, PartialEq)]
pub enum Constant {
    /// Integer constant that translates to [i32].
    Int(i32),

    // String constant that translates to [String].
    Str(String),

    // Bool constant that translates to [bool].
    Bool(bool),
}

/// A function statement, the basis for the whole language.
///
/// # Syntax example
///
/// ```zypo
/// fun number_adder(first_num: int, second_num: int) -> int {
///     --snip--
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Function {
    /// Function identifier.
    pub ident: String,

    /// [Parameter]s declared inside of the function signature.
    pub params: Vec<String>,

    /// The main body inside of the function.
    pub body: Vec<StatementNode>,

    /// A markdown-compatible [String] that is a documenation comment.
    ///
    /// NOTE: This may be bound to a trait in the future to extend to other
    /// datatypes.
    pub docs: Option<String>,
}

/// Variable that includes a parameter but extends with a recursive
/// [ExpressionNode].
///
/// # Syntax example
///
/// ```zypo
/// var x: int = 3432;
/// ```
#[derive(Debug, PartialEq)]
pub struct Variable {
    /// Variable name/identifier.
    pub ident: String,

    /// An expression body that can be evaluated (1x [ExpressionNode]).
    pub body: Box<ExpressionNode>,
}
