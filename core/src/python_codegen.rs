use core::panic;
use std::collections::HashMap;

use crate::ast::{Expr, Line, Op, ReAssignmentExtension, StructFieldKind};

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
                let mut args = HashMap::new();
                for (name, expr) in params {
                    let (pl, expr) = expr.to_python_code();
                    pl2.push_str(&pl);
                    args.insert(name.clone(), expr);
                }
                (
                    pl1 + &pl2,
                    format!(
                        "{}({})",
                        fun,
                        args.iter()
                            .map(|(k, v)| format!("{}={}", k, v))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                )
            }
            Expr::Lambda(parameters, block, deps) => {
                let mut pl = String::new();
                let (pl_, parameters) = parameters_to_python_code(&parameters);
                pl.push_str(&pl_);
                pl.push_str("def func(");
                pl.push_str(&parameters);
                pl.push_str("):\n");
                for dep in deps {
                    if !dep.contains(".") {
                        pl.push_str(&format!("    global {}\n", dep));
                    }
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
            Expr::Struct(fields) => {
                let mut constructor_body = String::new();
                let mut parameters: Vec<String> = Vec::new();
                let mut methods = String::new();
                let mut pl1 = String::new();
                for (name, default, kind) in fields {
                    match kind {
                        StructFieldKind::Property => {
                            let parameter = if let Some(default) = default {
                                let (pl, default) = default.to_python_code();
                                pl1.push_str(&pl);
                                format!("{}={}", name, default)
                            } else {
                                name.clone()
                            };
                            parameters.push(parameter);
                            constructor_body
                                .push_str(&format!("        self.{} = {}\n", name, name));
                        }
                        StructFieldKind::Method => {
                            // let (pl, method) = default.as_ref().unwrap().to_python_code();
                            // pl1.push_str(&pl);
                            // methods.push_str(&(indent(method) + "\n"))
                            if let Some(Expr::Lambda(parameters, block, deps)) = default {
                                let mut method = String::new();
                                let (pl, parameters) = parameters_to_python_code(&parameters);
                                pl1.push_str(&pl);
                                method.push_str(&pl);
                                method.push_str("def ");
                                method.push_str(&name);
                                method.push_str("(self, ");
                                method.push_str(&parameters);
                                method.push_str("):\n");
                                for dep in deps {
                                    if !dep.contains(".") {
                                        method.push_str(&format!("    global {}\n", dep));
                                    }
                                }
                                let (block_pl, expr) = (*block).to_python_code();
                                method.push_str(&indent(block_pl));
                                method.push_str("\n");
                                method.push_str(&indent("return ".to_string() + &expr));
                                methods.push_str(&(indent(method) + "\n"));
                            } else {
                                panic!("Method must be a lambda");
                            }
                        }
                    }
                }
                let pl2 = format!(
                    "
class Struct:
    def __init__(self, {}):
{}
{}
",
                    parameters.join(", "),
                    constructor_body,
                    methods,
                );
                (pl1 + &pl2, "Struct".to_string())
            }
            Expr::GetProperty(expr, field) => {
                let (pl, expr) = expr.to_python_code();
                (pl, format!("{}.{}", expr, field))
            }
            Expr::Map(fields) => {
                let mut pl = String::new();
                let mut fields_str = Vec::new();
                for (key, value) in fields {
                    let (pl2, key) = key.to_python_code();
                    let (pl3, value) = value.to_python_code();
                    pl.push_str(&pl2);
                    pl.push_str(&pl3);
                    fields_str.push(format!("{}: {}", key, value));
                }
                (pl, format!("{{{}}}", fields_str.join(", ")))
            }
            Expr::Matrix(rows) => {
                let mut pl = String::new();
                let mut rows_str = Vec::new();
                for row in rows {
                    let mut row_str = Vec::new();
                    for item in row {
                        let (pl2, item) = item.to_python_code();
                        pl.push_str(&pl2);
                        row_str.push(item);
                    }
                    rows_str.push(format!("[{}]", row_str.join(", ")));
                }
                (pl, format!("np.matrix([{}])", rows_str.join(", ")))
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
            Line::NewAssignment(id, expr, _) => {
                let (pl, expr) = expr.to_python_code();
                (pl, format!("{} = {}", id, expr))
            }
            Line::ReAssignment(id, extensions, expr) => {
                let mut pl = id.clone();
                let extensions = extensions
                    .iter()
                    .map(|ext| match ext {
                        ReAssignmentExtension::Index(index) => {
                            let (pl_, index) = index.to_python_code();
                            pl.push_str(&pl_);
                            format!("[{}]", index)
                        }
                        ReAssignmentExtension::Property(field) => format!(".{}", field),
                    })
                    .collect::<Vec<_>>()
                    .join("");
                let (pl_, expr) = expr.to_python_code();
                pl.push_str(&pl_);
                (pl, format!("{}{} = {}", id, extensions, expr))
            }
        };
        if pl.is_empty() {
            line
        } else {
            pl + "\n" + &line
        }
    }

    fn to_python_code_return(&self) -> (String, String) {
        match &self {
            Line::Expr(_) => {
                let line = self.to_python_code();
                ("".to_string(), line)
            }
            Line::NewAssignment(_, _, _) | Line::ReAssignment(_, _, _) => {
                let line = self.to_python_code();
                (line, "None".to_string())
            }
        }
    }
}

fn parameters_to_python_code(parameters: &Vec<(String, Option<Expr>)>) -> (String, String) {
    let mut pl = String::new();
    let parameters = parameters
        .into_iter()
        .map(|(id, default_value)| {
            if let Some(default_value) = default_value {
                let (pl_, default_value) = default_value.to_python_code();
                pl.push_str(&pl_);
                format!("{}={}", id, default_value)
            } else {
                id.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    (pl, parameters)
}
