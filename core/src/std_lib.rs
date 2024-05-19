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
            "pow".to_string(),
            "math.pow".to_string(),
            Type::Function(FunctionProfile {
                parameters: vec![
                    ("base".to_string(), Type::number(), true),
                    ("exp".to_string(), Type::number(), true),
                ],
                return_type: Box::new(Type::number()),
            }),
        ),
        (
            "atan2".to_string(),
            "math.atan2".to_string(),
            Type::Function(FunctionProfile {
                parameters: vec![
                    ("a".to_string(), Type::number(), true),
                    ("b".to_string(), Type::number(), true),
                ],
                return_type: Box::new(Type::number()),
            }),
        ),
        (
            "cross".to_string(),
            "np.cross".to_string(),
            Type::Function(FunctionProfile {
                parameters: vec![
                    ("a".to_string(), Type::Matrix(3, 1, None), true),
                    ("b".to_string(), Type::Matrix(3, 1, None), true),
                ],
                return_type: Box::new(Type::Matrix(3, 1, None)),
            }),
        ),
        ("abs".to_string(), "abs".to_string(), num_type.clone()),
        ("log".to_string(), "math.log".to_string(), num_type.clone()),
        ("exp".to_string(), "math.exp".to_string(), num_type.clone()),
        ("pi".to_string(), "math.pi".to_string(), Type::number()),
        ("e".to_string(), "math.e".to_string(), Type::number()),
        (
            "sqrt".to_string(),
            "math.sqrt".to_string(),
            num_type.clone(),
        ),
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
            "map".to_string(),
            "std.map".to_string(),
            Type::Type(TypeProfile::Function(map), None),
        ),
        (
            "mat".to_string(),
            "std.mat".to_string(),
            Type::Type(TypeProfile::Function(mat), None),
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
import numpy as np
import sympy as sp
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

fn map(mut args: Vec<Type>) -> Result<Type, String> {
    if args.len() == 0 {
        return Ok(Type::Map(Box::new(Type::Any), Box::new(Type::Any)));
    }
    if args.len() != 2 {
        return Err("map[] takes exactly two arguments".to_string());
    }
    let key = args.remove(0);
    let value = args.remove(0);
    let key = match key {
        Type::Type(profile, _) => match profile {
            TypeProfile::Function(fun) => fun(vec![])?,
            TypeProfile::Type(t) => *t,
        },
        _ => return Err("map[] takes a type as key argument".to_string()),
    };
    let value = match value {
        Type::Type(profile, _) => match profile {
            TypeProfile::Function(fun) => fun(vec![])?,
            TypeProfile::Type(t) => *t,
        },
        _ => return Err("map[] takes a type as value argument".to_string()),
    };
    Ok(Type::Map(Box::new(key), Box::new(value)))
}

fn mat(mut args: Vec<Type>) -> Result<Type, String> {
    if args.len() != 3 && args.len() != 2 {
        return Err("mat[] takes exactly three arguments".to_string());
    }
    let rows = match args.remove(0) {
        Type::Number(_, Some(NumberConstant::Integer(i))) => i,
        _ => return Err("mat[] takes a number as first argument".to_string()),
    };
    let cols = match args.remove(0) {
        Type::Number(_, Some(NumberConstant::Integer(i))) => i,
        _ => return Err("mat[] takes a number as second argument".to_string()),
    };
    let unit = if args.len() != 0 {
        Some(type_to_unit_set(args.remove(0))?)
    } else {
        None
    };
    Ok(Type::Matrix(rows as usize, cols as usize, unit))
}

fn type_to_unit_set(type_: Type) -> Result<UnitSet, String> {
    Ok(match type_ {
        Type::Number(unit, Some(NumberConstant::Integer(1))) => unit,
        Type::Number(unit, Some(NumberConstant::Float(f))) if f == 1.0 => unit,
        _ => return Err("Expected a unit".to_string()),
    })
}
