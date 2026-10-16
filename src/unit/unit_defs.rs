// #![allow(unused)]

mod aliases;
mod base_units;
mod derived_units;
mod log_units;

use gmp_mpfr_sys::gmp::{self, mpq_t};

use log_units::LOG_UNITS;
use rug::{Integer, Rational};

use crate::num::{
    int::IntegerWrapper, number::NumberWrapper, rat::RationalWrapper, smallrational::SmallRational,
};

pub(crate) use super::log_unit::LogUnit;
use super::single_unit::StaticSingleUnit;
pub(crate) use base_units::UnitDefStatic;

pub(crate) use aliases::UNIT_ALIASES;
#[allow(unused)]
pub(crate) use base_units::{BASE_UNITS, NUMBER_OF_BASE_UNITS};
pub(crate) use derived_units::OTHER_UNITS;

// prefix, abbr, multiplier
pub(super) type PrefixDef = (&'static str, &'static str, &'static NumberWrapper);

static STANDARD_PREFIXES: &[PrefixDef] = &[
    (
        "kibi",
        "Ki",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1 << 10)),
    ),
    (
        "mebi",
        "Mi",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1 << 20)),
    ),
    (
        "gibi",
        "Gi",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1 << 30)),
    ),
    (
        "tebi",
        "Ti",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1i64 << 40)),
    ),
    (
        "pebi",
        "Pi",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1i64 << 50)),
    ),
    (
        "exbi",
        "Ei",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1i64 << 60)),
    ),
    (
        "zebi",
        "Zi",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N([64, 64].as_ptr().cast_mut(), -2))
        })),
    ),
    (
        "yobi",
        "Yi",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N([65536, 65536].as_ptr().cast_mut(), -2))
        })),
    ),
    (
        "robi",
        "Ri",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N(
                [67108864, 67108864].as_ptr().cast_mut(),
                2,
            ))
        })),
    ),
    (
        "quebi",
        "Qi",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N(
                [68719476736, 68719476736].as_ptr().cast_mut(),
                2,
            ))
        })),
    ),
    (
        "quetta",
        "Q",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N(
                [5076944324515372240, 54210108624].as_ptr().cast_mut(),
                2,
            ))
        })),
    ),
    (
        "ronna",
        "R",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N(
                [11515845246319275580, 54210108].as_ptr().cast_mut(),
                2,
            ))
        })),
    ),
    (
        "yotta",
        "Y",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N(
                [2003764205206950850, 54210].as_ptr().cast_mut(),
                2,
            ))
        })),
    ),
    (
        "zetta",
        "Z",
        &NumberWrapper::Int(IntegerWrapper::Int(unsafe {
            Integer::from_raw(gmp::MPZ_ROINIT_N(
                [3875820019684212790, 54].as_ptr().cast_mut(),
                2,
            ))
        })),
    ),
    (
        "exa",
        "E",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1000000000000000000)),
    ),
    (
        "peta",
        "P",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1000000000000000)),
    ),
    (
        "tera",
        "T",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1000000000000)),
    ),
    (
        "giga",
        "G",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1000000000)),
    ),
    (
        "mega",
        "M",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1000000)),
    ),
    (
        "myria",
        "myria",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(10000)),
    ),
    (
        "kilo",
        "k",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(1000)),
    ),
    (
        "hecto",
        "h",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(100)),
    ),
    (
        "deca",
        "da",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(10)),
    ),
    (
        "deka",
        "da",
        &NumberWrapper::Int(IntegerWrapper::SmallInt(10)),
    ),
    (
        "deci",
        "d",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(1, 10, true))),
    ),
    (
        "centi",
        "c",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(1, 100, true))),
    ),
    (
        "milli",
        "m",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(1, 1000, true))),
    ),
    (
        "micro",
        "u",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(
            1, 1000000, true,
        ))),
    ),
    (
        "nano",
        "n",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(
            1, 1000000000, true,
        ))),
    ),
    (
        "pico",
        "p",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(
            1,
            1000000000000,
            true,
        ))),
    ),
    (
        "femto",
        "f",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(
            1,
            1000000000000000,
            true,
        ))),
    ),
    (
        "atto",
        "a",
        &NumberWrapper::Rat(RationalWrapper::SmallRat(SmallRational::new(
            1,
            1000000000000000000,
            true,
        ))),
    ),
    (
        "zepto",
        "z",
        &NumberWrapper::Rat(RationalWrapper::Rat(unsafe {
            Rational::from_raw(mpq_t {
                num: gmp::MPZ_ROINIT_N([1].as_ptr().cast_mut(), 1),
                den: gmp::MPZ_ROINIT_N([3875820019684212790, 54].as_ptr().cast_mut(), 2),
            })
        })),
    ),
    (
        "yocto",
        "y",
        &NumberWrapper::Rat(RationalWrapper::Rat(unsafe {
            Rational::from_raw(mpq_t {
                num: gmp::MPZ_ROINIT_N([1].as_ptr().cast_mut(), 1),
                den: gmp::MPZ_ROINIT_N([2003764205206950850, 54210].as_ptr().cast_mut(), 2),
            })
        })),
    ),
    (
        "ronto",
        "r",
        &NumberWrapper::Rat(RationalWrapper::Rat(unsafe {
            Rational::from_raw(mpq_t {
                num: gmp::MPZ_ROINIT_N([1].as_ptr().cast_mut(), 1),
                den: gmp::MPZ_ROINIT_N([11515845246319275580, 54210108].as_ptr().cast_mut(), 2),
            })
        })),
    ),
    (
        "quecto",
        "q",
        &NumberWrapper::Rat(RationalWrapper::Rat(unsafe {
            Rational::from_raw(mpq_t {
                num: gmp::MPZ_ROINIT_N([1].as_ptr().cast_mut(), 1),
                den: gmp::MPZ_ROINIT_N([5076944324515372240, 54210108624].as_ptr().cast_mut(), 2),
            })
        })),
    ),
];

