pub(crate) type ConstantDef = (&'static str, f64, &'static str);

// name, scale, units

mod aliases;
mod constant_defs;

use aliases::CONSTANT_ALIASES;
use constant_defs::CONSTANTS;

fn query_constant_aliases_internal(query: &str) -> &str {
    // if alias found, return pointer, else self
    for alias in CONSTANT_ALIASES {
        if query == alias.0 {
            return alias.1;
        }
    }
    query
}

fn query_constant_names_internal(query: &str) -> Option<&ConstantDef> {
    let query = query_constant_aliases_internal(query);

    for cons in CONSTANTS {
        if query == cons.0 {
            return Some(cons);
        }
    }

    return None;
}

pub(crate) fn search_for_constant_name(query: &str) -> Option<&ConstantDef> {
    query_constant_names_internal(query)
}
