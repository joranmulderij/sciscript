use crate::{
    types::{FunctionProfile, NumberConstant, Type, TypeProfile},
    units::UnitSet,
};

pub fn get_std_lib() -> (String, Vec<(String, String, Type)>) {
    let num_type = Type::Function(FunctionProfile {
        parameters: vec![("value".to_string(), Type::number(), true)],
        return_type: Box::new(Type::number()),
    });
    let variables = vec![
        ("sin".to_string(), "math.sin".to_string(), num_type.clone()),
        ("cos".to_string(), "math.cos".to_string(), num_type.clone()),
        ("tan".to_string(), "math.tan".to_string(), num_type.clone()),
        (
            "print".to_string(),
            "std.my_print".to_string(),
            Type::Function(FunctionProfile {
                parameters: vec![("value".to_string(), Type::Any, true)],
                return_type: Box::new(Type::Any),
            }),
        ),
        (
            "num".to_string(),
            "std.num".to_string(),
            Type::Type(
                TypeProfile::Function(number),
                Some(FunctionProfile {
                    parameters: vec![("value".to_string(), Type::Any, true)],
                    return_type: Box::new(Type::Number(UnitSet::empty(), None)),
                }),
            ),
        ),
        (
            "any".to_string(),
            "std.any".to_string(),
            Type::Type(TypeProfile::Type(Box::new(Type::Any)), None),
        ),
        (
            "bool".to_string(),
            "std.bool".to_string(),
            Type::Type(TypeProfile::Type(Box::new(Type::Bool)), None),
        ),
        (
            "list".to_string(),
            "std.list".to_string(),
            Type::Type(TypeProfile::Function(list), None),
        ),
        (
            "linspace".to_string(),
            "std.linspace".to_string(),
            Type::Function(FunctionProfile {
                parameters: vec![
                    ("start".to_string(), Type::number(), true),
                    ("stop".to_string(), Type::number(), true),
                    ("n".to_string(), Type::number(), true),
                ],
                return_type: Box::new(Type::List(Box::new(Type::number()))),
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
        Type::Type(profile, _) => match profile {
            TypeProfile::Function(fun) => fun(vec![])?,
            TypeProfile::Type(t) => *t,
        },
        _ => return Err("list[] takes a type as argument".to_string()),
    };
    Ok(Type::List(Box::new(type_)))
}
