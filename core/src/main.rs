mod ast;
mod parser;
mod python_codegen;
mod type_checking;
mod types;
mod units;

use std::io;

use crate::{parser::parse, type_checking::check_types};

fn main() -> io::Result<()> {
    let file = std::fs::read_to_string("input.sci")?;
    let ast = match parse(&file) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };
    println!("{:?}", ast);
    let (_, ast2) = match check_types(ast) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };
    println!("{:?}", ast2);
    // let mut python_code = String::new();
    // match ast {
    //     Ok(ast) => {
    //         for line in ast {
    //             python_code.push_str(&line.to_python());
    //             python_code.push_str("\n");
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("{}", e);
    //     }
    // }
    // std::fs::write("output.py", python_code)?;
    Ok(())
}
