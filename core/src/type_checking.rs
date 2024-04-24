use std::collections::{hash_set, HashSet};

use crate::{
    ast::{AssignmentType, Expr, ExprUnchecked, Line, LineUnchecked, Op, UncheckedTypeAnnotation},
    types::{NumberConstant, Type, TypeContext},
    units::{parse_unit, Unit, UnitSet},
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
            LineUnchecked::Assign(var, expr, assignment_type) => {
                let (expr, type2, dep) = check_expr_types(expr, type_context)?;
                deps.extend(dep);
                let id: String = match assignment_type {
                    AssignmentType::Normal => {
                        if let Some((id, type1, const_)) = type_context.get_variable(&var) {
                            if type1 != &type2 {
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
                        let id = type_context.insert_variable(var, type2.clone(), const_);
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
        ExprUnchecked::Number(i) => Ok((
            Expr::Number(i),
            Type::Number(UnitSet::empty(), Some(NumberConstant::Integer(i))),
            HashSet::new(),
        )),
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
            let (expr2, type2, mut dep2) = check_expr_types(*rhs, type_context)?;
            match (&type1, &type2) {
                (Type::Number(_, _), Type::Number(_, _)) => {
                    let pack1 = (expr1, type1, dep1);
                    let pack2 = (expr2, type2, dep2);
                    handle_bin_op(pack1, Op::Multiply, pack2)
                }
                (Type::Function(params, returned), param) => {
                    if params.len() != 1 {
                        return Err("Type mismatch in sequencial expression".to_string());
                    }
                    if params[0] != *param {
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
                Ok((
                    Expr::Variable(id.clone()),
                    type_.clone(),
                    HashSet::from([id.clone()]),
                ))
            } else {
                Err(format!("Variable {} not found in scope", var))
            }
        }
        ExprUnchecked::If(conditions, blocks, else_block) => {
            let mut new_conditions: Vec<Expr> = Vec::new();
            let mut deps: HashSet<String> = HashSet::new();
            for condition in conditions {
                let (expr, type_, deps) = check_expr_types(condition, &mut type_context)?;
                if type_ != Type::Bool {
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
                    Some(return_type) if type_ != return_type => Some(Type::Void),
                    _ => Some(type_),
                };
                new_blocks.push(new_block);
            }
            let new_else_block = match else_block {
                Some(else_block) => {
                    let (new_else_block, type_, dep) = check_types(else_block, type_context)?;
                    deps.extend(dep);
                    return_type = match return_type {
                        Some(return_type) if type_ != return_type => Some(Type::Void),
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
            type_context.print_last_scope();
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
            type_context.push_scope();
            for (name, type_) in parameters {
                let type_ = match type_ {
                    UncheckedTypeAnnotation::Number(unit) => {
                        let unit = parse_unit(&unit);
                        Type::Number(unit, None)
                    }
                    UncheckedTypeAnnotation::Custom(_) => unimplemented!(),
                };
                let name = type_context.insert_variable(name, type_.clone(), true);
                param_types.push(type_);
                param_names.push(name);
            }
            let (block, return_type, deps) = check_expr_types(*block, &mut type_context)?;
            type_context.pop_scope();
            let type_ = Type::Function(param_types, Box::new(return_type));
            let deps: HashSet<String> = deps
                .iter()
                .filter(|x| !param_names.contains(x))
                .map(|x| x.clone())
                .collect();
            let expr = Expr::Lambda(param_names, Box::new(block), deps.clone());
            Ok((expr, type_, deps))
        }
    }
}

fn handle_bin_op(
    left: (Expr, Type, HashSet<String>),
    op: Op,
    right: (Expr, Type, HashSet<String>),
) -> Result<(Expr, Type, HashSet<String>), String> {
    let (expr1, type1, dep1) = left;
    let (expr2, type2, mut dep2) = right;
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
        (Type::Number(unit1, c1), _, Type::Number(unit2, c2)) => {
            let c = match (c1, &c2) {
                (Some(NumberConstant::Integer(i1)), Some(NumberConstant::Integer(i2))) => {
                    match op {
                        Op::Multiply => Some(NumberConstant::Integer(i1 * i2)),
                        Op::Divide => Some(NumberConstant::Float(i1 as f64 / *i2 as f64)),
                        Op::Add => Some(NumberConstant::Integer(i1 + i2)),
                        Op::Subtract => Some(NumberConstant::Integer(i1 - i2)),
                        Op::Modulo => Some(NumberConstant::Integer(i1 % i2)),
                        Op::Power => Some(NumberConstant::Float((i1 as f64).powf(*i2 as f64))),
                        _ => unreachable!(),
                    }
                }
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
            let expr = match c {
                Some(NumberConstant::Integer(i)) => Expr::Number(i),
                Some(NumberConstant::Float(f)) => Expr::Number(f as i64),
                _ => Expr::BinOp {
                    lhs: Box::new(expr1),
                    op,
                    rhs: Box::new(expr2),
                },
            };
            (expr, Type::Number(unit, c))
        }
        _ => return Err("Type mismatch in binary operation".to_string()),
    };
    let mut deps = dep1;
    deps.extend(dep2);
    Ok((expr, type_, deps))
}
