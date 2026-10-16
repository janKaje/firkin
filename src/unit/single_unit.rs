use std::hash::{Hash, Hasher};

use crate::{
    num::smallrational::SmallRational,
    unit::unit_defs::{NUMBER_OF_BASE_UNITS, PrefixDef, UnitDefStatic},
};

// TODO: rewrite impl of prefixes so uses less memory, is faster to clone?
// being able to implement copy would be incredibly helpful for simplify speed i think
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct StaticSingleUnit {
    pub(crate) name: &'static str,
    pub(crate) abbr: &'static str,
    pub(crate) prefix: Option<PrefixDef>,
    pub(crate) scale: f64,
    pub(crate) offset: f64,
    pub(crate) base_units: [&'static SmallRational; NUMBER_OF_BASE_UNITS],
}

impl Hash for StaticSingleUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.abbr.hash(state);
    }
}

impl Eq for StaticSingleUnit {}

// name, abbr, offset (in and from base units), scale, (exponents) second, meter, kilogram, ampere, kelvin, mole, candela, USD
// TO DO: move to unit_defs, incorporate number_of_base_units

impl StaticSingleUnit {
    pub(crate) const fn create_from_unit_def(
        u: &'static UnitDefStatic,
        prefix: Option<PrefixDef>,
    ) -> Self {
        StaticSingleUnit {
            name: u.0,
            abbr: u.1,
            prefix,
            offset: u.2,
            scale: u.3,
            base_units: u.4,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct OwnedSingleUnit {
    pub(crate) name: String,
    pub(crate) abbr: String,
    pub(crate) scale: f64,
    pub(crate) base_units: [SmallRational; NUMBER_OF_BASE_UNITS],
}

impl Hash for OwnedSingleUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.abbr.hash(state);
    }
}

impl OwnedSingleUnit {
    pub(crate) const fn new(
        name: String,
        abbr: String,
        scale: f64,
        base_units: [SmallRational; NUMBER_OF_BASE_UNITS],
    ) -> Self {
        OwnedSingleUnit {
            name,
            abbr,
            scale,
            base_units,
        }
    }
}

impl Eq for OwnedSingleUnit {}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub(crate) enum SingleUnit {
    Static(StaticSingleUnit),
    Owned(OwnedSingleUnit),
}

impl SingleUnit {
    pub(crate) const fn new_static(u: &'static UnitDefStatic, prefix: Option<PrefixDef>) -> Self {
        Self::Static(StaticSingleUnit::create_from_unit_def(u, prefix))
    }

    pub(crate) const fn new_owned(
        name: String,
        abbr: String,
        scale: f64,
        base_units: [SmallRational; NUMBER_OF_BASE_UNITS],
    ) -> Self {
        Self::Owned(OwnedSingleUnit::new(name, abbr, scale, base_units))
    }

    pub(crate) fn name(&self) -> String {
        match self {
            SingleUnit::Static(static_single_unit) => match static_single_unit.prefix {
                Some(prefix) => String::from(prefix.0) + static_single_unit.name,
                None => static_single_unit.name.into(),
            },
            SingleUnit::Owned(owned_single_unit) => owned_single_unit.name.clone(),
        }
    }

    pub(crate) fn abbr(&self) -> String {
        match self {
            SingleUnit::Static(static_single_unit) => match static_single_unit.prefix {
                Some(prefix) => String::from(prefix.1) + static_single_unit.abbr,
                None => static_single_unit.abbr.into(),
            },
            SingleUnit::Owned(owned_single_unit) => owned_single_unit.abbr.clone(),
        }
    }

    pub(crate) fn offset(&self) -> f64 {
        match self {
            SingleUnit::Static(static_single_unit) => static_single_unit.offset,
            SingleUnit::Owned(_) => 0.0,
        }
    }

    pub(crate) fn scale(&self) -> f64 {
        match self {
            SingleUnit::Static(static_single_unit) => match static_single_unit.prefix {
                Some(prefix) => static_single_unit.scale * prefix.2,
                None => static_single_unit.scale,
            },
            SingleUnit::Owned(owned_single_unit) => owned_single_unit.scale,
        }
    }

    pub(crate) fn base_units(&self) -> [&SmallRational; NUMBER_OF_BASE_UNITS] {
        match self {
            SingleUnit::Static(static_single_unit) => static_single_unit.base_units,
            SingleUnit::Owned(owned_single_unit) => owned_single_unit.base_units.each_ref(),
        }
    }
}
