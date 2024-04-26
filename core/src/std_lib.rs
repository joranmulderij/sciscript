use crate::types::Type;

pub fn get_std_lib() -> (String, Vec<(String, String, Type)>) {
    let math_fun_type = || Type::Function(vec![Type::number()], Box::new(Type::number()), false);
    let variables = vec![
        ("sin".to_string(), "math.sin".to_string(), math_fun_type()),
        ("cos".to_string(), "math.cos".to_string(), math_fun_type()),
        ("tan".to_string(), "math.tan".to_string(), math_fun_type()),
        (
            "print".to_string(),
            "print".to_string(),
            Type::Function(vec![], Box::new(Type::Void), true),
        ),
        (
            "num".to_string(),
            "std.num".to_string(),
            Type::Type(Box::new(Type::number())),
        ),
        (
            "any".to_string(),
            "TODO".to_string(),
            Type::Type(Box::new(Type::Any)),
        ),
    ];
    let imports = "
import math
import std_lib as std
"
    .to_string();
    (imports, variables)
}
