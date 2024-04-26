use std::collections::HashSet;

use crate::{
    ast::{AssignmentType, Expr, ExprUnchecked, Line, LineUnchecked, Op, TypeAnnotationUnchecked},
    types::{NumberConstant, Type, TypeContext},
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
                                println!("{:?} dflkdjf {:?}", type1, type2);
                                return Err("Type mismatch in assignment".to_string());
                            }
                            if *const_ {
                                return Err("Cannot reassign const variable".to_string());
                            }
                            id.clone()
                        } else {
                            let id = type_context.insert_variable(var, type2.clone(), false);
                            id
                        }
                    }
                    AssignmentType::Const | AssignmentType::Let => {
                        let const_ = matches!(assignment_type, AssignmentType::Const);
                        let type_ = if let Some(type_annotation) = type_annotation {
                            let type_ = check_type_annotation_types(type_annotation, type_context)?;
                            if !type_.can_be_assigned_to(&type2) {
                                return Err("Type mismatch in assignment".to_string());
                            }
                            type_
                        } else {
                            type2.clone()
                        };
                        let id = type_context.insert_variable(var, type_, const_);
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
                type_context.insert_variable(name, type_.clone(), true);
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
            match (type1, type2) {
                (type1, type2)
                    if matches!(type1, Type::Number(_, _))
                        && matches!(type2, Type::Number(_, _)) =>
                {
                    let pack1 = (expr1, type1, dep1);
                    let pack2 = (expr2, type2, dep2);
                    handle_bin_op(pack1, Op::Multiply, pack2)
                }
                (Type::Function(arguments, returned, has_more_args), param) => {
                    if !Type::arguments_match_parameters(&arguments, &has_more_args, &vec![param]) {
                        return Err("Type mismatch in sequencial expression".to_string());
                    }
                    let mut deps = dep1;
                    deps.extend(dep2);
                    let expr = Expr::FunctionCall(Box::new(expr1), vec![expr2]);
                    Ok((expr, *returned.clone(), deps))
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
            let id = type_context.insert_variable(var.clone(), i_type, true);
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
        ExprUnchecked::Lambda(parameters, block) => {
            let mut param_types: Vec<Type> = Vec::new();
            let mut param_names: Vec<String> = Vec::new();
            let last_parameter_name = match &parameters.last() {
                Some((name, _)) => Some(name.clone()),
                None => None,
            };
            type_context.push_scope();
            for (name, type_) in parameters {
                let type_ = check_type_annotation_types(type_, type_context)?;
                let name = type_context.insert_variable(name, type_.clone(), true);
                param_types.push(type_);
                param_names.push(name);
            }

            let has_last_args = if last_parameter_name == Some("args".to_string())
                && param_types.last() == Some(&Type::List(Box::new(Type::Any)))
            {
                param_types.pop();
                true
            } else {
                false
            };
            let (block, return_type, deps) = check_expr_types(*block, &mut type_context)?;
            type_context.pop_scope();
            let type_ = Type::Function(param_types, Box::new(return_type), has_last_args);
            let deps: HashSet<String> = deps
                .iter()
                .filter(|x| !param_names.contains(x))
                .map(|x| x.clone())
                .collect();
            let expr = Expr::Lambda(param_names, Box::new(block), deps.clone(), has_last_args);
            Ok((expr, type_, deps))
        }
        ExprUnchecked::GetProperty(_expr, _property) => {
            // let (expr, type_, dep) = check_expr_types(*expr, &mut type_context)?;
            // let type_ = match type_ {
            //     Type::Number(_, _) => Type::Number(UnitSet::empty(), None),
            //     _ => return Err("Type mismatch in get property".to_string()),
            // };
            // Ok((Expr::GetProperty(Box::new(expr), property), type_, dep))
            todo!()
        }
        ExprUnchecked::List(items) => {
            let mut new_items: Vec<Expr> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            let mut item_type: Option<Type> = None;
            for item in items {
                let (expr, type_, dep) = check_expr_types(item, &mut type_context)?;
                deps.extend(dep);
                item_type = match item_type {
                    Some(item_type) if item_type.can_be_assigned_to(&type_) => Some(Type::Any),
                    _ => Some(type_),
                };
                new_items.push(expr);
            }
            let type_ = item_type.unwrap_or(Type::Void);
            Ok((Expr::List(new_items), Type::List(Box::new(type_)), deps))
        }
        ExprUnchecked::Index(expr, index) => {
            let (expr, type_expr, dep_expr) = check_expr_types(*expr, &mut type_context)?;
            let (index, type_index, dep_index) = check_expr_types(*index, &mut type_context)?;
            println!("{:?} {:?}", type_expr, type_index);
            let type_ = match type_expr {
                Type::List(type_) => {
                    if !Type::Number(UnitSet::empty(), None).can_be_assigned_to(&type_index) {
                        return Err("Type mismatch in index".to_string());
                    }
                    *type_
                }
                _ => return Err("Type mismatch in index".to_string()),
            };
            let mut deps = dep_expr;
            deps.extend(dep_index);
            Ok((Expr::Index(Box::new(expr), Box::new(index)), type_, deps))
        }
        ExprUnchecked::FunctionCall(function, mut arguments) => {
            let mut deps: HashSet<String> = HashSet::new();
            let (function, type_, dep1) = check_expr_types(*function, &mut type_context)?;
            deps.extend(dep1);
            if let Type::Function(parameters, returned, has_more_args) = type_ {
                let mut new_args: Vec<Expr> = Vec::new();
                let arguments_result: Result<Vec<Type>, String> = arguments
                    .drain(..)
                    .map(|arg| -> Result<Type, String> {
                        let (expr, type_, dep) = check_expr_types(arg, &mut type_context)?;
                        deps.extend(dep);
                        new_args.push(expr);
                        Ok(type_)
                    })
                    .collect();
                let arguments = arguments_result?;
                let arguments_match =
                    Type::arguments_match_parameters(&arguments, &has_more_args, &parameters);
                if !arguments_match {
                    return Err("Type mismatch in function call".to_string());
                }

                Ok((
                    Expr::FunctionCall(Box::new(function), new_args),
                    *returned,
                    deps,
                ))
            } else {
                Err("Type mismatch in function call".to_string())
            }
        }
    }
}

fn check_type_annotation_types(
    type_: TypeAnnotationUnchecked,
    type_context: &mut TypeContext,
) -> Result<Type, String> {
    Ok(match type_ {
        TypeAnnotationUnchecked::Number(unit_expr) => {
            let type_ = match unit_expr {
                Some(unit_expr) => {
                    // _dep is ignored because it only gets used in the type context
                    let (_expr, type_, _dep) = check_expr_types(unit_expr, type_context)?;
                    Some(type_)
                }
                None => None,
            };
            println!("{:?}", type_);
            let unit = match type_ {
                Some(Type::Number(unit, Some(NumberConstant::Integer(1)))) => unit,
                Some(Type::Number(unit, Some(NumberConstant::Float(f)))) if f == 1.0 => unit,
                None => UnitSet::empty(),
                _ => return Err("Type mismatch in number annotation".to_string()),
            };
            Type::Number(unit, None)
        }
        TypeAnnotationUnchecked::List(item_type) => {
            let type_ = match item_type {
                Some(unit_expr) => {
                    // _dep is ignored because it only gets used in the type context
                    let (_expr, type_, _dep) = check_expr_types(unit_expr, type_context)?;
                    if let Type::Type(type_) = type_ {
                        *type_
                    } else {
                        return Err("Type mismatch in list type annotation".to_string());
                    }
                }
                None => Type::Any,
            };
            Type::List(Box::new(type_))
        }
        TypeAnnotationUnchecked::Custom(_) => unimplemented!(),
    })
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
        _ => return Err("Type mismatch in binary operation".to_string()),
    };
    let mut deps = dep1;
    deps.extend(dep2);
    Ok((expr, type_, deps))
}
