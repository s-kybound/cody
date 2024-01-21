use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::{FunctionType, BasicType, IntType};
use inkwell::values::{FunctionValue, BasicValue, GenericValue, IntValue, AsValueRef, PointerValue};

use crate::parser::node_types::ExpressionAST;
use crate::parser::token_types::AtomBinary;

use super::scope::{Scope, self};

pub trait Codegen {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> IntValue<'a>;
}

impl Codegen for ExpressionAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> IntValue<'a> {
        match self {
            // variables
            ExpressionAST::VariableExpr(s) => {
                match scope.get_variable(&s) {
                    Some(v) => builder.build_load(context.i32_type() ,v, &s)
                    .expect("Failed to load variable.")
                    .into_int_value(),
                    None => panic!("Variable {} not found in scope.", s)
                }
            },

            // values
            ExpressionAST::IntegerExpr(i) => context.i32_type().const_int(i as u64, false),
            // ExpressionAST::NoneExpr => context.i32_type().ptr_type(inkwell::AddressSpace::Generic).const_null(),
            // ExpressionAST::PairExpr(_, _) => ,
            // ExpressionAST::FunctionExpr(_, _) => context.i32_type().const_int(42, false).as_basic_value(),

            // definitions
            ExpressionAST::DefineExpr(var, val) => {
                let var_name = match *var {
                    ExpressionAST::VariableExpr(s) => s,
                    _ => panic!("Expected variable name in define expression.")
                };
                let val_value: IntValue<'a> = val.codegen(context, builder, scope);
                let val_type: IntType<'a> = val_value.get_type();
                let var_value: PointerValue<'a> = builder.build_alloca(val_type, var_name.as_str())
                    .expect("Failed to allocate variable");
                builder.build_store(var_value, val_value);
                scope.add_variable(var_name, var_value);
                val_value
            },

            // calls
            // ExpressionAST::CallExpr(_, _) => context.i32_type().const_int(42, false).as_basic_value(),
            
            // conditionals
            ExpressionAST::IfExpr(pred, conseq, alt) => {
                let pred_value: IntValue<'a> = pred.codegen(context, builder, scope);
                let pred_type: IntType<'a> = pred_value.get_type();
                let pred_bool_value: IntValue<'a> = builder.build_int_compare(inkwell::IntPredicate::NE, pred_value, pred_type.const_int(0, false), "ifcond")
                    .expect("Failed to build if condition.");
                let function: FunctionValue<'a> = builder.get_insert_block().unwrap().get_parent().unwrap();
                let then_block: inkwell::basic_block::BasicBlock = context.append_basic_block(function, "then");
                let else_block: inkwell::basic_block::BasicBlock = context.append_basic_block(function, "else");
                let merge_block: inkwell::basic_block::BasicBlock = context.append_basic_block(function, "ifcont");
                builder.build_conditional_branch(pred_bool_value, then_block, else_block);
                builder.position_at_end(then_block);
                let conseq_value: IntValue<'a> = conseq.codegen(context, builder, scope);
                builder.build_unconditional_branch(merge_block);
                let then_block: inkwell::basic_block::BasicBlock = builder.get_insert_block().unwrap();
                builder.position_at_end(else_block);
                let alt_value: IntValue<'a> = alt.codegen(context, builder, scope);
                builder.build_unconditional_branch(merge_block);
                let else_block: inkwell::basic_block::BasicBlock = builder.get_insert_block().unwrap();
                builder.position_at_end(merge_block);
                let phi_node: inkwell::values::PhiValue<'a> = builder.build_phi(context.i32_type(), "iftmp").unwrap();
                phi_node.add_incoming(&[(&conseq_value, then_block), (&alt_value, else_block)]);
                phi_node.as_basic_value().into_int_value()
            },

            // match case
            // ExpressionAST::MatchExpr(_, _) => context.i32_type().const_int(42, false).as_basic_value(),
            // ExpressionAST::MatchArmExpr(_, _) => context.i32_type().const_int(42, false).as_basic_value(),

            // sequence expressions
            ExpressionAST::SeqExpr(seq) => {
                let mut last = context.i32_type().const_int(0, false);
                for expr in seq {
                    last = expr.codegen(context, builder, scope);
                }
                last
            },

            // atomic binary expressions
            ExpressionAST::AtomBinExpr(op, l, r) => {
                let left = l.codegen(context, builder, scope);
                let right = r.codegen(context, builder, scope);
                match op {
                    AtomBinary::Add => builder.build_int_add(left, right, "add"),
                    AtomBinary::Sub => builder.build_int_sub(left, right, "sub"),
                    AtomBinary::Mul => builder.build_int_mul(left, right, "mul"),
                    AtomBinary::Div => builder.build_int_signed_div(left, right, "div"),
                    AtomBinary::And => builder.build_and(left, right, "and"),
                    AtomBinary::Or => builder.build_or(left, right, "or"),
                    AtomBinary::Not => builder.build_not(left, "not"),
                    AtomBinary::Eq => builder.build_int_compare(inkwell::IntPredicate::EQ, left, right, "eq"),
                    AtomBinary::Lt => builder.build_int_compare(inkwell::IntPredicate::SLT, left, right, "lt"),
                }.expect("Failed to build binary expression.")
            },

            // external functions
            //ExpressionAST::ExternExpr(_) => context.i32_type().const_int(42, false).as_basic_value(),

            _ => panic!("Expression not supported as of version 0.0.1: {:?}", self)
        }
    }
}