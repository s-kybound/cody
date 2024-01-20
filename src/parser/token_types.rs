//! Token types for the programming language cody.

/// The different types of tokens that the lexer can produce.
#[derive(Clone, Debug)]
pub enum Token {

    // data types

    // atomic data types
    // Char(char),
    Integer(i32),
    // Float(f32),
    // Bool(Boolean),

    // defined data types
    Function,
    // Enum(String),
    // BitType(String),
    
    // syntax 

    // scope brackets
    LeftPar, RightPar,
    
    // pair syntax 
    LeftBkt, RightBkt, Dot, 
    
    // quote syntax
    Grave, Quote, At,

    // sequence expressions
    Seq,

    // definition
    Define,

    // identifiers 
    Identifier(String), 

    // conditionals
    If,
    
    // match case
    Match, Pipe, Arrow,

    // continuation
    Cont,

    // external functions
    Extern,

    // atomic binary operators
    AtomicOp(AtomBinary),

    EOF,
}

/// The different types of atomic binary operators.
#[derive(Clone, Debug)]
pub enum AtomBinary {
    Add,
    Sub,
    Mul,
    Div,
    // Mod,
    // Pow,
    And,
    Or,
    Not,
    // Xor,
    // Shl,
    // Shr,
    Eq,
    Lt,
    // Leq,
    // Geq,
}