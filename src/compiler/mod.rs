use crate::parser::node_types;

pub mod ast_converter;
pub mod ir_constructor;
pub mod scope;

pub fn compile(ast: node_types::ExpressionAST, output: &str) {
    ir_constructor::construct(ast, output);
}