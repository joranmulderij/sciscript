use std::collections::{HashMap, HashSet};

use crate::{
    ast::{
        AssignmentType, Expr, ExprUnchecked, Line, LineUnchecked, Op, StructFieldKind,
        TypeAnnotationUnchecked,
    },
    types::{FunctionProfile, NumberConstant, Type, TypeContext, TypeProfile},
    units::{Unit, UnitSet},
};

pub fn check_types(
    ast: Vec<LineUnchecked>,
    type_context: &mut TypeContext,
) -> Result<(Vec<Line>, Type, HashSet<String>), String> {
    let mut new_ast: Vec<Line> = Vec::new();
    let mut return_type: Type = Type::Void;
    let mut deps: HashSet<String> = HashSet::new();
    for line in ast {
        return_type = match line {
            LineUnchecked::Expr(expr) => {
                let (expr, type_, dep) = check_expr_types(expr, type_context)?;
                new_ast.push(Line::Expr(expr));
                deps.extend(dep);
                type_
            }
            LineUnchecked::Assign(var, type_annotation, expr, assignment_type) => {
                let (expr, type2, dep) = check_expr_types(expr, type_context)?;
                deps.extend(dep);
                let id: String = match assignment_type {
                    AssignmentType::Normal => {
                        if let Some((id, type1, const_)) = type_context.get_variable(&var) {
                            if !type1.can_be_assigned_to(&type2) {
                                println!("{:?} {:?}", type1, type2);
                                return Err("Type mismatch in assignment".to_string());
                            }
                            if *const_ {
                                return Err("Cannot reassign const variable".to_string());
                            }
                            id.clone()
                        } else {
                            let id = type_context.insert_variable(var, None, type2.clone(), false);
                            id
                        }
                    }
                    AssignmentType::Const | AssignmentType::Let => {
                        let const_ = matches!(assignment_type, AssignmentType::Const);
                        let type_ = if let Some(type_annotation) = type_annotation {
                            println!("{:?}", type_annotation);
                            let type_ = check_type_annotation_types(type_annotation, type_context)?;
                            if !type_.can_be_assigned_to(&type2) {
                                println!("{:?} --- {:?}", type_, type2);
                                return Err("Type mismatch in assignment".to_string());
                            }
                            type_
                        } else {
                            type2.clone()
                        };
                        let id = type_context.insert_variable(var, None, type_, const_);
                        id
                    }
                };
                new_ast.push(Line::Assign(id, expr, assignment_type));
                type2
            }
            LineUnchecked::UnitDef(name) => {
                let unit = Unit { name: name.clone() };
                let type_ =
                    Type::Number(UnitSet::single_unit(unit), Some(NumberConstant::Integer(1)));
                type_context.insert_variable(name, None, type_.clone(), true);
                type_
            }
        }
    }
    Ok((new_ast, return_type, deps))
}

