use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::{FunctionType, BasicType};
use inkwell::values::{FunctionValue, BasicValue, IntValue};

use crate::parser::node_types::ExpressionAST;

pub fn construct(ast: ExpressionAST, output: &str) {
    let context = Context::create();
    let module = context.create_module(output);
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let fn_value = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(basic_block);

    let i32_type = context.i32_type();
    let const_int = i32_type.const_int(42, false);
    builder.build_return(Some(&const_int));

    module.print_to_stderr();
}