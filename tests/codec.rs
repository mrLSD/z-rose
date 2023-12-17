use semantic_analyzer::ast::{self, Ident};

#[test]
fn basic_ast_serialize() {
    // Import
    let imports: ast::ImportPath = vec![ast::ImportName::new(Ident::new("import1"))];
    let import_stm = ast::MainStatement::Import(imports);

    let constant1 = ast::Constant {
        name: ast::ConstantName::new(Ident::new("const1")),
        constant_type: ast::Type::Primitive(ast::PrimitiveTypes::None),
        constant_value: ast::ConstantExpression {
            value: ast::ConstantValue::Constant(ast::ConstantName::new(Ident::new("const2"))),
            operation: None,
        },
    };
    let constant_stm = ast::MainStatement::Constant(constant1.clone());

    let ty = ast::StructTypes {
        name: Ident::new("StructType"),
        attributes: vec![ast::StructType {
            attr_name: Ident::new("y"),
            attr_type: ast::Type::Primitive(ast::PrimitiveTypes::U64),
        }],
    };
    let ty_stm = ast::MainStatement::Types(ty.clone());

    let let_binding = ast::LetBinding {
        name: ast::ValueName::new(Ident::new("x")),
        mutable: true,
        value_type: None,
        value: Box::new(ast::Expression {
            expression_value: ast::ExpressionValue::PrimitiveValue(ast::PrimitiveValue::Bool(
                false,
            )),
            operation: None,
        }),
    };
    let body_let_binding = ast::BodyStatement::LetBinding(let_binding.clone());
    let body_binding = ast::BodyStatement::Binding(ast::Binding {
        name: ast::ValueName::new(Ident::new("x")),
        value: Box::new(ast::Expression {
            expression_value: ast::ExpressionValue::PrimitiveValue(ast::PrimitiveValue::Bool(true)),
            operation: None,
        }),
    });
    let body_fn_call = ast::BodyStatement::FunctionCall(ast::FunctionCall {
        name: ast::FunctionName::new(Ident::new("fn2")),
        parameters: vec![],
    });
    let body_if = ast::BodyStatement::If(ast::IfStatement {
        condition: ast::IfCondition::Single(ast::Expression {
            expression_value: ast::ExpressionValue::PrimitiveValue(ast::PrimitiveValue::Bool(true)),
            operation: Some((
                ast::ExpressionOperations::And,
                Box::new(ast::Expression {
                    expression_value: ast::ExpressionValue::PrimitiveValue(
                        ast::PrimitiveValue::Bool(true),
                    ),
                    operation: None,
                }),
            )),
        }),
        body: ast::IfBodyStatements::If(vec![ast::IfBodyStatement::FunctionCall(
            ast::FunctionCall {
                name: ast::FunctionName::new(Ident::new("fn2")),
                parameters: vec![],
            },
        )]),
        else_statement: None,
        else_if_statement: None,
    });
    let body_loop = ast::BodyStatement::Loop(vec![
        ast::LoopBodyStatement::If(ast::IfStatement {
            condition: ast::IfCondition::Logic(ast::ExpressionLogicCondition {
                left: ast::ExpressionCondition {
                    left: ast::Expression {
                        expression_value: ast::ExpressionValue::PrimitiveValue(
                            ast::PrimitiveValue::U32(10),
                        ),
                        operation: None,
                    },
                    condition: ast::Condition::GreatEq,
                    right: ast::Expression {
                        expression_value: ast::ExpressionValue::PrimitiveValue(
                            ast::PrimitiveValue::U32(20),
                        ),
                        operation: None,
                    },
                },
                right: Some((
                    ast::LogicCondition::Or,
                    Box::new(ast::ExpressionLogicCondition {
                        left: ast::ExpressionCondition {
                            left: ast::Expression {
                                expression_value: ast::ExpressionValue::PrimitiveValue(
                                    ast::PrimitiveValue::U32(30),
                                ),
                                operation: None,
                            },
                            condition: ast::Condition::Less,
                            right: ast::Expression {
                                expression_value: ast::ExpressionValue::PrimitiveValue(
                                    ast::PrimitiveValue::U32(40),
                                ),
                                operation: None,
                            },
                        },
                        right: None,
                    }),
                )),
            }),
            else_statement: None,
            else_if_statement: None,
            body: ast::IfBodyStatements::Loop(vec![
                ast::IfLoopBodyStatement::LetBinding(let_binding.clone()),
                ast::IfLoopBodyStatement::Break,
            ]),
        }),
        ast::LoopBodyStatement::FunctionCall(ast::FunctionCall {
            name: ast::FunctionName::new(Ident::new("fn2")),
            parameters: vec![],
        }),
    ]);
    let body_return = ast::BodyStatement::Return(ast::Expression {
        expression_value: ast::ExpressionValue::PrimitiveValue(ast::PrimitiveValue::Bool(true)),
        operation: None,
    });
    let fn1 = ast::FunctionStatement {
        name: ast::FunctionName::new(Ident::new("fn1")),
        parameters: vec![],
        result_type: ast::Type::Primitive(ast::PrimitiveTypes::Bool),
        body: vec![
            body_let_binding.clone(),
            body_binding,
            body_fn_call,
            body_if,
            body_loop,
            body_return.clone(),
        ],
    };
    let fn1_stm = ast::MainStatement::Function(fn1);

    let body_expr_return = ast::BodyStatement::Expression(ast::Expression {
        expression_value: ast::ExpressionValue::PrimitiveValue(ast::PrimitiveValue::U32(23)),
        operation: None,
    });
    let fn2 = ast::FunctionStatement {
        name: ast::FunctionName::new(Ident::new("fn2")),
        parameters: vec![ast::FunctionParameter {
            name: ast::ParameterName::new(Ident::new("x")),
            parameter_type: ast::Type::Primitive(ast::PrimitiveTypes::U32),
        }],
        result_type: ast::Type::Primitive(ast::PrimitiveTypes::U32),
        body: vec![body_expr_return],
    };
    let fn2_stm = ast::MainStatement::Function(fn2.clone());

    let main_stm: ast::Main = vec![import_stm, constant_stm, ty_stm, fn1_stm, fn2_stm];
    let json = serde_json::to_string(&main_stm).unwrap();
    // println!("{json}");
    // assert_eq!(json, JSON_AST);
    let ser_ast: ast::Main = serde_json::from_str(&json).unwrap();
    assert_eq!(main_stm, ser_ast);
}

