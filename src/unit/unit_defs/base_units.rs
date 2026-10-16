/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\base_units.csv`
/// If you want to change file layout, edit `build.rs`
use crate::num::smallrational::SmallRational as SR;

pub(crate) const NUMBER_OF_BASE_UNITS: usize = 8;

#[rustfmt::skip]
pub(crate) type UnitDefStatic = (&'static str, &'static str, f64, f64, [&'static SR; NUMBER_OF_BASE_UNITS]);

#[rustfmt::skip]
pub(crate) const BASE_UNITS: &[UnitDefStatic] = &[
    ("second", "s", 0.0, 1.0, [&SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("meter", "m", 0.0, 1.0, [&SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("kilogram", "kg", 0.0, 1.0, [&SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("ampere", "A", 0.0, 1.0, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("kelvin", "K", 0.0, 1.0, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("mole", "mol", 0.0, 1.0, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, ]),
    ("candela", "cd", 0.0, 1.0, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, ]),
    ("USD", "USD", 0.0, 1.0, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, ]),
];
