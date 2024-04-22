pub trait AstNode {
    fn to_python(&self) -> String;
}

#[derive(Debug)]
pub enum LineNode<VarT> {
    Expr(ExprNode<VarT>),
    Assign(VarT, ExprNode<VarT>),
}

impl AstNode for LineNode<u32> {
    fn to_python(&self) -> String {
        match self {
            LineNode::Expr(expr) => expr.to_python(),
            LineNode::Assign(var, expr) => format!("{} = {}", var, expr.to_python()),
        }
    }
}

#[derive(Debug)]
pub enum ExprNode<VarT> {
    Number(i32, String), // The second field is the unit
    UnaryMinus(Box<ExprNode<VarT>>),
    BinOp {
        lhs: Box<ExprNode<VarT>>,
        op: Op,
        rhs: Box<ExprNode<VarT>>,
    },
    Variable(VarT),
    If(
        Vec<ExprNode<VarT>>,
        Vec<Vec<LineNode<VarT>>>,
        Option<Vec<LineNode<VarT>>>,
    ),
    For(VarT, Box<ExprNode<VarT>>, Vec<LineNode<VarT>>),
    Boolean(bool),
}

impl AstNode for ExprNode<u32> {
    fn to_python(&self) -> String {
        match self {
            ExprNode::Number(i, _) => i.to_string(),
            ExprNode::UnaryMinus(expr) => format!("-{}", expr.to_python()),
            ExprNode::If(_, _, _) => unimplemented!(),
            ExprNode::Variable(var) => var.to_string(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Range,
}
