use crate::utils::SemanticTest;
use semantic_analyzer::ast;
use semantic_analyzer::types::semantic::SemanticStackContext;
use semantic_analyzer::types::{
    block_state::BlockState,
    error::StateErrorKind,
    expression::ExpressionResultValue,
    types::{PrimitiveTypes, Type},
    PrimitiveValue, Value,
};
use std::cell::RefCell;
use std::rc::Rc;

mod utils;

#[test]
fn expression_value_name_not_found() {
    let block_state = Rc::new(RefCell::new(BlockState::new(None)));
    let mut t = SemanticTest::new();
    let src = ast::Ident::new("x");
    let value_name = ast::ValueName::new(src);
    let expr = ast::Expression {
        expression_value: ast::ExpressionValue::ValueName(value_name),
        operation: None,
    };
    let res = t.state.expression(&expr, &block_state);
    assert!(res.is_none());
    assert!(t.check_error(StateErrorKind::ValueNotFound));
    let state = block_state.borrow().context.clone().get();
    assert!(state.is_empty());
}

#[test]
fn expression_value_name_exists() {
    let block_state = Rc::new(RefCell::new(BlockState::new(None)));
    let mut t = SemanticTest::new();
    let src = ast::Ident::new("x");
    let value_name = ast::ValueName::new(src);
    let expr = ast::Expression {
        expression_value: ast::ExpressionValue::ValueName(value_name.clone()),
        operation: None,
    };
    let ty = Type::Primitive(PrimitiveTypes::I8);
    let value = Value {
        inner_name: "x".into(),
        inner_type: ty.clone(),
        mutable: false,
        alloca: false,
        malloc: false,
    };
    block_state
        .borrow_mut()
        .values
        .insert(value_name.into(), value.clone());
    let res = t.state.expression(&expr, &block_state).unwrap();
    assert_eq!(res.expr_value, ExpressionResultValue::Register);
    assert_eq!(res.expr_type, ty);
    let state = block_state.borrow().context.clone().get();
    assert_eq!(state.len(), 1);
    assert_eq!(
        state[0],
        SemanticStackContext::ExpressionValue { expression: value }
    );
}

#[test]
fn expression_primitive_value() {
    let block_state = Rc::new(RefCell::new(BlockState::new(None)));
    let mut t = SemanticTest::new();
    let expr = ast::Expression {
        expression_value: ast::ExpressionValue::PrimitiveValue(ast::PrimitiveValue::I32(10)),
        operation: None,
    };
    let res = t.state.expression(&expr, &block_state).unwrap();
    assert_eq!(
        res.expr_value,
        ExpressionResultValue::PrimitiveValue(PrimitiveValue::I32(10))
    );
    let ty = Type::Primitive(PrimitiveTypes::I32);
    assert_eq!(res.expr_type, ty);
    let state = block_state.borrow().context.clone().get();
    assert!(state.is_empty());
}
