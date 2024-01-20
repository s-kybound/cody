mod ast_generator;
mod lexer;

pub mod node_types;
pub mod token_types;

/// Parses a program string into an AST.
pub fn parse(program: &str) -> node_types::ExpressionAST {
    let tokens = lexer::lex(program);
    ast_generator::ast_generate(&tokens)
}