fn check_expr_types(
    expr: ExprUnchecked,
    mut type_context: &mut TypeContext,
) -> Result<(Expr, Type, HashSet<String>), String> {
    match expr {
        ExprUnchecked::Number(num) => Ok((
            Expr::Number(num.clone()),
            Type::Number(UnitSet::empty(), Some(num)),
            HashSet::new(),
        )),
        ExprUnchecked::Null => Ok((Expr::Null, Type::Void, HashSet::new())),
        ExprUnchecked::Boolean(b) => Ok((Expr::Boolean(b), Type::Bool, HashSet::new())),
        ExprUnchecked::UnaryMinus(expr) => {
            let (expr, type_, dep) = check_expr_types(*expr, type_context)?;
            match type_ {
                Type::Number(unit, number_constant) => {
                    let number_constant = match number_constant {
                        Some(NumberConstant::Integer(i)) => Some(NumberConstant::Integer(-i)),
                        _ => None,
                    };
                    Ok((
                        Expr::UnaryMinus(Box::new(expr)),
                        Type::Number(unit.clone(), number_constant),
                        dep,
                    ))
                }
                _ => Err("Type mismatch in unary minus".to_string()),
            }
        }
        ExprUnchecked::Sequencial(lhs, rhs) => {
            let (expr1, type1, dep1) = check_expr_types(*lhs, type_context)?;
            let (expr2, type2, dep2) = check_expr_types(*rhs, type_context)?;
            println!("{:?} {:?}", type1, type2);
            match (type1, type2) {
                (type1, type2)
                    if matches!(type1, Type::Number(_, _))
                        && matches!(type2, Type::Number(_, _)) =>
                {
                    let pack1 = (expr1, type1, dep1);
                    let pack2 = (expr2, type2, dep2);
                    handle_bin_op(pack1, Op::Multiply, pack2)
                }
                (
                    Type::Function(FunctionProfile {
                        parameters,
                        return_type: ret,
                    }),
                    param,
                ) => {
                    let name_mappings = arguments_match_parameters(
                        vec![(param.clone(), expr2)],
                        HashMap::new(),
                        &parameters,
                    )?;
                    let mut deps = dep1;
                    deps.extend(dep2);
                    let expr = Expr::FunctionCall(Box::new(expr1), name_mappings);
                    Ok((expr, *ret.clone(), deps))
                }
                _ => Err("Type mismatch in sequencial expression".to_string()),
            }
        }
        ExprUnchecked::BinOp { lhs, op, rhs } => {
            let left = check_expr_types(*lhs, type_context)?;
            let right = check_expr_types(*rhs, type_context)?;
            handle_bin_op(left, op, right)
        }
        ExprUnchecked::Variable(var) => {
            if let Some((id, type_, _)) = type_context.get_variable(&var) {
                let mut deps = HashSet::new();
                if !matches!(type_, Type::Function { .. }) {
                    deps.insert(id.clone());
                }

                Ok((Expr::Variable(id.clone()), type_.clone(), deps))
            } else {
                Err(format!("Variable {} not found in scope", var))
            }
        }
        ExprUnchecked::If(conditions, blocks, else_block) => {
            let mut new_conditions: Vec<Expr> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            for condition in conditions {
                let (expr, type_, dep) = check_expr_types(condition, &mut type_context)?;
                deps.extend(dep);
                if type_.can_be_assigned_to(&Type::Bool) {
                    return Err("Type mismatch in if condition".to_string());
                }
                new_conditions.push(expr);
            }
            let mut new_blocks: Vec<Vec<Line>> = Vec::new();
            let mut return_type: Option<Type> = None;
            for block in blocks {
                let (new_block, type_, dep) = check_types(block, type_context)?;
                deps.extend(dep);
                return_type = match return_type {
                    Some(return_type) if return_type.can_be_assigned_to(&type_) => Some(Type::Void),
                    _ => Some(type_),
                };
                new_blocks.push(new_block);
            }
            let new_else_block = match else_block {
                Some(else_block) => {
                    let (new_else_block, type_, dep) = check_types(else_block, type_context)?;
                    deps.extend(dep);
                    return_type = match return_type {
                        Some(return_type) if return_type.can_be_assigned_to(&type_) => {
                            Some(Type::Void)
                        }
                        _ => Some(type_),
                    };
                    Some(new_else_block)
                }
                None => None,
            };
            let return_type = return_type.unwrap_or(Type::Void);
            let expr = Expr::If(new_conditions, new_blocks, new_else_block);
            Ok((expr, return_type, deps))
        }
        ExprUnchecked::For(var, expr, block) => {
            let (expr, type_, dep1) = check_expr_types(*expr, &mut type_context)?;
            let i_type = match type_ {
                Type::Range => Type::Number(UnitSet::empty(), None),
                _ => return Err("Type mismatch in for loop".to_string()),
            };
            type_context.push_scope();
            let id = type_context.insert_variable(var.clone(), None, i_type, true);
            let (block, type_, dep2) = check_types(block, type_context)?;
            let variables = type_context.pop_scope();
            let info = Expr::For(id, Box::new(expr), block);
            let mut deps = dep1;
            deps.extend(dep2);
            deps.retain(|x| !variables.contains(x));
            Ok((info, type_, deps))
        }
        ExprUnchecked::Block(lines) => {
            type_context.push_scope();
            let (new_lines, type_, mut deps) = check_types(lines, &mut type_context)?;
            let variables = type_context.pop_scope();
            deps.retain(|x| !variables.contains(x));
            let expr = Expr::Block(new_lines);
            Ok((expr, type_, deps))
        }
        ExprUnchecked::Lambda(parameters, block, type_annotation) => {
            let return_type = match type_annotation {
                Some(type_annotation) => Some(check_type_annotation_types(
                    type_annotation,
                    &mut type_context,
                )?),
                None => None,
            };
            type_context.push_scope();
            let mut deps: HashSet<String> = HashSet::new();
            let parameters: Vec<(String, Type, Option<Expr>)> = parameters
                .into_iter()
                .map(|(name, type_, default_value)| {
                    let type_ = check_type_annotation_types(type_, &mut type_context)?;
                    let id = type_context.insert_variable(name.clone(), None, type_.clone(), false);
                    let default_value = match default_value {
                        Some(default_value) => {
                            let (expr, default_value_type, dep) =
                                check_expr_types(default_value, &mut type_context)?;
                            deps.extend(dep);
                            if !type_.can_be_assigned_to(&default_value_type) {
                                return Err("Type mismatch in lambda parameter".to_string());
                            }
                            Some(expr)
                        }
                        None => None,
                    };
                    Ok((id, type_, default_value))
                })
                .collect::<Result<Vec<_>, String>>()?;
            let (block, return_type2, dep2) = check_expr_types(*block, &mut type_context)?;
            let return_type = if let Some(return_type) = return_type {
                if !return_type.can_be_assigned_to(&return_type2) {
                    return Err("Type mismatch in lambda return type".to_string());
                }
                return_type
            } else {
                return_type2
            };
            deps.extend(dep2);
            type_context.pop_scope();
            let type_ = Type::Function(FunctionProfile {
                parameters: parameters
                    .iter()
                    .map(|(name, type_, default)| (name.clone(), type_.clone(), default.is_some()))
                    .collect(),
                return_type: Box::new(return_type),
            });
            let deps: HashSet<String> = deps
                .iter()
                .filter(|x| !parameters.iter().any(|(name, _, _)| *x == name))
                .map(|x| x.clone())
                .collect();
            let expr = Expr::Lambda(
                parameters
                    .into_iter()
                    .map(|(name, _, default)| (name.clone(), default))
                    .collect(),
                Box::new(block),
                deps.clone(),
            );
            Ok((expr, type_, deps))
        }
        ExprUnchecked::GetProperty(_expr, _property) => {
            let (expr, type_, dep) = check_expr_types(*_expr, &mut type_context)?;
            let type_ = match type_ {
                Type::Struct(fields) => {
                    let field = fields.iter().find(|(name, _, _)| name == &_property);
                    if let Some((_name, type_, _)) = field {
                        type_.clone()
                    } else {
                        return Err("Property not found in struct".to_string());
                    }
                }
                _ => return Err("Type mismatch in get property".to_string()),
            };
            Ok((Expr::GetProperty(Box::new(expr), _property), type_, dep))
        }
        ExprUnchecked::List(items) => {
            let mut new_items: Vec<Expr> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            let mut item_type: Option<Type> = None;
            for item in items {
                let (expr, type_, dep) = check_expr_types(item, &mut type_context)?;
                deps.extend(dep);
                item_type = match item_type {
                    Some(item_type) if !item_type.can_be_assigned_to(&type_) => Some(Type::Any),
                    _ => Some(type_),
                };
                new_items.push(expr);
            }
            let type_ = item_type.unwrap_or(Type::Void);
            Ok((Expr::List(new_items), Type::List(Box::new(type_)), deps))
        }
        ExprUnchecked::Map(items) => {
            let mut new_items: Vec<(Expr, Expr)> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            let mut key_type: Option<Type> = None;
            let mut value_type: Option<Type> = None;
            for (key, value) in items {
                let (key, key_type_, dep1) = check_expr_types(key, &mut type_context)?;
                let (value, value_type_, dep2) = check_expr_types(value, &mut type_context)?;
                deps.extend(dep1);
                deps.extend(dep2);
                key_type = match key_type {
                    Some(key_type) if !key_type.can_be_assigned_to(&key_type_) => Some(Type::Any),
                    _ => Some(key_type_),
                };
                value_type = match value_type {
                    Some(value_type) if !value_type.can_be_assigned_to(&value_type_) => {
                        Some(Type::Any)
                    }
                    _ => Some(value_type_),
                };
                new_items.push((key, value));
            }
            let key_type = key_type.unwrap_or(Type::Void);
            let value_type = value_type.unwrap_or(Type::Void);
            let type_ = Type::Map(Box::new(key_type), Box::new(value_type));
            Ok((Expr::Map(new_items), type_, deps))
        }
        ExprUnchecked::Index(expr, index) => {
            let (expr, type_expr, dep_expr) = check_expr_types(*expr, &mut type_context)?;
            let (index, type_index, dep_index) = check_expr_types(*index, &mut type_context)?;
            println!("{:?} {:?}", type_expr, type_index);
            let type_ = match (type_expr, type_index) {
                (Type::List(type_), type_index) => {
                    if !Type::Number(UnitSet::empty(), None).can_be_assigned_to(&type_index) {
                        return Err("Type mismatch in index".to_string());
                    }
                    *type_
                }
                (Type::Type(type_profile, constructor), type_index) => match type_profile {
                    TypeProfile::Function(fun) => Type::Type(
                        TypeProfile::Type(Box::new(fun(vec![type_index])?)),
                        constructor,
                    ),
                    TypeProfile::Type(_) => return Err("Type mismatch in index".to_string()),
                },
                (Type::Map(key_type, value_type), type_index) => {
                    if !key_type.can_be_assigned_to(&type_index) {
                        return Err("Type mismatch in index".to_string());
                    }
                    *value_type
                }
                _ => return Err("Type mismatch in index".to_string()),
            };
            let mut deps = dep_expr;
            deps.extend(dep_index);
            Ok((Expr::Index(Box::new(expr), Box::new(index)), type_, deps))
        }
        ExprUnchecked::FunctionCall(function, positional_arguments, named_arguments) => {
            let mut deps: HashSet<String> = HashSet::new();
            let (function, type_, dep1) = check_expr_types(*function, &mut type_context)?;
            deps.extend(dep1);
            match type_ {
                Type::Function(FunctionProfile {
                    parameters,
                    return_type: ret,
                })
                | Type::Type(
                    _,
                    Some(FunctionProfile {
                        parameters,
                        return_type: ret,
                    }),
                ) => {
                    println!(
                        "{:?} {:?} {:?}",
                        parameters, positional_arguments, named_arguments
                    );
                    let positional_arguments = positional_arguments
                        .into_iter()
                        .map(|arg| {
                            let (expr, type_, dep) = check_expr_types(arg, &mut type_context)?;
                            deps.extend(dep);
                            Ok((type_, expr))
                        })
                        .collect::<Result<_, String>>()?;
                    let mut new_named_arguments: HashMap<String, (Type, Expr)> = HashMap::new();
                    for (name, arg) in named_arguments {
                        let (expr, type_, dep) = check_expr_types(arg, &mut type_context)?;
                        deps.extend(dep);
                        new_named_arguments.insert(name, (type_, expr));
                    }
                    let name_mappings = arguments_match_parameters(
                        positional_arguments,
                        new_named_arguments,
                        &parameters,
                    )?;

                    Ok((
                        Expr::FunctionCall(Box::new(function), name_mappings),
                        *ret,
                        deps,
                    ))
                }
                _ => Err("Type mismatch in function call".to_string()),
            }
        }
        ExprUnchecked::Struct(fields) => {
            let mut new_fields: Vec<(String, Option<Expr>, StructFieldKind)> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            let mut parameters: Vec<(String, Type, bool)> = Vec::new();
            for (name, type_annotation, default_value, field_kind) in fields {
                let mut type_ = None;

                if let Some(type_annotation) = type_annotation {
                    type_ = Some(check_type_annotation_types(
                        type_annotation,
                        &mut type_context,
                    )?)
                }
                let default_value = match default_value {
                    Some(default_value) => {
                        type_context.push_scope();
                        for (name, type_, _) in &parameters {
                            let id = format!("self.{}", name);
                            type_context.insert_variable(
                                name.clone(),
                                Some(id),
                                type_.clone(),
                                false,
                            );
                        }
                        let (expr, expr_type, dep) =
                            check_expr_types(default_value, &mut type_context)?;
                        deps.extend(dep);
                        let dep = type_context.pop_scope();
                        deps.extend(dep);
                        if let Some(type_) = &type_ {
                            if !type_.can_be_assigned_to(&expr_type) {
                                return Err("Type mismatch in struct field".to_string());
                            }
                        } else {
                            type_ = Some(expr_type);
                        }
                        Some(expr)
                    }
                    None => None,
                };
                parameters.push((
                    name.clone(),
                    type_.unwrap_or(Type::Any),
                    default_value.is_none(),
                ));
                new_fields.push((name, default_value, field_kind));
            }
            let struct_type = Type::Struct(parameters.clone());
            Ok((
                Expr::Struct(new_fields),
                Type::Type(
                    TypeProfile::Type(Box::new(struct_type.clone())),
                    Some(FunctionProfile {
                        parameters,
                        return_type: Box::new(struct_type),
                    }),
                ),
                deps,
            ))
        }
        ExprUnchecked::Matrix(matrix) => {
            let mut new_matrix: Vec<Vec<Expr>> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            let mut row_length: Option<usize> = None;
            let mut unitset: Option<UnitSet> = None;
            for row in matrix {
                let mut new_row: Vec<Expr> = Vec::new();
                for item in row {
                    let (expr, type_, dep) = check_expr_types(item, &mut type_context)?;
                    deps.extend(dep);
                    if let Type::Number(unit, _) = type_ {
                        unitset = match unitset {
                            Some(unit_set) => {
                                if unit_set != unit {
                                    return Err("Unit mismatch in matrix".to_string());
                                }
                                Some(unit_set)
                            }
                            None => Some(unit),
                        };
                    } else {
                        return Err("Type mismatch in matrix".to_string());
                    }
                    new_row.push(expr);
                }
                row_length = match row_length {
                    Some(row_length) if row_length != new_row.len() => {
                        return Err("Row length mismatch in matrix".to_string());
                    }
                    _ => Some(new_row.len()),
                };
                new_matrix.push(new_row);
            }
            let row_length = row_length.unwrap_or(0);
            let type_ = Type::Matrix(row_length, new_matrix.len(), unitset);
            Ok((Expr::Matrix(new_matrix), type_, deps))
        }
    }
}

