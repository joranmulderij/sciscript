// pub trait AstNode {
//     fn to_python(&self) -> String;
// }

#[derive(Debug)]
pub enum LineNode<VarT, UnitT> {
    Expr(ExprNode<VarT, UnitT>),
    Assign(VarT, ExprNode<VarT, UnitT>),
    UnitDef(UnitT),
}

// impl AstNode for LineNode<u32> {
//     fn to_python(&self) -> String {
//         match self {
//             LineNode::Expr(expr) => expr.to_python(),
//             LineNode::Assign(var, expr) => format!("{} = {}", var, expr.to_python()),
//         }
//     }
// }

#[derive(Debug)]
pub enum ExprNode<VarT, UnitT> {
    Number(i64),
    UnaryMinus(Box<ExprNode<VarT, UnitT>>),
    BinOp {
        lhs: Box<ExprNode<VarT, UnitT>>,
        op: Op,
        rhs: Box<ExprNode<VarT, UnitT>>,
    },
    Variable(VarT),
    If(
        Vec<ExprNode<VarT, UnitT>>,
        Vec<Vec<LineNode<VarT, UnitT>>>,
        Option<Vec<LineNode<VarT, UnitT>>>,
    ),
    For(VarT, Box<ExprNode<VarT, UnitT>>, Vec<LineNode<VarT, UnitT>>),
    Boolean(bool),
}

// impl AstNode for ExprNode<u32> {
//     fn to_python(&self) -> String {
//         match self {
//             ExprNode::Number(i, _) => i.to_string(),
//             ExprNode::UnaryMinus(expr) => format!("-{}", expr.to_python()),
//             ExprNode::If(_, _, _) => unimplemented!(),
//             ExprNode::Variable(var) => var.to_string(),
//             _ => unimplemented!(),
//         }
//     }
// }

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
