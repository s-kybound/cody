//! Node types for the parser.

#[derive(Clone)]
pub enum ExpressionAST {
    // variables
    VariableExpr(String),

    // data
    IntegerExpr(i32), 
    ListExpr(Vec<ExpressionAST>), // list of data
    FunctionExpr(Vec<String>, Box<ExpressionAST>), // function parameters and expression
    ContExpr(Box<ExpressionAST>),  // continuation expression

    // definitions
    DefineExpr(String, Box<ExpressionAST>), // identifier and expression

    // calls
    CallExpr(ExpressionAST, Vec<ExpressionAST>), // function and arguments

    // conditionals
    IfExpr(Box<ExpressionAST>, Box<ExpressionAST>, Box<ExpressionAST>), // predicate, then, else

    // match case
    MatchExpr(Box<ExpressionAST>, Vec<MatchArmExpr>), // expression and match arms
    MatchArmExpr(Vec<ExpressionAST>, Box<ExpressionAST>),  // patterns and expression

    // sequence expressions
    SeqExpr(Vec<ExpressionAST>), // list of expressions, sequences evaluate to their last expression
}