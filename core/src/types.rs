use std::collections::HashMap;

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

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
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
}

pub struct Scope {
    pub variables: HashMap<String, (String, Type, bool)>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Number(UnitSet, Option<NumberConstant>),
    Function(Vec<Type>, Box<Type>),
    Range,
    Bool,
    Void,
}

impl Type {
    pub fn number() -> Self {
        Self::Number(UnitSet::empty(), None)
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
impl Eq for Type {}

#[derive(Debug, Clone)]
pub enum NumberConstant {
    Integer(i64),
    Float(f64),
}

impl ToString for NumberConstant {
    fn to_string(&self) -> String {
        match self {
            Self::Integer(n) => n.to_string(),
            Self::Float(n) => n.to_string(),
        }
    }
}
