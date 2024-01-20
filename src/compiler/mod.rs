use crate::parser::node_types;

pub mod ir_constructor;

pub fn compile(ast: node_types::ExpressionAST, output: &str) {
    ir_constructor::construct(ast, output);
}