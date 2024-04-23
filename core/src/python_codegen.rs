use crate::{
    ast::{Expr, ExprInfo, Line, Op},
    types::Type,
};

pub fn generate_python_code(ast: Vec<Line>) -> String {
    let mut python_code = String::new();
    for line in ast {
        python_code.push_str(&line.to_python_code());
        python_code.push_str("\n");
    }
    python_code
}

impl Expr {
    /// Expression code, previous line code
    fn to_python_code(&self) -> (String, String) {
        let Expr { type_, info } = self;
        match info {
            ExprInfo::Number(n) => ("".to_string(), n.to_string()),
            ExprInfo::UnaryMinus(expr) => {
                let code = expr.to_python_code();
                (code.0, format!("-{}", code.1))
            }
            ExprInfo::BinOp { lhs, op, rhs } => {
                let (pl1, lhs) = lhs.to_python_code();
                let (pl2, rhs) = rhs.to_python_code();
                if let Type::Range = type_ {
                    return (format!("range({}, {})", lhs, rhs), pl1 + &pl2);
                }
                let expr = format!(
                    "({} {} {})",
                    lhs,
                    match op {
                        Op::Add => "+",
                        Op::Subtract => "-",
                        Op::Multiply => "*",
                        Op::Divide => "/",
                        Op::Modulo => "%",
                        Op::Power => "**",
                        Op::Range => unreachable!(),
                    },
                    rhs
                );
                (pl1 + &pl2, expr)
            }
            ExprInfo::Variable(id) => {
                let expr = if let Type::Number(_, Some(number_constant)) = type_ {
                    number_constant.to_string()
                } else {
                    id.clone()
                };
                ("".to_string(), expr)
            }
            ExprInfo::If(conditions, bodies, else_) => {
                // let mut pl = String::new();
                // for (i, (condition, body)) in conditions.iter().zip(bodies.iter()).enumerate() {
                //     if i == 0 {
                //         pl.push_str(&format!("if {}:\n", condition.to_python_code()));
                //     } else {
                //         pl.push_str(&format!("elif {}:\n", condition.to_python_code()));
                //     }
                //     for line in body {
                //         pl.push_str(&line.to_python_code());
                //         pl.push_str("\n");
                //     }
                // }
                // if let Some(else_) = else_ {
                //     pl.push_str("else:\n");
                //     for line in else_ {
                //         pl.push_str(&line.to_python_code(indent + 1));
                //         pl.push_str("\n");
                //     }
                // }
                // pl
                unimplemented!()
            }
            ExprInfo::For(id, range, body) => {
                let mut lines: Vec<String> = Vec::new();
                for i in 0..body.len() - 1 {
                    lines.push(indent(body[i].to_python_code()));
                }
                let (last_line, expr) = body.last().unwrap().to_python_code_return();
                lines.push(indent(last_line));
                let pl2 = format!(
                    "for {} in {}:\n{}",
                    id,
                    range.to_python_code().0,
                    lines.join("\n")
                );
                (pl2, expr)
            }
            ExprInfo::Boolean(b) => {
                if *b {
                    ("".to_string(), "True".to_string())
                } else {
                    ("".to_string(), "False".to_string())
                }
            }
            // ExprInfo::Block(lines) => lines
            //     .iter()
            //     .map(|line| line.to_python_code())
            //     .collect::<Vec<_>>()
            //     .join("\n"),
            ExprInfo::Block(body) => {
                let mut lines: Vec<String> = Vec::new();
                for i in 0..body.len() - 1 {
                    lines.push(body[i].to_python_code());
                }
                let (last_line, expr) = body.last().unwrap().to_python_code_return();
                lines.push(last_line);
                let pl = lines.join("\n");
                (pl, expr)
            }
            ExprInfo::FunctionCall(fun, params) => {
                let (pl1, fun) = fun.to_python_code();
                let mut pl2 = String::new();
                let mut args = Vec::new();
                for param in params {
                    let (pl, param) = param.to_python_code();
                    pl2.push_str(&pl);
                    args.push(param);
                }
                (pl1 + &pl2, format!("{}({})", fun, args.join(", ")))
            }
            ExprInfo::SystemVariable(name) => ("".to_string(), name.clone()),
        }
    }
}

fn indent(input: String) -> String {
    input
        .lines()
        .map(|line| format!("    {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

impl Line {
    fn to_python_code(&self) -> String {
        let (pl, line) = match self {
            Line::Expr(expr) => expr.to_python_code(),
            Line::Assign(id, expr, _) => {
                let (pl, expr) = expr.to_python_code();
                (pl, format!("{} = {}", id, expr))
            }
        };
        if pl.is_empty() {
            line
        } else {
            pl + "\n" + &line
        }
    }

    fn to_python_code_return(&self) -> (String, String) {
        let (pl, line, expr) = match self {
            Line::Expr(expr) => {
                let (pl, expr) = expr.to_python_code();
                (pl, expr.clone(), expr)
            }
            Line::Assign(id, expr, _) => {
                let (pl, expr) = expr.to_python_code();
                (pl, format!("{} = {}", id.clone(), expr), id.clone())
            }
        };
        if pl.is_empty() {
            (line, expr)
        } else {
            (pl + "\n" + &line, expr)
        }
    }
}
