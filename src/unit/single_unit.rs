use std::hash::{Hash, Hasher};

use crate::unit::unit_defs::NUMBER_OF_BASE_UNITS;
use crate::unit::unit_defs::UnitDefStatic;

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct SingleUnit {
    pub(crate) name: String,
    pub(crate) abbr: String,
    pub(crate) scale: f64,
    pub(crate) offset: f64,
    pub(crate) base_units: [f64; NUMBER_OF_BASE_UNITS],
}

impl Hash for SingleUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Eq for SingleUnit {}

// name, abbr, offset (in and from base units), scale, (exponents) second, meter, kilogram, ampere, kelvin, mole, candela, USD
// TO DO: move to unit_defs, incorporate number_of_base_units

pub(crate) type UnitDefString = (String, String, f64, f64, [f64; NUMBER_OF_BASE_UNITS]);

impl SingleUnit {
    pub(crate) fn create_from_unit_def_string(u: UnitDefString) -> Self {
        SingleUnit {
            name: u.0,
            abbr: u.1,
            offset: u.2,
            scale: u.3,
            base_units: u.4,
        }
    }

    pub(crate) fn create_from_unit_def_static(u: &UnitDefStatic) -> Self {
        SingleUnit {
            name: u.0.to_string(),
            abbr: u.1.to_string(),
            offset: u.2,
            scale: u.3,
            base_units: u.4.into(),
        }
    }
}
