use std::process::Command;

use parser::parse;
use python_codegen::generate_python_code;
use std_lib::get_std_lib;
use type_checking::check_types;
use types::TypeContext;

pub mod ast;
pub mod parser;
pub mod python_codegen;
pub mod std_lib;
pub mod type_checking;
pub mod types;
pub mod units;

pub fn run_code(code: &str) -> String {
    let ast = parse(code).unwrap();
    let (imports, std_lib) = get_std_lib();
    let mut type_context = TypeContext::new(std_lib);
    let (ast, _type, _deps) = check_types(ast, &mut type_context).unwrap();
    let python_code = generate_python_code(ast);
    let python_code = format!(
        "
{}
{}
    ",
        imports, python_code
    );
    let result = run_python(&python_code);
    return result.replace("\r\n", "\n").trim_end().to_owned();
}

fn run_python(code: &str) -> String {
    let f = Command::new("python")
        .args(["-c", code])
        .output()
        .expect("failed to execute process");
    String::from_utf8(f.stdout).unwrap()
}
