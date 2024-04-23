use crate::{
    ast::{AssignmentType, Expr, ExprInfo, ExprUnchecked, Line, LineUnchecked, Op},
    types::{NumberConstant, Type, TypeContext},
    units::{Unit, UnitSet},
};

pub fn check_types(
    ast: Vec<LineUnchecked>,
    type_context: &mut TypeContext,
) -> Result<(Type, Vec<Line>), String> {
    let mut new_ast: Vec<Line> = Vec::new();
    let mut return_type: Type = Type::Void;
    for line in ast {
        return_type = match line {
            LineUnchecked::Expr(expr) => {
                let expr = check_expr_types(expr, type_context)?;
                let type_ = expr.type_.clone();
                new_ast.push(Line::Expr(expr));
                type_
            }
            LineUnchecked::Assign(var, expr, assignment_type) => {
                let expr = check_expr_types(expr, type_context)?;
                let type2 = expr.type_.clone();
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
    Ok((return_type, new_ast))
}

fn check_expr_types(
    expr: ExprUnchecked,
    mut type_context: &mut TypeContext,
) -> Result<Expr, String> {
    match expr {
        ExprUnchecked::Number(i) => Ok(Expr {
            info: ExprInfo::Number(i),
            type_: Type::Number(UnitSet::empty(), Some(NumberConstant::Integer(i))),
        }),
        ExprUnchecked::Boolean(b) => Ok(Expr {
            type_: Type::Bool,
            info: ExprInfo::Boolean(b),
        }),
        ExprUnchecked::UnaryMinus(expr) => {
            let expr = check_expr_types(*expr, type_context)?;
            let Expr { type_, info: _ } = &expr;
            match type_ {
                Type::Number(unit, number_constant) => {
                    let number_constant = match number_constant {
                        Some(NumberConstant::Integer(i)) => Some(NumberConstant::Integer(-i)),
                        _ => None,
                    };
                    Ok(Expr {
                        type_: Type::Number(unit.clone(), number_constant),
                        info: ExprInfo::UnaryMinus(Box::new(expr)),
                    })
                }
                _ => Err("Type mismatch in unary minus".to_string()),
            }
        }
        ExprUnchecked::Sequencial(lhs, rhs) => {
            let expr1 = check_expr_types(*lhs, type_context)?;
            let expr2 = check_expr_types(*rhs, type_context)?;
            match (&expr1.type_.clone(), &expr2.type_) {
                (Type::Number(_, _), Type::Number(_, _)) => {
                    handle_bin_op(expr1, Op::Multiply, expr2)
                }
                (Type::Function(params, returned), param) => {
                    if params.len() != 1 {
                        return Err("Type mismatch in sequencial expression".to_string());
                    }
                    if params[0] != *param {
                        return Err("Type mismatch in sequencial expression".to_string());
                    }

                    Ok(Expr {
                        info: ExprInfo::FunctionCall(Box::new(expr1), vec![expr2]),
                        type_: *returned.clone(),
                    })
                }
                _ => Err("Type mismatch in sequencial expression".to_string()),
            }
        }
        ExprUnchecked::BinOp { lhs, op, rhs } => {
            let expr1 = check_expr_types(*lhs, type_context)?;
            let expr2 = check_expr_types(*rhs, type_context)?;
            handle_bin_op(expr1, op, expr2)
        }
        ExprUnchecked::Variable(var) => {
            if let Some((id, type_, _)) = type_context.get_variable(&var) {
                Ok(Expr {
                    info: ExprInfo::Variable(id.clone()),
                    type_: type_.clone(),
                })
            } else {
                Err(format!("Variable {} not found in scope", var))
            }
        }
        ExprUnchecked::If(conditions, blocks, else_block) => {
            let mut new_conditions: Vec<Expr> = Vec::new();
            for condition in conditions {
                let expr = check_expr_types(condition, &mut type_context)?;
                if expr.type_ != Type::Bool {
                    return Err("Type mismatch in if condition".to_string());
                }
                new_conditions.push(expr);
            }
            let mut new_blocks: Vec<Vec<Line>> = Vec::new();
            let mut return_type: Option<Type> = None;
            for block in blocks {
                let (type_, new_block) = check_types(block, type_context)?;
                return_type = match return_type {
                    Some(return_type) if type_ != return_type => Some(Type::Void),
                    _ => Some(type_),
                };
                new_blocks.push(new_block);
            }
            let new_else_block = match else_block {
                Some(else_block) => {
                    let (type_, new_else_block) = check_types(else_block, type_context)?;
                    return_type = match return_type {
                        Some(return_type) if type_ != return_type => Some(Type::Void),
                        _ => Some(type_),
                    };
                    Some(new_else_block)
                }
                None => None,
            };
            let return_type = return_type.unwrap_or(Type::Void);
            let info = ExprInfo::If(new_conditions, new_blocks, new_else_block);
            Ok(Expr {
                info,
                type_: return_type,
            })
        }
        ExprUnchecked::For(var, expr, block) => {
            let expr = check_expr_types(*expr, &mut type_context)?;
            let i_type = match expr.type_ {
                Type::Range => Type::Number(UnitSet::empty(), None),
                _ => return Err("Type mismatch in for loop".to_string()),
            };
            type_context.push_scope();
            let id = type_context.insert_variable(var.clone(), i_type, true);
            let (type_, block) = check_types(block, type_context)?;
            type_context.pop_scope();
            let info = ExprInfo::For(id, Box::new(expr), block);
            Ok(Expr { info, type_ })
        }
        ExprUnchecked::Block(lines) => {
            type_context.push_scope();
            let (type_, new_lines) = check_types(lines, &mut type_context)?;
            type_context.pop_scope();
            Ok(Expr {
                info: ExprInfo::Block(new_lines),
                type_,
            })
        }
    }
}

fn handle_bin_op(expr1: Expr, op: Op, expr2: Expr) -> Result<Expr, String> {
    match (expr1.type_.clone(), &op, expr2.type_.clone()) {
        (Type::Number(unit1, _), Op::Range, Type::Number(unit2, _)) => {
            if !unit1.is_empty() || !unit2.is_empty() {
                return Err("Unit mismatch in range operation".to_string());
            }
            Ok(Expr {
                type_: Type::Range,
                info: ExprInfo::BinOp {
                    lhs: Box::new(expr1),
                    op: Op::Range,
                    rhs: Box::new(expr2),
                },
            })
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
            let info = match c {
                Some(NumberConstant::Integer(i)) => ExprInfo::Number(i),
                Some(NumberConstant::Float(f)) => ExprInfo::Number(f as i64),
                _ => ExprInfo::BinOp {
                    lhs: Box::new(expr1),
                    op,
                    rhs: Box::new(expr2),
                },
            };
            Ok(Expr {
                type_: Type::Number(unit, c),
                info,
            })
        }
        _ => Err("Type mismatch in binary operation".to_string()),
    }
}
