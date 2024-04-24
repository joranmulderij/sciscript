use crate::types::Type;

pub fn get_std_lib() -> (String, Vec<(String, String, Type)>) {
    let math_fun_type = || Type::Function(vec![Type::number()], Box::new(Type::number()));
    let variables = vec![
        ("sin".to_string(), "math.sin".to_string(), math_fun_type()),
        ("cos".to_string(), "math.cos".to_string(), math_fun_type()),
        ("tan".to_string(), "math.tan".to_string(), math_fun_type()),
        (
            "print".to_string(),
            "print".to_string(),
            Type::Function(vec![Type::number()], Box::new(Type::Void)),
        ),
        (
            "int".to_string(),
            "int".to_string(),
            Type::Type(Box::new(Type::number())),
        ),
        (
            "num".to_string(),
            "float".to_string(),
            Type::Type(Box::new(Type::number())),
        ),
    ];
    let imports = "
import math
"
    .to_string();
    (imports, variables)
}
