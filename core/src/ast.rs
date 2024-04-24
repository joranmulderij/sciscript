// pub trait AstNode {
//     fn to_python(&self) -> String;
// }

use std::collections::HashSet;

use crate::types::Type;

#[derive(Debug)]
pub enum LineUnchecked {
    Expr(ExprUnchecked),
    Assign(String, ExprUnchecked, AssignmentType),
    UnitDef(String),
}

#[derive(Debug)]
pub enum Line {
    Expr(Expr),
    Assign(String, Expr, AssignmentType),
}

#[derive(Debug)]
pub enum ExprUnchecked {
    Number(i64),
    UnaryMinus(Box<ExprUnchecked>),
    BinOp {
        lhs: Box<ExprUnchecked>,
        op: Op,
        rhs: Box<ExprUnchecked>,
    },
    // FunctionCall(Box<ExprUnchecked>, Vec<ExprUnchecked>),
    Variable(String),
    Sequencial(Box<ExprUnchecked>, Box<ExprUnchecked>),
    If(
        Vec<ExprUnchecked>,
        Vec<Vec<LineUnchecked>>,
        Option<Vec<LineUnchecked>>,
    ),
    For(String, Box<ExprUnchecked>, Vec<LineUnchecked>),
    Boolean(bool),
    Block(Vec<LineUnchecked>),
    Lambda(Vec<(String, UncheckedTypeAnnotation)>, Box<ExprUnchecked>),
}

#[derive(Debug)]
pub enum UncheckedTypeAnnotation {
    Number(String),
    Custom(String),
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Variable(String),
    If(Vec<Expr>, Vec<Vec<Line>>, Option<Vec<Line>>),
    For(String, Box<Expr>, Vec<Line>),
    Boolean(bool),
    Block(Vec<Line>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>, HashSet<String>),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Range,
}

#[derive(Debug)]
pub enum AssignmentType {
    Normal,
    Let,
    Const,
}
