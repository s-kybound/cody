mod lexer;
mod token_types;
mod node_types;
mod ast_generator;

/// Parses a program string into an AST.
pub fn parse(program: &str) -> node_types::ExpressionAST {
    let tokens = lexer::lex(program);
    ast_generator::ast_generate(&tokens)
}