use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::units::UnitSet;

pub struct TypeContext {
    scopes: Vec<Scope>,
    counter: usize,
}

impl TypeContext {
    pub fn new(std_lib: Vec<(String, String, Type)>) -> Self {
        let mut variables = HashMap::new();
        for (name, py_name, type_) in std_lib {
            variables.insert(name, (py_name, type_, true));
        }

        TypeContext {
            scopes: vec![Scope { variables }],
            counter: 1,
        }
    }

    pub fn push_scope(&mut self) {
        let variables = HashMap::new();
        self.scopes.push(Scope { variables });
    }

    pub fn pop_scope(&mut self) -> HashSet<String> {
        let mut variables = HashSet::new();
        for (_, (id, _, _)) in self.scopes.last().unwrap().variables.iter() {
            variables.insert(id.clone());
        }
        self.scopes.pop();
        variables
    }

    pub fn insert_variable(&mut self, name: String, type_: Type, const_: bool) -> String {
        let type_ = match type_ {
            Type::Number(unit_set, _) if !const_ => Type::Number(unit_set, None),
            type_ => type_,
        };
        self.counter += 1;
        let id = format!("var_{}", self.counter);
        self.scopes
            .last_mut()
            .unwrap()
            .variables
            .insert(name, (id.clone(), type_, const_));
        id
    }

    pub fn get_variable(&self, name: &str) -> Option<&(String, Type, bool)> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.variables.get(name) {
                return Some(var);
            }
        }
        None
    }

    // pub fn print_last_scope(&self) {
    //     for (name, (id, type_, const_)) in self.scopes.last().unwrap().variables.iter() {
    //         println!("{}: {} {:?} {}", name, id, type_, const_);
    //     }
    // }
}

pub struct Scope {
    pub variables: HashMap<String, (String, Type, bool)>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Number(UnitSet, Option<NumberConstant>),
    Function(Vec<Type>, Box<Type>, bool), // bool is for whether the function has a last "args" parameter
    Type(Box<Type>),
    List(Box<Type>),
    Range,
    Bool,
    Void,
    Any,
}

impl Type {
    pub fn number() -> Self {
        Self::Number(UnitSet::empty(), None)
    }

    pub fn can_be_assigned_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(unit1, _), Self::Number(unit2, _)) if unit1 == unit2 => true,
            (
                Self::Function(args1, ret1, has_last_args1),
                Self::Function(args2, ret2, has_last_args2),
            ) => {
                let arguments_match =
                    Self::arguments_match_parameters(args1, has_last_args1, args2);
                let returns_match = ret1.can_be_assigned_to(ret2.as_ref());
                let last_args_match = *has_last_args1 && !has_last_args2;
                arguments_match && returns_match && last_args_match
            }
            (Self::Type(t1), Self::Type(t2)) => t1.can_be_assigned_to(t2),
            (Self::List(t1), Self::List(t2)) => t1.can_be_assigned_to(t2),
            (Self::Bool, Self::Bool) => true,
            (Self::Void, Self::Void) => true,
            (Self::Any, _) => true,
            _ => false,
        }
    }

    pub fn arguments_match_parameters(
        arguments: &Vec<Type>,
        has_more_args: &bool,
        parameters: &Vec<Type>,
    ) -> bool {
        if arguments.len() < parameters.len() {
            return false;
        }
        if !has_more_args && arguments.len() > parameters.len() {
            return false;
        }
        for i in 0..parameters.len() {
            if !arguments[i].can_be_assigned_to(&parameters[i]) {
                return false;
            }
        }
        true
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0, _), Self::Number(r0, _)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NumberConstant {
    Integer(i64),
    Float(f64),
}

impl Add for NumberConstant {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l + r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l + r),
            (Self::Integer(l), Self::Float(r)) => Self::Float(l as f64 + r),
            (Self::Float(l), Self::Integer(r)) => Self::Float(l + r as f64),
        }
    }
}

impl Sub for NumberConstant {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l - r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l - r),
            (Self::Integer(l), Self::Float(r)) => Self::Float(l as f64 - r),
            (Self::Float(l), Self::Integer(r)) => Self::Float(l - r as f64),
        }
    }
}

impl Mul for NumberConstant {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l * r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l * r),
            (Self::Integer(l), Self::Float(r)) => Self::Float(l as f64 * r),
            (Self::Float(l), Self::Integer(r)) => Self::Float(l * r as f64),
        }
    }
}

impl Div for NumberConstant {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            // (Self::Integer(1), Self::Integer(1)) => Self::Integer(1),
            (Self::Integer(l), Self::Integer(r)) => Self::Float(l as f64 / r as f64),
            (Self::Float(l), Self::Float(r)) => Self::Float(l / r),
            (Self::Integer(l), Self::Float(r)) => Self::Float(l as f64 / r),
            (Self::Float(l), Self::Integer(r)) => Self::Float(l / r as f64),
        }
    }
}

impl Rem for NumberConstant {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l % r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l % r),
            (Self::Integer(l), Self::Float(r)) => Self::Float(l as f64 % r),
            (Self::Float(l), Self::Integer(r)) => Self::Float(l % r as f64),
        }
    }
}

impl NumberConstant {
    pub fn pow(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Float((*l as f64).powi(*r as i32)),
            (Self::Float(l), Self::Float(r)) => Self::Float(l.powf(*r)),
            (Self::Integer(l), Self::Float(r)) => Self::Float((*l as f64).powf(*r)),
            (Self::Float(l), Self::Integer(r)) => Self::Float(l.powi(*r as i32)),
        }
    }
}

impl ToString for NumberConstant {
    fn to_string(&self) -> String {
        match self {
            Self::Integer(n) => n.to_string(),
            Self::Float(n) => n.to_string(),
        }
    }
}
