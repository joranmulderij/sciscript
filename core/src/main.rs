mod ast;
mod parser;
mod python_codegen;
mod std_lib;
mod type_checking;
mod types;
mod units;

use std::io;

use crate::{parser::parse, type_checking::check_types, types::TypeContext};

fn main() -> io::Result<()> {
    let file = std::fs::read_to_string("fem.sci")?;
    let ast = match parse(&file) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };
    println!("{:?}", ast);
    let (imports, std_lib) = std_lib::get_std_lib();
    let mut type_context = TypeContext::new(std_lib);
    let (ast, _type, _deps) = match check_types(ast, &mut type_context) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };
    println!("{:?}", ast);

    let python_code = crate::python_codegen::generate_python_code(ast);
    let python_code = format!(
        "
{}
{}
    ",
        imports, python_code
    );
    std::fs::write("output.py", python_code)?;

    // println!("{}", "sdf".to_string() == "sdf".to_string());

    Ok(())
}
