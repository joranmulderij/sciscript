use std::collections::HashMap;

use pest::iterators::Pair;
use pest::pratt_parser::PrattParser;
use pest::{iterators::Pairs, Parser};

use crate::ast::{AssignmentType, ExprUnchecked, LineUnchecked, Op, TypeAnnotationUnchecked};
use crate::types::NumberConstant;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

pub fn parse(input: &str) -> Result<Vec<LineUnchecked>, pest::error::Error<Rule>> {
    let pairs = MyParser::parse(Rule::entry, input)?;
    Ok(build_line_ast(pairs))
}

fn build_line_ast(pairs: Pairs<Rule>) -> Vec<LineUnchecked> {
    let mut lines: Vec<LineUnchecked> = Vec::new();
    for pair in pairs {
        let rule = pair.as_rule();
        match &rule {
            Rule::expr_line => {
                let expr = pair.into_inner().next().unwrap();
                let node = LineUnchecked::Expr(build_expr_ast(expr));
                lines.push(node);
            }
            Rule::normal_assignment_line
            | Rule::const_assignment_line
            | Rule::let_assignment_line => {
                let mut inner = pair.into_inner();
                let var = inner.next().unwrap().as_str().to_string();
                let optional_type_annotation = inner.next().unwrap();
                println!("{:?}", optional_type_annotation.as_rule());
                let type_annotation = match optional_type_annotation.into_inner().next() {
                    Some(type_annotation) => Some(parse_type_annotation(type_annotation)),
                    None => None,
                };
                let expr = inner.next().unwrap();
                let assignment_type = match rule {
                    Rule::normal_assignment_line => crate::ast::AssignmentType::Normal,
                    Rule::let_assignment_line => crate::ast::AssignmentType::Let,
                    Rule::const_assignment_line => crate::ast::AssignmentType::Const,
                    _ => unreachable!(),
                };
                let node = LineUnchecked::Assign(
                    var,
                    type_annotation,
                    build_expr_ast(expr),
                    assignment_type,
                );
                lines.push(node);
            }
            Rule::unitdef_line => {
                let unit = pair.into_inner().next().unwrap().as_str().to_string();
                let node = LineUnchecked::UnitDef(unit);
                lines.push(node);
            }
            Rule::function_line => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let args = inner.next().unwrap();
                let block = inner.next().unwrap();
                let args = parse_args(args);
                let node = LineUnchecked::Assign(
                    name,
                    None,
                    ExprUnchecked::Lambda(
                        args,
                        Box::new(ExprUnchecked::Block(build_line_ast(block.into_inner()))),
                    ),
                    AssignmentType::Let,
                );
                lines.push(node);
            }
            Rule::struct_line => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let block = inner.next().unwrap();
                let fields = block
                    .into_inner()
                    .map(|line| {
                        let mut inner = line.into_inner();
                        let name = inner.next().unwrap().as_str().to_string();
                        let type_annotation = inner.next().unwrap();
                        let type_annotation = parse_type_annotation(type_annotation);
                        let default_value = inner.next();
                        let default_value = match default_value {
                            Some(default_value) => Some(build_expr_ast(default_value)),
                            None => None,
                        };
                        (name, type_annotation, default_value)
                    })
                    .collect();
                let node = LineUnchecked::Assign(
                    name,
                    None,
                    ExprUnchecked::Struct(fields),
                    AssignmentType::Let,
                );
                lines.push(node);
            }
            Rule::EOI => {}
            _ => {
                println!("{:?}", pair.as_rule());
                unreachable!();
            }
        }
    }
    lines
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(Rule::range, Left))
            .op(Op::infix(Rule::equals, Left) | Op::infix(Rule::not_equals, Left))
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
            .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left) | Op::infix(Rule::modulo, Left))
            .op(Op::infix(Rule::power, Left))
            .op(Op::prefix(Rule::unary_minus) | Op::postfix(Rule::propget) | Op::postfix(Rule::index) | Op::postfix(Rule::function_call))
    };
}

fn build_expr_ast(pair: Pair<Rule>) -> ExprUnchecked {
    assert!(pair.as_rule() == Rule::expr);
    let mut pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
    let rule = &pairs[0].as_rule();
    match rule {
        Rule::if_statement => {
            let first_pair = pairs.remove(0);
            let inner = first_pair.into_inner();
            let mut conditions: Vec<ExprUnchecked> = Vec::new();
            let mut blocks: Vec<Vec<LineUnchecked>> = Vec::new();
            let mut else_block: Option<Vec<LineUnchecked>> = None;
            for item in inner {
                match item.as_rule() {
                    Rule::expr => conditions.push(build_expr_ast(item)),
                    Rule::block => {
                        let ast = build_line_ast(item.into_inner());
                        if blocks.len() + 1 == conditions.len() {
                            blocks.push(ast);
                        } else if blocks.len() == conditions.len() {
                            else_block.replace(ast);
                        } else {
                            unreachable!();
                        }
                    }
                    _ => unreachable!(),
                }
            }
            ExprUnchecked::If(conditions, blocks, else_block)
        }
        Rule::for_statement => {
            let first_pair = pairs.remove(0);
            let mut inner = first_pair.into_inner();
            let var = inner.next().unwrap().as_str().to_string();
            let expr_ops = inner.next().unwrap();
            let block = inner.next().unwrap();
            ExprUnchecked::For(
                var,
                Box::new(build_op_expr_ast(expr_ops)),
                build_line_ast(block.into_inner()),
            )
        }
        Rule::expr_ops => {
            let mut expr: Option<ExprUnchecked> = None;
            pairs.reverse();
            for item in pairs {
                let expr_left = build_op_expr_ast(item);
                expr = match expr {
                    Some(expr_right) => Some(ExprUnchecked::Sequencial(
                        Box::new(expr_left),
                        Box::new(expr_right),
                    )),
                    None => Some(expr_left),
                };
            }
            expr.unwrap()
        }
        _ => unreachable!(),
    }
}

