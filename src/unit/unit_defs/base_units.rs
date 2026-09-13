/// This file was generated automatically by the build script. 
/// If you want to add units, edit `unit_definitions\base_units.toml`
/// If you want to change file layout, edit `build.rs`

#[allow(unused)]
pub(crate) const NUMBER_OF_BASE_UNITS: usize = 8;

#[allow(unused)]
pub(crate) type UnitDefStatic = (&'static str, &'static str, f64, f64, [f64; NUMBER_OF_BASE_UNITS]);

#[rustfmt::skip]
pub(crate) const BASE_UNITS: &[UnitDefStatic] = &[
    ("USD", "USD", 0.0, 1.0, [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ]),
    ("ampere", "A", 0.0, 1.0, [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ]),
    ("candela", "cd", 0.0, 1.0, [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, ]),
    ("kelvin", "K", 0.0, 1.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, ]),
    ("kilogram", "kg", 0.0, 1.0, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, ]),
    ("meter", "m", 0.0, 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, ]),
    ("mole", "mol", 0.0, 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, ]),
    ("second", "s", 0.0, 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, ]),
];
