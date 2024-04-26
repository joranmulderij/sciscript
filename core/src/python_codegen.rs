use crate::ast::{Expr, Line, Op};

pub fn generate_python_code(ast: Vec<Line>) -> String {
    let mut python_code = String::new();
    for line in ast {
        python_code.push_str(&line.to_python_code());
        python_code.push_str("\n");
    }
    python_code
}

impl Expr {
    /// previous line code, expression code
    fn to_python_code(&self) -> (String, String) {
        match self {
            Expr::Number(n) => ("".to_string(), n.to_string()),
            Expr::UnaryMinus(expr) => {
                let code = expr.to_python_code();
                (code.0, format!("-{}", code.1))
            }
            Expr::Null => ("".to_string(), "None".to_string()),
            Expr::BinOp { lhs, op, rhs } => {
                let (pl1, lhs) = lhs.to_python_code();
                let (pl2, rhs) = rhs.to_python_code();
                if let Op::Range = op {
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
                        Op::Equals => "==",
                        Op::NotEquals => "!=",
                        Op::Range => unreachable!(),
                        // Op::LessThan => "<",
                    },
                    rhs
                );
                (pl1 + &pl2, expr)
            }
            Expr::List(items) => {
                let mut pl = String::new();
                let mut items_str = Vec::new();
                for item in items {
                    let (pl2, item) = item.to_python_code();
                    pl.push_str(&pl2);
                    items_str.push(item);
                }
                (pl, format!("[{}]", items_str.join(", ")))
            }
            Expr::Variable(id) => ("".to_string(), id.clone()),
            Expr::If(_conditions, _bodies, _else_) => {
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
            Expr::For(id, range, body) => {
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
            Expr::Boolean(b) => {
                if *b {
                    ("".to_string(), "True".to_string())
                } else {
                    ("".to_string(), "False".to_string())
                }
            }
            // Expr::Block(lines) => lines
            //     .iter()
            //     .map(|line| line.to_python_code())
            //     .collect::<Vec<_>>()
            //     .join("\n"),
            Expr::Block(body) => {
                let mut lines: Vec<String> = Vec::new();
                for i in 0..body.len() - 1 {
                    lines.push(body[i].to_python_code());
                }
                let (last_line, expr) = body.last().unwrap().to_python_code_return();
                lines.push(last_line);
                let pl = lines.join("\n");
                (pl, expr)
            }
            Expr::FunctionCall(fun, params) => {
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
            Expr::Lambda(parameters, block, deps, has_more_args) => {
                let mut pl = String::new();
                let mut parameters = parameters.clone();
                if *has_more_args {
                    let last = parameters.last_mut().unwrap();
                    *last = format!("*{}", last);
                }
                pl.push_str("def func(");
                pl.push_str(&parameters.join(", "));
                pl.push_str("):\n");
                for dep in deps {
                    pl.push_str(&format!("    global {}\n", dep));
                }
                let (block_pl, expr) = (*block).to_python_code();
                pl.push_str(&indent(block_pl));
                pl.push_str("\n");
                pl.push_str(&indent("return ".to_string() + &expr));
                (pl, "func".to_string())
            }
            Expr::Index(expr, index) => {
                let (pl1, expr) = expr.to_python_code();
                let (pl2, index) = index.to_python_code();
                (pl1 + &pl2, format!("{}[{}]", expr, index))
            }
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
                (pl, "".to_string(), expr)
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
