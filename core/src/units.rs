use std::collections::HashMap;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

#[derive(Debug)]
pub struct Unit {
    name: &'static str,
    dimensions: DimensionProfile,
}

impl Unit {
    const METER: Unit = Unit {
        name: "meter",
        dimensions: DimensionProfile::LENGTH,
    };
    const KILOGRAM: Unit = Unit {
        name: "kilogram",
        dimensions: DimensionProfile::MASS,
    };
    const SECOND: Unit = Unit {
        name: "second",
        dimensions: DimensionProfile::TIME,
    };
    const NEWTON: Unit = Unit {
        name: "newton",
        dimensions: DimensionProfile::FORCE,
    };
}

#[derive(Debug, PartialEq, Eq)]
struct DimensionProfile {
    mass: i8,
    length: i8,
    time: i8,
    current: i8,
    temperature: i8,
    amount: i8,
    luminosity: i8,
}

impl DimensionProfile {
    const fn new(
        mass: i8,
        length: i8,
        time: i8,
        current: i8,
        temperature: i8,
        amount: i8,
        luminosity: i8,
    ) -> Self {
        Self {
            mass,
            length,
            time,
            current,
            temperature,
            amount,
            luminosity,
        }
    }
    const LENGTH: DimensionProfile = DimensionProfile::new(0, 1, 0, 0, 0, 0, 0);
    const MASS: DimensionProfile = DimensionProfile::new(1, 0, 0, 0, 0, 0, 0);
    const TIME: DimensionProfile = DimensionProfile::new(0, 0, 1, 0, 0, 0, 0);
    const CURRENT: DimensionProfile = DimensionProfile::new(0, 0, 0, 1, 0, 0, 0);
    const TEMPERATURE: DimensionProfile = DimensionProfile::new(0, 0, 0, 0, 1, 0, 0);
    const AMOUNT: DimensionProfile = DimensionProfile::new(0, 0, 0, 0, 0, 1, 0);
    const LUMINOSITY: DimensionProfile = DimensionProfile::new(0, 0, 0, 0, 0, 0, 1);

    const FORCE: DimensionProfile = DimensionProfile::new(1, 1, -2, 0, 0, 0, 0);
}
