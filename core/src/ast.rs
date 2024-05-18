// pub trait AstNode {
//     fn to_python(&self) -> String;
// }

use std::collections::{HashMap, HashSet};

use crate::types::NumberConstant;

#[derive(Debug)]
pub enum LineUnchecked {
    Expr(ExprUnchecked),
    NewAssignment(
        String,
        Option<TypeAnnotationUnchecked>,
        ExprUnchecked,
        NewAssignmentModifier,
    ),
    ReAssignment(String, Vec<ReAssignmentExtensionUnchecked>, ExprUnchecked),
    UnitDef(String),
}

#[derive(Debug)]
pub enum Line {
    Expr(Expr),
    NewAssignment(String, Expr, NewAssignmentModifier),
    ReAssignment(String, Vec<ReAssignmentExtension>, Expr),
}

#[derive(Debug)]
pub enum ExprUnchecked {
    Number(NumberConstant),
    UnaryMinus(Box<ExprUnchecked>),
    GetProperty(Box<ExprUnchecked>, String),
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
    Null,
    Block(Vec<LineUnchecked>),
    Lambda(
        Vec<(String, TypeAnnotationUnchecked, Option<ExprUnchecked>)>,
        Box<ExprUnchecked>,
        Option<TypeAnnotationUnchecked>,
    ),
    List(Vec<ExprUnchecked>),
    Map(Vec<(ExprUnchecked, ExprUnchecked)>),
    Index(Box<ExprUnchecked>, Box<ExprUnchecked>),
    FunctionCall(
        Box<ExprUnchecked>,
        Vec<ExprUnchecked>,
        HashMap<String, ExprUnchecked>,
    ),
    Struct(
        Vec<(
            String,
            Option<TypeAnnotationUnchecked>,
            Option<ExprUnchecked>,
            StructFieldKind,
        )>,
    ),
    Matrix(Vec<Vec<ExprUnchecked>>),
}

#[derive(Debug)]
pub enum StructFieldKind {
    Property,
    Method,
}

#[derive(Debug)]
pub struct TypeAnnotationUnchecked {
    pub name: String,
    pub generics: Vec<ExprUnchecked>,
}

#[derive(Debug)]
pub enum Expr {
    Number(NumberConstant),
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
    Null,
    Block(Vec<Line>),
    FunctionCall(Box<Expr>, Vec<(String, Expr)>),
    Lambda(Vec<(String, Option<Expr>)>, Box<Expr>, HashSet<String>),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Struct(Vec<(String, Option<Expr>, StructFieldKind)>),
    GetProperty(Box<Expr>, String),
    Map(Vec<(Expr, Expr)>),
    Matrix(Vec<Vec<Expr>>),
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
    Equals,
    NotEquals,
}

#[derive(Debug)]
pub enum NewAssignmentModifier {
    Let,
    Const,
}

#[derive(Debug)]
pub enum ReAssignmentExtension {
    Property(String),
    Index(Expr),
}

#[derive(Debug)]
pub enum ReAssignmentExtensionUnchecked {
    PropGet(String),
    Index(ExprUnchecked),
}