fn build_op_expr_ast(pair: Pair<Rule>) -> ExprUnchecked {
    assert!(pair.as_rule() == Rule::expr_ops);
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::expr => build_expr_ast(primary),
            Rule::variable => ExprUnchecked::Variable(primary.as_str().to_string()),
            Rule::true_ => ExprUnchecked::Boolean(true),
            Rule::false_ => ExprUnchecked::Boolean(false),
            Rule::null => ExprUnchecked::Null,
            Rule::integer => {
                let number = primary.as_str();
                ExprUnchecked::Number(NumberConstant::Integer(number.parse::<i64>().unwrap()))
            }
            Rule::float => {
                let number = primary.as_str();
                ExprUnchecked::Number(NumberConstant::Float(number.parse::<f64>().unwrap()))
            }
            Rule::block => {
                let inner = primary.into_inner();
                let lines = build_line_ast(inner);
                ExprUnchecked::Block(lines)
            }
            Rule::lambda => {
                let mut inner = primary.into_inner();
                let arguments = inner.next().unwrap();
                let expr = inner.next().unwrap();
                let args = parse_args(arguments);
                ExprUnchecked::Lambda(args, Box::new(build_expr_ast(expr)))
            }
            Rule::list => {
                let inner = primary.into_inner();
                let items = inner.map(build_expr_ast).collect();
                ExprUnchecked::List(items)
            }
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let rule = op.as_rule();
            let op = match rule {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                Rule::range => Op::Range,
                Rule::power => Op::Power,
                Rule::equals => Op::Equals,
                Rule::not_equals => Op::NotEquals,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            ExprUnchecked::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => ExprUnchecked::UnaryMinus(Box::new(rhs)),
            // Rule::variable => {
            //     let op = ExprUnchecked::Variable(op.as_str().to_string());
            //     ExprUnchecked::FunctionCall(Box::new(op), vec![rhs])
            // }
            _ => unreachable!(
                "Expr::parse expected prefix operation, found {:?}",
                op.as_rule()
            ),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::propget => ExprUnchecked::GetProperty(
                Box::new(lhs),
                op.into_inner().next().unwrap().as_str().to_string(),
            ),
            Rule::index => {
                let index = op.into_inner().next().unwrap();
                ExprUnchecked::Index(Box::new(lhs), Box::new(build_expr_ast(index)))
            }
            Rule::function_call => {
                let mut positional_args = Vec::new();
                let mut named_args = HashMap::new();
                for arg in op.into_inner() {
                    match arg.as_rule() {
                        Rule::expr => positional_args.push(build_expr_ast(arg)),
                        Rule::named_argument => {
                            let mut inner = arg.into_inner();
                            let name = inner.next().unwrap().as_str().to_string();
                            let expr = inner.next().unwrap();
                            named_args.insert(name, build_expr_ast(expr));
                        }
                        _ => unreachable!(),
                    }
                }
                // let args = op.into_inner().map(build_expr_ast).collect();
                ExprUnchecked::FunctionCall(Box::new(lhs), positional_args, named_args)
            }
            _ => unreachable!(
                "Expr::parse expected postfix operation, found {:?}",
                op.as_rule()
            ),
        })
        .parse(pair.into_inner())
}

fn parse_args(pair: Pair<Rule>) -> Vec<(String, TypeAnnotationUnchecked, Option<ExprUnchecked>)> {
    pair.into_inner()
        .map(|arg| {
            let mut inner = arg.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let type_annotation = inner.next().unwrap();
            let type_ = parse_type_annotation(type_annotation);
            let default_value = inner.next();
            let default_value = match default_value {
                Some(default_value) => Some(build_expr_ast(default_value)),
                None => None,
            };
            (name, type_, default_value)
        })
        .collect()
}

fn parse_type_annotation(type_annotation: Pair<'_, Rule>) -> TypeAnnotationUnchecked {
    assert!(
        type_annotation.as_rule() == Rule::type_annotation,
        "{:?}",
        type_annotation.as_rule()
    );
    let mut inner = type_annotation.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let generics = inner.map(build_op_expr_ast).collect();
    TypeAnnotationUnchecked { name, generics }
    // let type_annotation = type_annotation.into_inner().next().unwrap();
    // match type_annotation.as_rule() {
    //     Rule::num_type => {
    //         let unit = type_annotation.into_inner().next();
    //         let unit = match unit {
    //             Some(unit) => Some(build_op_expr_ast(unit)),
    //             None => None,
    //         };
    //         TypeAnnotationUnchecked::Number(unit)
    //     }
    //     Rule::list_type => {
    //         let item_type = type_annotation.into_inner().next();
    //         let item_type = match item_type {
    //             Some(item_type) => Some(build_op_expr_ast(item_type)),
    //             None => None,
    //         };
    //         TypeAnnotationUnchecked::List(item_type)
    //     }
    //     Rule::variable => TypeAnnotationUnchecked::Custom(type_annotation.as_str().to_string()),
    //     _ => unreachable!(),
    // }
}
