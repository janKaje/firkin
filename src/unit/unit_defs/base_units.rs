/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\base_units.csv`
/// If you want to change file layout, edit `build.rs`
use crate::num::{number::NumberWrapper as NW, smallrational::SmallRational as SR};

pub(crate) const NUMBER_OF_BASE_UNITS: usize = 8;

#[rustfmt::skip]
pub(crate) type UnitDefStatic = (&'static str, &'static str, &'static NW, &'static NW, [&'static SR; NUMBER_OF_BASE_UNITS]);

#[rustfmt::skip]
pub(crate) const BASE_UNITS: &[UnitDefStatic] = &[
    ("second", "s", &NW::ZERO, &NW::ONE, [&SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("meter", "m", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("kilogram", "kg", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("ampere", "A", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("kelvin", "K", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, &SR::ZERO, ]),
    ("mole", "mol", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, &SR::ZERO, ]),
    ("candela", "cd", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, &SR::ZERO, ]),
    ("USD", "USD", &NW::ZERO, &NW::ONE, [&SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ZERO, &SR::ONE, ]),
];
