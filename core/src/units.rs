use std::{
    collections::HashMap,
    ops::{Add, Mul, Sub},
};

pub fn parse_unit(input: &str) -> UnitSet {
    // TODO: Implement this function
    UnitSet::empty()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnitSet {
    units: HashMap<Unit, i64>,
}

impl Mul<i64> for UnitSet {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        let mut units = self.units;
        for (_, power) in units.iter_mut() {
            *power *= other;
        }
        Self { units }
    }
}

impl Add for UnitSet {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut units = self.units;
        for (unit, power) in other.units {
            let new_power = units.entry(unit.clone()).or_insert(0);
            *new_power += power;
            if *new_power == 0 {
                units.remove(&unit);
            }
        }
        Self { units }
    }
}

impl Sub for UnitSet {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut units = self.units;
        for (unit, power) in other.units {
            let new_power = units.entry(unit.clone()).or_insert(0);
            *new_power -= power;
            if *new_power == 0 {
                units.remove(&unit);
            }
        }
        Self { units }
    }
}

impl UnitSet {
    pub fn empty() -> Self {
        Self {
            units: HashMap::new(),
        }
    }

    pub fn single_unit(unit: Unit) -> Self {
        let mut units = HashMap::new();
        units.insert(unit, 1);
        Self { units }
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }
}

impl ToString for UnitSet {
    fn to_string(&self) -> String {
        let mut parts = Vec::new();
        for (unit, power) in self.units.iter() {
            if *power == 1 {
                parts.push(unit.name.to_string());
            } else {
                parts.push(format!("{}{}", unit.name, power));
            }
        }
        parts.join(" ")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Unit {
    pub name: String,
}
