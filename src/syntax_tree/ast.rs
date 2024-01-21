use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::{FunctionType, BasicType, IntType, AnyType};
use inkwell::values::{FunctionValue, BasicValue, GenericValue, IntValue, AsValueRef, PointerValue, AnyValue, AnyValueEnum, BasicValueEnum, StructValue};

use crate::parser::token_types::{AtomBinary, AtomUnary};
use crate::compiler::scope::Scope;

pub trait Codegen {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>>;
}

impl Codegen for VariableExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        match scope.get_variable(&self.name) {
            Some(value) => Box::new(builder.build_load(context.i32_type(), value, &self.name).expect("Failed to load variable.").into_int_value()),
            None => panic!("Variable {} not found in scope.", self.name),
        }
    }
}

impl Codegen for IntegerExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>>{
        Box::new(context.i32_type().const_int(self.value as u64, false))
    }
}

impl Codegen for NoneExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        Box::new(context.i32_type().const_int(0, false))
    }
}

// impl Codegen for PairExprAST {
//     fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
//         let left = (self.left).codegen(context, builder, scope).as_any_value_enum().downcast::<BasicValueEnum>().unwrap();
//         let right = (self.right).codegen(context, builder, scope).as_any_value_enum();

//         let pair_type = context.struct_type(&[left.get_type(), right.get_type()], false);
//         let pair = pair_type.const_named_struct(&[left, right]);
//         let pair_pointer = builder.build_alloca(pair.get_type(), "pair");
//         builder.build_store(pair_pointer, pair);
//         pair
//     }
// }

impl Codegen for FunctionExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        let params_type: Vec<IntType> = self.parameters.iter().map(|_| context.i32_type()).collect();
        
        let function_type = context.i32_type().fn_type(&params_type, false);
        let function = context.append_basic_block(function_type, "function");
        let function_value = function.as_any_value_enum();

        let function_scope = Scope::new(Some(scope));
        for parameter in self.parameters {
            function_scope.add_variable(parameter.name, function_value);
        }

        let function_body = self.body.codegen(context, builder, &function_scope);
        builder.build_return(Some(& function_body));

        function_value
    }
}

impl Codegen for ContExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        let function_type = context.i32_type().fn_type(&[], false);
        let function = context.append_basic_block(function_type, "function");
        let function_value = function.as_any_value_enum();

        let function_scope = Scope::new(Some(scope));
        function_scope.add_variable(String::from("cont"), function_value);

        let function_body = self.body.codegen(context, builder, &function_scope);
        builder.build_return(Some(&function_body));

        function_value
    }
}

impl Codegen for DefineExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        let value = self.body.codegen(context, builder, scope).as_any_value_enum().into_int_value();
        let p_value = builder.build_alloca(value.get_type(), &self.name.name).unwrap();
        builder.build_store(p_value, value);
        scope.add_variable(self.name.name, p_value);
        value
    }
}

impl Codegen for CallExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>>{
        let function = self.function.codegen(context, builder, scope);
        let mut arguments: Vec<dyn AnyValue<'a>> = Vec::new();
        for argument in self.arguments {
            arguments.push(argument.codegen(context, builder, scope));
        }
        builder.build_call(function.as_any_value_enum().into_function_value(), &arguments, "call").as_any_value_enum()
    }
}

impl Codegen for IfExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        let condition = self.condition.codegen(context, builder, scope);
        let condition = builder.build_int_compare(inkwell::IntPredicate::NE, condition.into_int_value(), context.i32_type().const_int(0, false), "ifcond");

        let function = builder.get_insert_block().unwrap().get_parent().unwrap();
        let then_block = context.append_basic_block(function, "then");
        let else_block = context.append_basic_block(function, "else");
        let merge_block = context.append_basic_block(function, "ifcont");

        builder.build_conditional_branch(condition, then_block, else_block);

        builder.position_at_end(then_block);
        let then_value = self.then_body.codegen(context, builder, scope);
        builder.build_unconditional_branch(merge_block);
        let then_block = builder.get_insert_block().unwrap();

        builder.position_at_end(else_block);
        let else_value = self.else_body.codegen(context, builder, scope);
        builder.build_unconditional_branch(merge_block);
        let else_block = builder.get_insert_block().unwrap();

        builder.position_at_end(merge_block);
        let phi = builder.build_phi(context.i32_type(), "iftmp");
        phi.add_incoming(&[(&then_value, then_block), (&else_value, else_block)]);
        phi.as_any_value_enum()
    }
}