fn query_unit_aliases_internal(query: &str) -> &str {
    // if alias found, return pointer, else self
    for alias in UNIT_ALIASES {
        if query == alias.0 {
            return alias.1;
        }
    }
    query
}

fn query_unit_names_internal(query: &str, include_aliases: bool) -> Option<&'static UnitDefStatic> {
    let query = if include_aliases {
        query_unit_aliases_internal(query)
    } else {
        query
    };

    for unit in BASE_UNITS {
        if query == unit.0 {
            return Some(unit);
        }
    }

    for unit in OTHER_UNITS {
        if query == unit.0 {
            return Some(unit);
        }
    }

    return None;
}

fn query_unit_symbols_internal(query: &str) -> Option<&'static UnitDefStatic> {
    for unit in BASE_UNITS {
        if query == unit.1 {
            return Some(unit);
        }
    }

    for unit in OTHER_UNITS {
        if query == unit.1 {
            return Some(unit);
        }
    }

    return None;
}

pub(crate) fn search_for_unit_name(query: &str) -> Option<StaticSingleUnit> {
    // substitute from alias if applicable
    let query = query_unit_aliases_internal(query);

    // first, see if the query contains a perfect match
    if let Some(result) = query_unit_names_internal(query, false) {
        return Some(StaticSingleUnit::create_from_unit_def(result, None));
    } else if let Some(result) = query_unit_symbols_internal(query) {
        return Some(StaticSingleUnit::create_from_unit_def(result, None));
    }

    // if not, try to remove prefix
    for &prefix_def in STANDARD_PREFIXES {
        if query.starts_with(prefix_def.0) {
            // if the query starts with a standard prefix, try taking it out and querying the names
            if let Some(result) =
                query_unit_names_internal(&query.replacen(prefix_def.0, "", 1), true)
            {
                // disallow prefixes for nonabsolute scales
                if !result.2.is_zero() {
                    return None;
                }

                return Some(StaticSingleUnit::create_from_unit_def(
                    result,
                    Some(prefix_def),
                ));
            }
        } else if query.starts_with(prefix_def.1) {
            // if the query starts with a shortened prefix, try taking it out and querying the abbreviations
            if let Some(result) = query_unit_symbols_internal(&query.replacen(prefix_def.1, "", 1))
            {
                // disallow prefixes for nonabsolute scales
                if !result.2.is_zero() {
                    return None;
                }

                return Some(StaticSingleUnit::create_from_unit_def(
                    result,
                    Some(prefix_def),
                ));
            }
            // break statement not helpful for shortened prefix
        }
    }
    // if not, return none
    None
}

pub(crate) fn search_for_log_unit_name(query: &str) -> Option<LogUnit> {
    for log_unit in LOG_UNITS {
        if query == log_unit.0 || query == log_unit.1 {
            return Some(LogUnit {
                name: log_unit.0.to_string(),
                abbr: log_unit.1.to_string(),
                scale: log_unit.2,
                scale_per: log_unit.3,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_search() {
        let _meter = match search_for_unit_name("meter") {
            Some(unit) => unit,
            None => panic!("meter not found"),
        };
    }

    #[test]
    fn pascal_search() {
        let _pascal = match search_for_unit_name("pascal") {
            Some(unit) => unit,
            None => panic!("pascal not found"),
        };
    }

    #[test]
    fn metre_search() {
        let _meter = match search_for_unit_name("metre") {
            Some(unit) => unit,
            None => panic!("metre not found"),
        };
    }

    #[test]
    fn kilometer_search() {
        let kilometer = match search_for_unit_name("kilometer") {
            Some(unit) => unit,
            None => panic!("kilometer not found"),
        };
        assert_eq!(kilometer.scale, &NumberWrapper::ONE);
        assert_eq!(kilometer.prefix.unwrap().2, &NumberWrapper::from(1000));
    }

    #[test]
    fn yobidyne_search() {
        let _yobidyne = match search_for_unit_name("yobidyne") {
            Some(unit) => unit,
            None => panic!("yobidyne not found"),
        };
    }

    #[test]
    fn cm_search() {
        let _mm = match search_for_unit_name("cm") {
            Some(unit) => unit,
            None => panic!("cm not found"),
        };
    }

    #[test]
    fn km_hg_search() {
        let _km_hg = match search_for_unit_name("kmHg") {
            Some(unit) => unit,
            None => panic!("kmHg not found"),
        };
    }
}
