use crate::{
    ast::{ExprNode, LineNode, Op},
    types::{Type, TypeContext},
};

pub fn check_types(ast: Vec<LineNode<String>>) -> Result<(Type, Vec<LineNode<u32>>), String> {
    let mut type_context = TypeContext::new();
    let mut new_ast: Vec<LineNode<u32>> = Vec::new();
    let mut return_type: Type = Type::Void;
    for line in ast {
        return_type = match line {
            LineNode::Expr(expr) => {
                let (type_, expr) = check_expr_types(expr, &mut type_context)?;
                new_ast.push(LineNode::Expr(expr));
                type_
            }
            LineNode::Assign(var, expr) => {
                let (type1, expr) = check_expr_types(expr, &mut type_context)?;
                let id = if let Some((id, type2)) = type_context.get_variable(&var) {
                    if type1 != *type2 {
                        return Err("Type mismatch in assignment".to_string());
                    }
                    *id
                } else {
                    let id = type_context.insert_variable(var, type1.clone());
                    id
                };
                new_ast.push(LineNode::Assign(id, expr));
                type1
            }
        }
    }
    Ok((return_type, new_ast))
}

fn check_expr_types(
    expr: ExprNode<String>,
    mut type_context: &mut TypeContext,
) -> Result<(Type, ExprNode<u32>), String> {
    match expr {
        ExprNode::Number(i, unit) => Ok((Type::Number(unit.clone()), ExprNode::Number(i, unit))),
        ExprNode::Boolean(b) => Ok((Type::Bool, ExprNode::Boolean(b))),
        ExprNode::UnaryMinus(expr) => {
            let (type_, expr) = check_expr_types(*expr, type_context)?;
            match type_ {
                Type::Number(unit) => {
                    Ok((Type::Number(unit), ExprNode::UnaryMinus(Box::new(expr))))
                }
                _ => Err("Type mismatch in unary minus".to_string()),
            }
        }
        ExprNode::BinOp { lhs, op, rhs } => {
            let (lht, lhs) = check_expr_types(*lhs, type_context)?;
            let (rht, rhs) = check_expr_types(*rhs, type_context)?;
            match (lht, op, rht) {
                (Type::Number(unit1), Op::Range, Type::Number(unit2)) => {
                    if unit1 != "" || unit2 != "" {
                        return Err("Unit mismatch in range operation".to_string());
                    }
                    Ok((
                        Type::Range,
                        ExprNode::BinOp {
                            lhs: Box::new(lhs),
                            op: Op::Range,
                            rhs: Box::new(rhs),
                        },
                    ))
                }
                (Type::Number(unit1), op, Type::Number(unit2)) => {
                    if unit1 != unit2 {
                        return Err("Unit mismatch in binary operation".to_string());
                    }
                    Ok((
                        Type::Number(unit1),
                        ExprNode::BinOp {
                            lhs: Box::new(lhs),
                            op,
                            rhs: Box::new(rhs),
                        },
                    ))
                }
                _ => Err("Type mismatch in binary operation".to_string()),
            }
        }
        ExprNode::Variable(var) => {
            if let Some((id, type_)) = type_context.get_variable(&var) {
                Ok((type_.clone(), ExprNode::Variable(*id)))
            } else {
                Err(format!("Variable {} not found in scope", var))
            }
        }
        ExprNode::If(conditions, blocks, else_block) => {
            let mut new_conditions = Vec::new();
            for condition in conditions {
                let (type_, condition) = check_expr_types(condition, &mut type_context)?;
                if type_ != Type::Bool {
                    return Err("Type mismatch in if condition".to_string());
                }
                new_conditions.push(condition);
            }
            let mut new_blocks: Vec<Vec<LineNode<u32>>> = Vec::new();
            let mut return_type: Option<Type> = None;
            for block in blocks {
                let (type_, new_block) = check_types(block)?;
                return_type = match return_type {
                    Some(return_type) if type_ != return_type => Some(Type::Void),
                    _ => Some(type_),
                };
                new_blocks.push(new_block);
            }
            let new_else_block = match else_block {
                Some(else_block) => {
                    let (type_, new_else_block) = check_types(else_block)?;
                    return_type = match return_type {
                        Some(return_type) if type_ != return_type => Some(Type::Void),
                        _ => Some(type_),
                    };
                    Some(new_else_block)
                }
                None => None,
            };
            Ok((
                return_type.unwrap(),
                ExprNode::If(new_conditions, new_blocks, new_else_block),
            ))
        }
        ExprNode::For(var, expr, block) => {
            let (type_, expr) = check_expr_types(*expr, &mut type_context)?;
            let i_type = match type_ {
                Type::Range => Type::Number("".to_string()),
                _ => return Err("Type mismatch in for loop".to_string()),
            };
            type_context.push_scope();
            let id = type_context.insert_variable(var.clone(), i_type);
            let (type_, block) = check_types(block)?;
            type_context.pop_scope();
            Ok((type_, ExprNode::For(id, Box::new(expr), block)))
        }
    }
}
