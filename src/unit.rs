use std::{
    collections::HashMap,
    fmt,
    ops::{Div, Mul},
};

mod single_unit;
mod unit_defs;

use single_unit::SingleUnit;
use unit_defs::{BASE_UNITS, search_for_unit_name};

const UNIT_SEP: &str = "."; // separate units within num/denom
const UNIT_DIV_SEP: &str = "/"; // separate numerator from denominator
const UNIT_FORMAT: (&str, &str) = ("[", "]"); // to come before and after unit
const UNIT_POW_IND: &str = ""; // indicates unit raised to a power

// so this will need everything you can do with just units:
// multiply, divide, pow
// coerce single unit to unit collection
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct UnitCollection {
    pub(crate) single_units: HashMap<SingleUnit, f64>,
    pub(crate) base_units: [f64; 8],
    pub(crate) scale: f64,
}

impl Eq for UnitCollection {}

impl Mul for UnitCollection {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // easy stuff first - base units and scale
        let mut prod_base_units = [0.0; 8];
        for i in 0..8 {
            prod_base_units[i] += self.base_units[i] + rhs.base_units[i]
        }
        let prod_scale = self.scale * rhs.scale;

        // iterate through both single units
        let mut prod_single_units = self.single_units.clone();

        for (key, &value) in rhs.single_units.iter() {
            if prod_single_units.contains_key(key) {
                *prod_single_units.get_mut(key).unwrap() += value;
            } else {
                prod_single_units.insert(key.clone(), value);
            }
            // if zero, just discard
            if prod_single_units[key] == 0.0 {
                prod_single_units.remove(key);
            }
        }

        UnitCollection {
            single_units: prod_single_units,
            base_units: prod_base_units,
            scale: prod_scale,
        }
    }
}

impl Div for UnitCollection {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // easy stuff first - base units and scale
        let mut quot_base_units = [0.0; 8];
        for i in 0..8 {
            quot_base_units[i] += self.base_units[i] - rhs.base_units[i]
        }
        let quot_scale = self.scale / rhs.scale;

        // iterate through both single units
        let mut quot_single_units = self.single_units.clone();

        for (key, &value) in rhs.single_units.iter() {
            if quot_single_units.contains_key(key) {
                *quot_single_units.get_mut(key).unwrap() -= value;
            } else {
                quot_single_units.insert(key.clone(), -value);
            }
            // if zero, just discard
            if quot_single_units[key] == 0.0 {
                quot_single_units.remove(key);
            }
        }

        UnitCollection {
            single_units: quot_single_units,
            base_units: quot_base_units,
            scale: quot_scale,
        }
    }
}

impl UnitCollection {
    pub(crate) fn offset(&self) -> Option<f64> {
        let mut keys = self.single_units.keys().collect::<Vec<&SingleUnit>>();
        if keys.len() != 1 {
            None
        } else if let Some(unit) = keys.pop() {
            Some(unit.offset)
        } else {
            None //unreachable but w/e
        }
    }

    // raise to a power
    pub(crate) fn pow(&self, exponent: f64) -> Self {
        let mut result_single_units = HashMap::new();

        for (k, v) in self.single_units.iter() {
            result_single_units.insert(k.clone(), v * exponent);
        }

        let mut result_base_units = self.base_units.clone();

        for i in 0..8 {
            result_base_units[i] *= exponent
        }

        UnitCollection {
            single_units: result_single_units,
            base_units: result_base_units,
            scale: self.scale.powf(exponent),
        }
    }

    pub(crate) fn coerce_unit_to_collection(unit: SingleUnit) -> Self {
        UnitCollection {
            base_units: unit.base_units,
            scale: unit.scale,
            single_units: HashMap::from([(unit, 1.0)]),
        }
    }

    pub(crate) fn empty_collection() -> Self {
        UnitCollection {
            base_units: [0.0; 8],
            scale: 1.0,
            single_units: HashMap::new(),
        }
    }

