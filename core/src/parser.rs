use pest::iterators::Pair;
use pest::pratt_parser::PrattParser;
use pest::{iterators::Pairs, Parser};

use crate::ast::{ExprUnchecked, LineUnchecked, Op};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

pub fn parse(input: &str) -> Result<Vec<LineUnchecked>, pest::error::Error<Rule>> {
    let pairs = MyParser::parse(Rule::entry, input)?;
    // println!("{:?}", pairs);
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
                let expr = inner.next().unwrap();
                let assignment_type = match rule {
                    Rule::normal_assignment_line => crate::ast::AssignmentType::Normal,
                    Rule::let_assignment_line => crate::ast::AssignmentType::Let,
                    Rule::const_assignment_line => crate::ast::AssignmentType::Const,
                    _ => unreachable!(),
                };
                let node = LineUnchecked::Assign(var, build_expr_ast(expr), assignment_type);
                lines.push(node);
            }
            Rule::unitdef_line => {
                let unit = pair.into_inner().next().unwrap().as_str().to_string();
                let node = LineUnchecked::UnitDef(unit);
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
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
            .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left) | Op::infix(Rule::modulo, Left))
            .op(Op::infix(Rule::power, Left))
            .op(Op::prefix(Rule::unary_minus))
    };
}

fn build_expr_ast(pair: Pair<Rule>) -> ExprUnchecked {
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
            let expr = inner.next().unwrap();
            let block = inner.next().unwrap();
            ExprUnchecked::For(
                var,
                Box::new(build_expr_ast(expr)),
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

pub fn build_op_expr_ast(pair: Pair<Rule>) -> ExprUnchecked {
    assert!(pair.as_rule() == Rule::expr_ops);
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::expr => build_expr_ast(primary),
            Rule::variable => ExprUnchecked::Variable(primary.as_str().to_string()),
            Rule::true_ => ExprUnchecked::Boolean(true),
            Rule::false_ => ExprUnchecked::Boolean(false),
            Rule::number => {
                let mut inner = primary.into_inner();
                let number = inner.next().unwrap().as_str();
                ExprUnchecked::Number(number.parse::<i64>().unwrap())
            }
            Rule::block => {
                let inner = primary.into_inner();
                let lines = build_line_ast(inner);
                ExprUnchecked::Block(lines)
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
        .parse(pair.into_inner())
}
