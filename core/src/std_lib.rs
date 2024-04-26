use crate::{
    types::{FunctionProfile, NumberConstant, Type},
    units::UnitSet,
};

pub fn get_std_lib() -> (String, Vec<(String, String, Type)>) {
    let num_type = Type::Function(FunctionProfile {
        parameters: vec![Type::number()],
        ret: Box::new(Type::number()),
        has_more_args: false,
    });
    let variables = vec![
        ("sin".to_string(), "math.sin".to_string(), num_type.clone()),
        ("cos".to_string(), "math.cos".to_string(), num_type.clone()),
        ("tan".to_string(), "math.tan".to_string(), num_type.clone()),
        (
            "print".to_string(),
            "print".to_string(),
            // Type::Function(vec![], Box::new(Type::Void), true),
            Type::Function(FunctionProfile {
                parameters: vec![],
                ret: Box::new(Type::Any),
                has_more_args: true,
            }),
        ),
        (
            "num".to_string(),
            "std.num".to_string(),
            Type::Type(
                number,
                Some(FunctionProfile {
                    parameters: vec![Type::Any],
                    ret: Box::new(Type::Number(UnitSet::empty(), None)),
                    has_more_args: false,
                }),
            ),
        ),
        (
            "any".to_string(),
            "std.any".to_string(),
            Type::Type(|_| Ok(Type::Any), None),
        ),
        (
            "list".to_string(),
            "std.list".to_string(),
            Type::Type(list, None),
        ),
        (
            "linspace".to_string(),
            "std.linspace".to_string(),
            Type::Function(FunctionProfile {
                parameters: vec![Type::number(), Type::number(), Type::number()],
                ret: Box::new(Type::List(Box::new(Type::number()))),
                has_more_args: false,
            }),
        ),
    ];
    let imports = "
import math
import std_lib as std
"
    .to_string();
    (imports, variables)
}

fn number(mut args: Vec<Type>) -> Result<Type, String> {
    if args.len() == 0 {
        return Ok(Type::Number(UnitSet::empty(), None));
    }
    if args.len() != 1 {
        return Err("num[] takes exactly one argument".to_string());
    }
    let arg = args.remove(0);
    let unit = match arg {
        Type::Number(unit, Some(NumberConstant::Integer(1))) => unit,
        Type::Number(unit, Some(NumberConstant::Float(f))) if f == 1.0 => unit,
        _ => return Err("num[] takes a number as argument".to_string()),
    };
    Ok(Type::Number(unit, None))
}

fn list(mut args: Vec<Type>) -> Result<Type, String> {
    if args.len() == 0 {
        return Ok(Type::List(Box::new(Type::Any)));
    }
    if args.len() != 1 {
        return Err("list[] takes exactly one argument".to_string());
    }
    let arg = args.remove(0);
    let type_ = match arg {
        Type::Type(fun, _) => fun(vec![])?,
        _ => return Err("list[] takes a type as argument".to_string()),
    };
    Ok(Type::List(Box::new(type_)))
}
