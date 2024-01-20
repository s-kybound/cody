//! Node types for the parser.

#[derive(Clone, Debug)]
pub enum ExpressionAST {
    // variables
    VariableExpr(String),

    // data
    IntegerExpr(i32),
    NoneExpr, 
    PairExpr(Box<ExpressionAST>, Box<ExpressionAST>), // pair data
    FunctionExpr(Vec<ExpressionAST>, Box<ExpressionAST>), // function parameters and expression
    //ContExpr(Box<ExpressionAST>),  // continuation expression

    // definitions
    DefineExpr(Box<ExpressionAST>, Box<ExpressionAST>), // identifier and expression

    // calls
    CallExpr(Box<ExpressionAST>, Vec<ExpressionAST>), // function and arguments

    // conditionals
    IfExpr(Box<ExpressionAST>, Box<ExpressionAST>, Box<ExpressionAST>), // predicate, then, else

    // match case
    MatchExpr(Box<ExpressionAST>, Vec<ExpressionAST>), // expression and match arms
    MatchArmExpr(Vec<ExpressionAST>, Box<ExpressionAST>),  // patterns and expression

    // sequence expressions
    SeqExpr(Vec<ExpressionAST>), // list of expressions, sequences evaluate to their last expression

    // external functions
    // ie calling c library sin with ((extern sin) 1.0)
    //ExternExpr(Box<ExpressionAST>), // name of the external function 
}