#[test]
fn ident_serialize() {
    let id = Ident::new("x");
    let id_json = serde_json::to_string(&id).unwrap();
    assert_eq!(
        id_json,
        r#"{"offset":0,"line":1,"fragment":"x","extra":null}"#
    );
    let new_id: Ident = serde_json::from_str(&id_json).unwrap();
    assert_eq!(id, new_id);
}

#[test]
fn ast_extended_serde_check() {
    // It covers uncovered serde parts
    let pv = ast::PrimitiveValue::None;
    let to_json = serde_json::to_string(&pv).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(pv, to_val);

    let pv = ast::PrimitiveValue::Ptr;
    let to_json = serde_json::to_string(&pv).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(pv, to_val);

    let pv = ast::PrimitiveValue::F32(1.2);
    let to_json = serde_json::to_string(&pv).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(pv, to_val);

    let pv = ast::PrimitiveValue::String("test".to_string());
    let to_json = serde_json::to_string(&pv).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(pv, to_val);

    let cl = ast::CodeLocation::new(10, 20);
    let to_json = serde_json::to_string(&cl).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(cl, to_val);

    let est = ast::ExpressionStructValue {
        name: ast::ValueName::new(Ident::new("x")),
        attribute: ast::ValueName::new(Ident::new("y")),
    };
    let to_json = serde_json::to_string(&est).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(est, to_val);

    let lbs = ast::LoopBodyStatement::Continue;
    let to_json = serde_json::to_string(&lbs).unwrap();
    let to_val = serde_json::from_str(&to_json).unwrap();
    assert_eq!(lbs, to_val);
}