    pub(crate) fn from_unit_name(query: &str) -> Option<Self> {
        if let Some(single_unit) = search_for_unit_name(query) {
            Some(Self::coerce_unit_to_collection(single_unit))
        } else {
            let query = query.trim();

            // separate by numerator/denominator
            let mut num_denom: Vec<&str> = query.split(UNIT_DIV_SEP).collect();
            let (numerator, denominator) = match num_denom.len() {
                0 => return None,
                1 => (num_denom.remove(0), ""),
                2 => (num_denom.remove(0), num_denom.pop().unwrap()),
                _ => return None,
            };

            // instantiate result so we can just multiply them all together
            let mut result = UnitCollection::empty_collection();

            // go through numerator units
            for num_query in numerator.split(UNIT_SEP) {
                if num_query == "1" || num_query == "" {
                    continue;
                }

                // separate unit and exponent
                let (q, exp) = match num_query.rfind(char::is_numeric) {
                    Some(i) => num_query.split_at(i),
                    None => (num_query, "1"),
                };

                // parse exponent into number
                let exp = match exp.parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => continue,
                };

                match search_for_unit_name(q) {
                    Some(unit) => {
                        result = result * UnitCollection::coerce_unit_to_collection(unit).pow(exp)
                    }
                    None => continue,
                };
            }

            // go through denominator units
            for denom_query in denominator.split(UNIT_SEP) {
                if denom_query == "1" || denom_query == "" {
                    continue;
                }

                // separate unit and exponent
                let (q, exp) = match denom_query.rfind(char::is_numeric) {
                    Some(i) => denom_query.split_at(i),
                    None => (denom_query, "1"),
                };

                // parse exponent into number
                let exp = match exp.parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => continue,
                };

                match search_for_unit_name(q) {
                    Some(unit) => {
                        result = result / UnitCollection::coerce_unit_to_collection(unit).pow(exp)
                    }
                    None => continue,
                };
            }

            if result.single_units.len() == 0 {
                None
            } else {
                Some(result)
            }
        }
    }

    pub(crate) fn as_string(&self) -> String {
        let mut num = self
            .single_units
            .iter()
            .filter(|x| *x.1 > 0.0)
            .map(|x| {
                if *x.1 != 1.0 {
                    x.0.abbr.clone() + UNIT_POW_IND + &x.1.to_string()
                } else {
                    x.0.abbr.clone()
                }
            })
            .collect::<Vec<String>>();
        num.sort();
        let num = num.join(UNIT_SEP);

        let mut denom = self
            .single_units
            .iter()
            .filter(|x| *x.1 < 0.0)
            .map(|x| {
                if *x.1 != -1.0 {
                    x.0.abbr.clone() + UNIT_POW_IND + &(-x.1).to_string()
                } else {
                    x.0.abbr.clone()
                }
            })
            .collect::<Vec<String>>();
        denom.sort();
        let denom = denom.join(UNIT_SEP);

        if denom != "" {
            if num == "" {
                return format!(
                    "{}1{}{}{}",
                    UNIT_FORMAT.0, UNIT_DIV_SEP, denom, UNIT_FORMAT.1
                );
            } else {
                return format!(
                    "{}{}{}{}{}",
                    UNIT_FORMAT.0, num, UNIT_DIV_SEP, denom, UNIT_FORMAT.1
                );
            }
        } else {
            return format!("{}{}{}", UNIT_FORMAT.0, num, UNIT_FORMAT.1);
        }
    }

    pub(crate) fn as_descriptive_string(&self) -> String {
        let mut num = self
            .single_units
            .iter()
            .filter(|x| *x.1 > 0.0)
            .map(|x| {
                if *x.1 != 1.0 {
                    x.0.name.clone() + UNIT_POW_IND + &x.1.to_string()
                } else {
                    x.0.name.clone()
                }
            })
            .collect::<Vec<String>>();
        num.sort();
        let num = num.join(UNIT_SEP);

        let mut denom = self
            .single_units
            .iter()
            .filter(|x| *x.1 < 0.0)
            .map(|x| {
                if *x.1 != -1.0 {
                    x.0.name.clone() + UNIT_POW_IND + &(-x.1).to_string()
                } else {
                    x.0.name.clone()
                }
            })
            .collect::<Vec<String>>();
        denom.sort();
        let denom = denom.join(UNIT_SEP);

        if denom != "" {
            if num == "" {
                return format!(
                    "{}1{}{}{}",
                    UNIT_FORMAT.0, UNIT_DIV_SEP, denom, UNIT_FORMAT.1
                );
            } else {
                return format!(
                    "{}{}{}{}{}",
                    UNIT_FORMAT.0, num, UNIT_DIV_SEP, denom, UNIT_FORMAT.1
                );
            }
        } else {
            return format!("{}{}{}", UNIT_FORMAT.0, num, UNIT_FORMAT.1);
        }
    }

    pub(crate) fn equivalent_scale_diff(&self, other: &Self) -> Option<ScaleDiff> {
        if self.base_units != other.base_units {
            None
        } else {
            Some(ScaleDiff {
                self_scale: self.scale,
                other_scale: other.scale,
            })
        }
    }

    pub(crate) fn get_base_units(&self) -> UnitCollection {
        let mut base_unit_hashmap = HashMap::new();

        for i in 0..8 {
            base_unit_hashmap.insert(
                SingleUnit::create_from_unit_def(BASE_UNITS[i]),
                self.base_units[i],
            );
        }

        UnitCollection {
            single_units: base_unit_hashmap,
            base_units: self.base_units,
            scale: 1.0,
        }
    }

    pub(crate) fn to_single_unit(self, name: String, abbr: String) -> SingleUnit {
        SingleUnit {
            name,
            abbr,
            scale: self.scale,
            offset: 0.0,
            base_units: self.base_units,
        }
    }
}

