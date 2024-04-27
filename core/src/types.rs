use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::units::UnitSet;

#[derive(Debug, Clone)]
pub enum Type {
    Number(UnitSet, Option<NumberConstant>),
    Function(FunctionProfile),
    Type(TypeProfile, Option<FunctionProfile>),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Matrix(usize, usize, Option<UnitSet>),
    Range,
    Bool,
    Void,
    Any,
    Struct(Vec<(String, Type, bool)>),
}

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

    pub fn insert_variable(
        &mut self,
        name: String,
        id: Option<String>,
        type_: Type,
        const_: bool,
    ) -> String {
        let type_ = match type_ {
            Type::Number(unit_set, _) if !const_ => Type::Number(unit_set, None),
            type_ => type_,
        };
        let id = match id {
            Some(id) => id,
            None => {
                self.counter += 1;
                format!("var_{}", self.counter)
            }
        };
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
}

pub struct Scope {
    pub variables: HashMap<String, (String, Type, bool)>,
}

#[derive(Debug, Clone)]
pub struct FunctionProfile {
    pub parameters: Vec<(String, Type, bool)>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub enum TypeProfile {
    Function(fn(Vec<Type>) -> Result<Type, String>),
    Type(Box<Type>),
}

impl Type {
    pub fn number() -> Self {
        Self::Number(UnitSet::empty(), None)
    }

    pub fn can_be_assigned_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(unit1, _), Self::Number(unit2, _)) if unit1 == unit2 => true,
            (
                Self::Function(FunctionProfile {
                    parameters: parameters1,
                    return_type: ret1,
                }),
                Self::Function(FunctionProfile {
                    parameters: parameters2,
                    return_type: ret2,
                }),
            ) => {
                parameters1.len() == parameters2.len()
                    && parameters1
                        .iter()
                        .zip(parameters2.iter())
                        .all(|((_, t1, _), (_, t2, _))| t1.can_be_assigned_to(t2))
                    && ret1.can_be_assigned_to(ret2)
            }
            (Self::Type(_t1, _profile1), Self::Type(_t2, _profile2)) => todo!(),
            (Self::List(t1), Self::List(t2)) => t1.can_be_assigned_to(t2),
            (Self::Matrix(rows1, cols1, unit1), Self::Matrix(rows2, cols2, unit2)) => {
                (unit1.is_none() || unit1 == unit2) && rows1 == rows2 && cols1 == cols2
            }
            (Self::Map(k1, v1), Self::Map(k2, v2)) => {
                k1.can_be_assigned_to(k2) && v1.can_be_assigned_to(v2)
            }
            (Self::Bool, Self::Bool) => true,
            (Self::Void, Self::Void) => true,
            (Self::Any, _) => true,
            (Self::Struct(fields1), Self::Struct(fields2)) => {
                fields1.len() == fields2.len()
                    && fields1.iter().zip(fields2.iter()).all(
                        |((name1, type1, _), (name2, type2, _))| {
                            name1 == name2 && type1.can_be_assigned_to(type2)
                        },
                    )
            }
            _ => false,
        }
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