fn check_type_annotation_types(
    type_annotation: TypeAnnotationUnchecked,
    mut type_context: &mut TypeContext,
) -> Result<Type, String> {
    println!("{:?}", type_context.get_variable(&type_annotation.name));
    let type_profile = if let Some((_, Type::Type(fun, _), _)) =
        type_context.get_variable(&type_annotation.name)
    {
        fun.clone()
    } else {
        return Err(format!("Type {} not found in scope", type_annotation.name));
    };
    type_context.get_variable("f");
    let args = type_annotation
        .generics
        .into_iter()
        .map(|type_annotation| check_expr_types(type_annotation, &mut type_context))
        .collect::<Result<Vec<(Expr, Type, HashSet<String>)>, String>>()?;
    let args = args
        .into_iter()
        .map(|(_, type_, _)| type_)
        .collect::<Vec<Type>>();
    match type_profile {
        TypeProfile::Function(fun) => fun(args),
        TypeProfile::Type(type_) => {
            if !args.is_empty() {
                return Err("Type takes no arguments".to_string());
            }
            Ok(*type_)
        }
    }
}

fn handle_bin_op(
    left: (Expr, Type, HashSet<String>),
    op: Op,
    right: (Expr, Type, HashSet<String>),
) -> Result<(Expr, Type, HashSet<String>), String> {
    let (expr1, type1, dep1) = left;
    let (expr2, type2, dep2) = right;
    let (expr, type_) = match (type1, &op, type2) {
        (Type::Number(unit1, _), Op::Range, Type::Number(unit2, _)) => {
            if !unit1.is_empty() || !unit2.is_empty() {
                return Err("Unit mismatch in range operation".to_string());
            }
            (
                Expr::BinOp {
                    lhs: Box::new(expr1),
                    op: Op::Range,
                    rhs: Box::new(expr2),
                },
                Type::Range,
            )
        }
        (type1, Op::Equals | Op::NotEquals, type2) => {
            if type1 != type2 {
                return Err("Type mismatch in comparison operation".to_string());
            }
            (
                Expr::BinOp {
                    lhs: Box::new(expr1),
                    op,
                    rhs: Box::new(expr2),
                },
                Type::Bool,
            )
        }
        (
            Type::Number(unit1, c1),
            Op::Multiply | Op::Divide | Op::Add | Op::Subtract | Op::Modulo | Op::Power,
            Type::Number(unit2, c2),
        ) => {
            let const_ = match (c1, c2.clone()) {
                (Some(const1), Some(const2)) => match op {
                    Op::Multiply => Some(const1 * const2),
                    Op::Divide => Some(const1 / const2),
                    Op::Add => Some(const1 + const2),
                    Op::Subtract => Some(const1 - const2),
                    Op::Modulo => Some(const1 % const2),
                    Op::Power => Some(const1.pow(&const2)),
                    _ => unreachable!(),
                },
                // TODO: Handle floats
                _ => None,
            };
            let unit = match op {
                Op::Multiply => unit1 + unit2,
                Op::Divide => unit1 - unit2,
                Op::Add | Op::Subtract | Op::Modulo => {
                    if unit1 != unit2 {
                        return Err("Unit mismatch in binary operation".to_string());
                    } else {
                        unit1
                    }
                }
                Op::Power => {
                    if !unit2.is_empty() {
                        return Err("Unit mismatch in power operation".to_string());
                    }
                    let unit = if unit1.is_empty() {
                        UnitSet::empty()
                    } else if let Some(NumberConstant::Integer(i)) = c2 {
                        unit1 * i
                    } else {
                        return Err("Unit mismatch in power operation".to_string());
                    };
                    unit
                }
                _ => unreachable!(),
            };
            let expr = match const_.clone() {
                Some(const_) => Expr::Number(const_),
                _ => Expr::BinOp {
                    lhs: Box::new(expr1),
                    op,
                    rhs: Box::new(expr2),
                },
            };
            (expr, Type::Number(unit, const_))
        }
        (Type::Matrix(rows1, cols1, unit1), Op::Multiply, Type::Matrix(rows2, cols2, unit2)) => {
            if cols1 == 1 && cols2 == 1 {
                // dot product
                if rows1 != rows2 {
                    return Err("Matrix dimensions mismatch in multiplication".to_string());
                }
                let unit = match (unit1, unit2) {
                    (Some(unit1), Some(unit2)) => Some(unit1 + unit2),
                    _ => None,
                };
                (
                    Expr::BinOp {
                        lhs: Box::new(expr1),
                        op,
                        rhs: Box::new(expr2),
                    },
                    Type::Number(unit.unwrap_or(UnitSet::empty()), None),
                )
            } else {
                todo!()
            }
        }
        _ => return Err("Type mismatch in binary operation".to_string()),
    };
    let mut deps = dep1;
    deps.extend(dep2);
    Ok((expr, type_, deps))
}

pub fn arguments_match_parameters(
    mut positional_arguments: Vec<(Type, Expr)>,
    mut named_arguments: HashMap<String, (Type, Expr)>,
    parameters: &Vec<(String, Type, bool)>,
) -> Result<Vec<(String, Expr)>, String> {
    let mut name_mapping: Vec<(String, Expr)> = Vec::new();
    for (name, type_, required) in parameters.iter() {
        if !positional_arguments.is_empty() {
            let (arg_type, expr) = positional_arguments.remove(0);
            if !type_.can_be_assigned_to(&arg_type) {
                println!("arg: {:?} type: {:?}", arg_type, type_);
                return Err("Type mismatch in function call".to_string());
            } else {
                name_mapping.push((name.clone(), expr));
            }
        } else if let Some((arg_type, expr)) = named_arguments.remove(name) {
            if !type_.can_be_assigned_to(&arg_type) {
                return Err("Type mismatch in function call".to_string());
            } else {
                name_mapping.push((name.clone(), expr));
            }
        } else {
            if *required {
                return Err("Missing required argument".to_string());
            }
        }
    }
    if named_arguments.is_empty() && positional_arguments.is_empty() {
        Ok(name_mapping)
    } else {
        Err("Extra arguments in function call".to_string())
    }
}
