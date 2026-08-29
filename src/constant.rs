pub(crate) type ConstantDef = (&'static str, f64, &'static str);
type Alias = (&'static str, &'static str);

// name, scale, units

const CONSTANTS: &[ConstantDef] = &[
    ("light speed", 299792458.0, "m/s"),
    ("planck's constant", 6.62607015e-34, "J.s"),
    ("reduced planck's constant", 1.054571817e-34, "J.s"),
    ("boltzmann's constant", 1.380649e-23, "J/K"),
    ("electron charge", 1.602176634e-19, "coulomb"),
    ("electron mass", 9.1093837015e-31, "kg"),
    ("proton mass", 1.67262192369e-27, "kg"),
    ("neutron mass", 1.67492749804e-27, "kg"),
    ("avogadro's constant", 6.02214076e23, "mol"),
    ("gravitational constant", 6.67430e-11, "N.m2/kg.2"),
    ("earth gravity", 9.80665, "m/s2"),
    ("stefan-boltzmann constant", 5.670374419e-8, "W/m2.K4"),
    ("ideal gas constant", 8.31446261815324, "J/K.mol"),
    ("faraday constant", 9.64853321233100184e4, "coulomb/mol"),
    ("vacuum permittivity", 8.8541878188e-12, "farad/m"),
    ("vacuum permeability", 1.25663706127e-6, "N/A2"),
];

const CONSTANT_ALIASES: &[Alias] = &[
    ("c", "light speed"),
    ("speed of light", "light speed"),
    ("lightspeed", "light speed"),
    ("planck", "planck's constant"),
    ("boltzmann", "boltzmann's constant"),
    ("elementary charge", "electron charge"),
    ("avogadro", "avogadro's constant"),
    ("avogadro's number", "avogadro's constant"),
    ("N_A", "avogadro's constant"),
    ("stefan-boltzmann", "stefan-boltzmann's constant"),
    ("stefan boltzmann", "stefan-boltzmann's constant"),
    ("gas constant", "ideal gas constant"),
    ("molar gas constant", "ideal gas constant"),
    ("faraday", "faraday's constant"),
];

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
