#![allow(dead_code)]
use crate::ast;
use crate::ast::GetName;
use crate::codegen::Codegen;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    values::{FunctionValue, PointerValue},
};
use std::collections::HashMap;

//mod ink;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            fpm,
            variables: HashMap::new(),
            fn_value_opt: None,
        }
    }

    pub fn compile() {
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();

        // Create FPM
        let fpm = PassManager::create(&module);

        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();

        fpm.initialize();
        let _compiler = Compiler::new(&context, &module, &builder, &fpm);
    }

    fn get_basic_type(&self, ty: &ast::Type<'_>) -> BasicTypeEnum<'ctx> {
        match ty {
            ast::Type::Primitive(ty) => match ty {
                ast::PrimitiveTypes::I8 | ast::PrimitiveTypes::U8 => self.context.i8_type().into(),
                ast::PrimitiveTypes::I16 | ast::PrimitiveTypes::U16 => {
                    self.context.i16_type().into()
                }
                ast::PrimitiveTypes::I32 | ast::PrimitiveTypes::U32 => {
                    self.context.i32_type().into()
                }
                ast::PrimitiveTypes::I64 | ast::PrimitiveTypes::U64 => {
                    self.context.i64_type().into()
                }
                ast::PrimitiveTypes::F32 => self.context.f32_type().into(),
                ast::PrimitiveTypes::F64 => self.context.f64_type().into(),
                ast::PrimitiveTypes::Bool => self.context.bool_type().into(),
                ast::PrimitiveTypes::Char => self.context.i8_type().into(),
            },
            ast::Type::Struct(ty_struct) => {
                let struct_types = ty_struct
                    .types
                    .iter()
                    .map(|ty| self.get_basic_type(&ty.attr_type))
                    .collect::<Vec<BasicTypeEnum>>();
                self.context.struct_type(&struct_types[..], false).into()
            }
            ast::Type::Array(ty) => {
                let ty_array = self.get_basic_type(ty);
                // TODO: fix array length
                ty_array.array_type(10).into()
            }
        }
    }

    pub fn get_type(&self, ty: &ast::Type<'_>) -> BasicMetadataTypeEnum<'ctx> {
        match ty {
            ast::Type::Primitive(ty) => match ty {
                ast::PrimitiveTypes::I8 | ast::PrimitiveTypes::U8 => self.context.i8_type().into(),
                ast::PrimitiveTypes::I16 | ast::PrimitiveTypes::U16 => {
                    self.context.i16_type().into()
                }
                ast::PrimitiveTypes::I32 | ast::PrimitiveTypes::U32 => {
                    self.context.i32_type().into()
                }
                ast::PrimitiveTypes::I64 | ast::PrimitiveTypes::U64 => {
                    self.context.i64_type().into()
                }
                ast::PrimitiveTypes::F32 => self.context.f32_type().into(),
                ast::PrimitiveTypes::F64 => self.context.f64_type().into(),
                ast::PrimitiveTypes::Bool => self.context.bool_type().into(),
                ast::PrimitiveTypes::Char => self.context.i8_type().into(),
            },
            ast::Type::Struct(ty_struct) => {
                let struct_types = ty_struct
                    .types
                    .iter()
                    .map(|ty| self.get_basic_type(&ty.attr_type))
                    .collect::<Vec<BasicTypeEnum>>();
                self.context.struct_type(&struct_types[..], false).into()
            }
            ast::Type::Array(ty) => {
                let ty_array = self.get_basic_type(ty);
                // TODO: fix array length
                ty_array.array_type(10).into()
            }
        }
    }
}

impl<'a, 'ctx> Codegen for Compiler<'a, 'ctx> {
    fn function_declaration(&self, fn_decl: ast::FunctionStatement<'_>) {
        let param_types = fn_decl
            .parameters
            .iter()
            .map(|param| self.get_type(&param.parameter_type))
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let fn_type = self.context.f64_type().fn_type(&param_types, false);
        let _fn_val = self.module.add_function(&fn_decl.name(), fn_type, None);
    }

    fn expression(&self, _expression: ast::Expression) -> &Self {
        self
    }
}