impl Codegen for MatchExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
    }
}

impl Codegen for MatchArmExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>>{
        panic!("MatchArmExprAST should not be codegened.")
    }
}

impl Codegen for SeqExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        let mut last_value = context.i32_type().const_int(0, false);
        for expression in self.expressions {
            last_value = expression.codegen(context, builder, scope).into_int_value();
        }
        last_value.as_any_value_enum()
    }
}

impl Codegen for AtomUnExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) ->Box<dyn AnyValue<'a>> {
        let expression = self.expression.codegen(context, builder, scope);
        match self.operator {
            AtomUnary::Not => builder.build_int_compare(inkwell::IntPredicate::EQ, expression.into_int_value(), context.i32_type().const_int(0, false), "nottmp").as_any_value_enum(),
        }
    }
}

impl Codegen for AtomBinExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        let left = self.left.codegen(context, builder, scope);
        let right = self.right.codegen(context, builder, scope);
        match self.operator {
            AtomBinary::Add => builder.build_int_add(left.into_int_value(), right.into_int_value(), "addtmp").as_any_value_enum(),
            AtomBinary::Sub => builder.build_int_sub(left.into_int_value(), right.into_int_value(), "subtmp").as_any_value_enum(),
            AtomBinary::Mul => builder.build_int_mul(left.into_int_value(), right.into_int_value(), "multmp").as_any_value_enum(),
            AtomBinary::Div => builder.build_int_unsigned_div(left.into_int_value(), right.into_int_value(), "divtmp").as_any_value_enum(),
            AtomBinary::Eq => builder.build_int_compare(inkwell::IntPredicate::EQ, left.into_int_value(), right.into_int_value(), "eqtmp").as_any_value_enum(),
            AtomBinary::Lt => builder.build_int_compare(inkwell::IntPredicate::ULT, left.into_int_value(), right.into_int_value(), "lttmp").as_any_value_enum(),
            AtomBinary::And => builder.build_and(left.into_int_value(), right.into_int_value(), "andtmp").as_any_value_enum(),
            AtomBinary::Or => builder.build_or(left.into_int_value(), right.into_int_value(), "ortmp").as_any_value_enum(),
        }
    }
}

impl Codegen for ExternExprAST {
    fn codegen<'a>(self, context: &'a Context, builder: &'a Builder<'a>, scope: &'a Scope<'a>) -> Box<dyn AnyValue<'a>> {
        panic!("ExternExprAST is not supported yet.")
        scope.add_variable(self.name.name, context.i32_type().fn_type(&[], false).as_any_type_enum());
    }
}


pub struct VariableExprAST {
    pub name: String,
}

pub struct IntegerExprAST {
    pub value: i32,
}

pub struct NoneExprAST {}

pub struct PairExprAST {
    pub left: Box<dyn Codegen>,
    pub right: Box<dyn Codegen>,
}

pub struct FunctionExprAST {
    pub parameters: Vec<VariableExprAST>,
    pub body: Box<dyn Codegen>,
}

pub struct ContExprAST {
    pub body: Box<dyn Codegen>,
}

pub struct DefineExprAST {
    pub name: VariableExprAST,                
    pub body: Box<dyn Codegen>,
}

pub struct CallExprAST {
    pub function: Box<dyn Codegen>,
    pub arguments: Vec<Box<dyn Codegen>>,
}

pub struct IfExprAST {
    pub predicate: Box<dyn Codegen>,
    pub then: Box<dyn Codegen>,
    pub else_: Box<dyn Codegen>,
}

pub struct MatchExprAST {
    pub expression: Box<dyn Codegen>,
    pub arms: Vec<MatchArmExprAST>,
}

pub struct MatchArmExprAST {
    pub patterns: Vec<Box<dyn Codegen>>,
    pub body: Box<dyn Codegen>,
}

pub struct SeqExprAST {
    pub expressions: Vec<Box<dyn Codegen>>,
}

pub struct AtomUnExprAST {
    pub operator: AtomUnary,
    pub expression: Box<dyn Codegen>,
}

pub struct AtomBinExprAST {
    pub operator: AtomBinary,
    pub left: Box<dyn Codegen>,
    pub right: Box<dyn Codegen>,
}

pub struct ExternExprAST {
    pub name: VariableExprAST,
}