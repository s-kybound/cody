//! Token types for the programming language cody.

#[derive(Debug)]

/// The different types of tokens that the lexer can produce.
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

    // sequence expressions
    Seq,

    // definition
    Define,

    // identifiers 
    Identifier(String), 

    // conditionals
    If, Else,
    
    // match case
    Match, Pipe, Arrow,

    // continuation
    Cont,
}