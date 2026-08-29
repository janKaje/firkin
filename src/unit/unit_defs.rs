// #![allow(unused)]

mod base_units;
mod derived_units;
mod aliases;

use crate::unit::single_unit::{SingleUnit, UnitDef};

pub(crate) use aliases::UNIT_ALIASES;
#[allow(unused)]
pub(crate) use base_units::{BASE_UNITS, NUMBER_OF_BASE_UNITS};
pub(crate) use derived_units::OTHER_UNITS;

// prefix, abbr, multiplier
type PrefixDef = (&'static str, &'static str, f64);

const STANDARD_PREFIXES: &[PrefixDef] = &[
    ("kibi", "Ki", (2 << 10) as f64),
    ("mebi", "Mi", (2 << 20) as f64),
    ("gibi", "Gi", (2 << 30) as f64),
    ("tebi", "Ti", (2i64 << 40) as f64),
    ("pebi", "Pi", (2i64 << 50) as f64),
    ("exbi", "Ei", (2i64 << 60) as f64),
    ("zebi", "Zi", (2i128 << 70) as f64),
    ("yobi", "Yi", (2i128 << 80) as f64),
    ("quetta", "Q", 1e30),
    ("ronna", "R", 1e27),
    ("yotta", "Y", 1e24),
    ("zetta", "Z", 1e21),
    ("exa", "E", 1e18),
    ("peta", "P", 1e15),
    ("tera", "T", 1e12),
    ("giga", "G", 1e9),
    ("mega", "M", 1e6),
    ("myria", "myria", 1e4),
    ("kilo", "k", 1e3),
    ("hecto", "h", 1e2),
    ("deca", "da", 1e1),
    ("deka", "da", 1e1),
    ("deci", "d", 1e-1),
    ("centi", "c", 1e-2),
    ("milli", "m", 1e-3),
    ("micro", "u", 1e-6),
    ("nano", "n", 1e-9),
    ("pico", "p", 1e-12),
    ("femto", "f", 1e-15),
    ("atto", "a", 1e-18),
    ("zepto", "z", 1e-21),
    ("yocto", "y", 1e-24),
    ("ronto", "r", 1e-27),
    ("quecto", "q", 1e-30),
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

fn query_unit_names_internal(query: &str, include_aliases: bool) -> Option<&UnitDef> {
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

fn query_unit_symbols_internal(query: &str) -> Option<&UnitDef> {
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

pub(crate) fn search_for_unit_name(query: &str) -> Option<SingleUnit> {
    // substitute from alias if applicable
    let query = query_unit_aliases_internal(query);

    // first, see if the query contains a perfect match
    if let Some(result) = query_unit_names_internal(query, false) {
        return Some(SingleUnit::create_from_unit_def(*result));
    } else if let Some(result) = query_unit_symbols_internal(query) {
        return Some(SingleUnit::create_from_unit_def(*result));
    }

    // if not, try to remove prefix
    for prefix_def in STANDARD_PREFIXES {
        if query.starts_with(prefix_def.0) {
            // if the query starts with a standard prefix, try taking it out and querying the names
            if let Some(result) =
                query_unit_names_internal(&query.replacen(prefix_def.0, "", 1), true)
            {
                // disallow prefixes for nonabsolute scales
                if result.2 != 0.0 {
                    return None;
                }
                let ret = (
                    prefix_def.0.to_string() + result.0,
                    prefix_def.1.to_string() + result.1,
                    0.0,
                    result.3 * prefix_def.2,
                    result.4,
                    result.5,
                    result.6,
                    result.7,
                    result.8,
                    result.9,
                    result.10,
                    result.11,
                );

                return Some(SingleUnit::create_from_unit_def_string(ret));
            }
        } else if query.starts_with(prefix_def.1) {
            // if the query starts with a shortened prefix, try taking it out and querying the abbreviations
            if let Some(result) = query_unit_symbols_internal(&query.replacen(prefix_def.1, "", 1))
            {
                // disallow prefixes for nonabsolute scales
                if result.2 != 0.0 {
                    return None;
                }
                let ret = (
                    prefix_def.0.to_string() + result.0,
                    prefix_def.1.to_string() + result.1,
                    0.0,
                    result.3 * prefix_def.2,
                    result.4,
                    result.5,
                    result.6,
                    result.7,
                    result.8,
                    result.9,
                    result.10,
                    result.11,
                );

                return Some(SingleUnit::create_from_unit_def_string(ret));
            }
            // break statement not helpful for shortened prefix
        }
    }
    // if not, return none
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
        assert_eq!(kilometer.scale, 1000.0);
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
