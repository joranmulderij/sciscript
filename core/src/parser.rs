use pest::pratt_parser::PrattParser;
use pest::{iterators::Pairs, Parser};

use crate::ast::{ExprNode, LineNode, Op};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

pub fn parse(input: &str) -> Result<Vec<LineNode<String, String>>, pest::error::Error<Rule>> {
    let pairs = MyParser::parse(Rule::entry, input)?;
    Ok(build_line_ast(pairs))
}

fn build_line_ast(pairs: Pairs<Rule>) -> Vec<LineNode<String, String>> {
    let mut lines: Vec<LineNode<String, String>> = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr_line => {
                let expr = pair.into_inner().next().unwrap();
                let node = LineNode::Expr(build_expr_ast(expr.into_inner()));
                lines.push(node);
            }
            Rule::assignment_line => {
                let mut inner = pair.into_inner();
                let var = inner.next().unwrap().as_str().to_string();
                let expr = inner.next().unwrap();
                let node = LineNode::Assign(var, build_expr_ast(expr.into_inner()));
                lines.push(node);
            }
            Rule::unitdef_line => {
                let unit = pair.into_inner().next().unwrap().as_str().to_string();
                let node = LineNode::UnitDef(unit);
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

pub fn build_expr_ast(pairs: Pairs<Rule>) -> ExprNode<String, String> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::expr => build_expr_ast(primary.into_inner()),
            Rule::variable => ExprNode::Variable(primary.as_str().to_string()),
            Rule::true_ => ExprNode::Boolean(true),
            Rule::false_ => ExprNode::Boolean(false),
            Rule::number => {
                let mut inner = primary.into_inner();
                let number = inner.next().unwrap().as_str();
                ExprNode::Number(number.parse::<i64>().unwrap())
            }
            Rule::if_statement => {
                let inner = primary.into_inner();
                let mut conditions: Vec<ExprNode<String, String>> = Vec::new();
                let mut blocks: Vec<Vec<LineNode<String, String>>> = Vec::new();
                let mut else_block: Option<Vec<LineNode<String, String>>> = None;
                for item in inner {
                    match item.as_rule() {
                        Rule::expr => conditions.push(build_expr_ast(item.into_inner())),
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
                ExprNode::If(conditions, blocks, else_block)
            }
            Rule::for_statement => {
                let mut inner = primary.into_inner();
                let var = inner.next().unwrap().as_str().to_string();
                let expr = inner.next().unwrap();
                let block = inner.next().unwrap();
                ExprNode::For(
                    var,
                    Box::new(build_expr_ast(expr.into_inner())),
                    build_line_ast(block.into_inner()),
                )
            }
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                Rule::range => Op::Range,
                Rule::power => Op::Power,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            ExprNode::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => ExprNode::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}