impl fmt::Display for UnitCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())?;

        Ok(())
    }
}

pub(crate) struct ScaleDiff {
    self_scale: f64,
    other_scale: f64,
}

impl Mul<ScaleDiff> for f64 {
    type Output = f64;

    fn mul(self, rhs: ScaleDiff) -> f64 {
        self * rhs.other_scale / rhs.self_scale
    }
}

impl Div<ScaleDiff> for f64 {
    type Output = f64;

    fn div(self, rhs: ScaleDiff) -> f64 {
        self * rhs.self_scale / rhs.other_scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_collection() {
        let empty = UnitCollection::empty_collection();
        assert_eq!(format!("{}", empty), "[]");
    }

    #[test]
    fn from_single_unit() {
        let joule = UnitCollection::from_unit_name("joule").unwrap();
        assert_eq!(format!("{}", joule), "[J]");
    }

    #[test]
    fn test_equality() {
        let joule = UnitCollection::from_unit_name("joule").unwrap();
        let joule_copy = UnitCollection::from_unit_name("joule").unwrap();
        assert_eq!(joule, joule_copy);
    }

    #[test]
    fn test_inequality() {
        let joule = UnitCollection::from_unit_name("joule").unwrap();
        let meter = UnitCollection::from_unit_name("meter").unwrap();

        assert_ne!(joule, meter);
    }

    #[test]
    fn test_mul_div() {
        let joule = UnitCollection::from_unit_name("joule").unwrap();
        let meter = UnitCollection::from_unit_name("meter").unwrap();

        let joule_meter = joule.clone() * meter.clone();
        assert_eq!(format!("{}", joule_meter), "[J.m]");

        let joule_meter = joule / meter;
        assert_eq!(format!("{}", joule_meter), "[J/m]");
    }

    #[test]
    fn test_pow() {
        let joule = UnitCollection::from_unit_name("joule").unwrap();
        let meter = UnitCollection::from_unit_name("meter").unwrap();

        let joule2 = joule.clone().pow(2.0);
        assert_eq!(format!("{}", joule2), "[J2]");

        let joule_meter2 = (joule / meter).pow(2.0);
        assert_eq!(format!("{}", joule_meter2), "[J2/m2]");
    }
}
