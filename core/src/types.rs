use std::collections::HashMap;

use crate::units::UnitSet;

pub struct TypeContext {
    scopes: Vec<Scope>,
    counter: u32,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext {
            scopes: vec![Scope::new()],
            counter: 0,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert_variable(&mut self, name: String, type_: Type) -> u32 {
        self.counter += 1;
        self.scopes
            .last_mut()
            .unwrap()
            .variables
            .insert(name, (self.counter, type_));
        self.counter
    }

    pub fn get_variable(&self, name: &str) -> Option<(&u32, &Type)> {
        for scope in self.scopes.iter().rev() {
            if let Some((id, type_)) = scope.variables.get(name) {
                return Some((id, type_));
            }
        }
        None
    }
}

pub struct Scope {
    pub variables: HashMap<String, (u32, Type)>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Number(UnitSet, Option<NumberConstant>),
    Range,
    Bool,
    Void,
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
