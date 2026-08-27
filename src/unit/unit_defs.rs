// #![allow(unused)]

use crate::unit::single_unit::{SingleUnit, UnitDef};

// name, abbr, offset (in and from base units), scale, (exponents) second, meter, kilogram, ampere, kelvin, mole, candela, USD

#[rustfmt::skip]
pub(crate) const BASE_UNITS: &[UnitDef] = &[
    ("second", "s",    0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("meter", "m",     0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("kilogram", "kg", 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("ampere", "A",    0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
    ("kelvin", "K",    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
    ("mole", "mol",    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
    ("candela", "cd",  0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
    ("USD", "USD",     0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
];

#[rustfmt::skip]
const OTHER_UNITS: &[UnitDef] = &[
    ("gram", "g", 0.0, 0.001, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("pascal", "Pa", 0.0, 1.0, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("radian", "rad", 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("steradian", "sr", 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("hertz", "Hz", 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("newton", "N", 0.0, 1.0, -2.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("joule", "J", 0.0, 1.0, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("watt", "W", 0.0, 1.0, -3.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("coulomb", "C", 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
    ("volt", "V", 0.0, 1.0, -3.0, 2.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0),
    ("farad", "F", 0.0, 1.0, 4.0, -2.0, -1.0, 2.0, 0.0, 0.0, 0.0, 0.0),
    ("ohm", "\u{3A9}", 0.0, 1.0, -3.0, 2.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0),
    ("siemens", "S", 0.0, 1.0, 3.0, -2.0, -1.0, 2.0, 0.0, 0.0, 0.0, 0.0),
    ("weber", "Wb", 0.0, 1.0, -2.0, 2.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0),
    ("tesla", "T", 0.0, 1.0, -2.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0),
    ("hour", "hr", 0.0, 3600.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("henry", "H", 0.0, 1.0, -2.0, 2.0, 1.0, -2.0, 0.0, 0.0, 0.0, 0.0),
    ("degree Celsius", "\u{B0}C", -273.15, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
    ("lumen", "lm", 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
    ("lux", "lx", 0.0, 1.0, 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
    ("becquerel", "Bq", 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("gray", "Gy", 0.0, 1.0, -2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("sievert", "Sv", 0.0, 1.0, -2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("katal", "kat", 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
    ("minute", "min", 0.0, 60.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("day", "d", 0.0, 86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("degree (angle)", "\u{B0}", 0.0, 0.017453292519943295, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("minute (angle)", "'", 0.0, 0.0002908882086657216, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("second (angle)", "''", 0.0, 4.84813681109536e-06, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("liter", "L", 0.0, 0.001, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("metric ton", "t", 0.0, 1000.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("neper", "Np", 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("decibel", "dB", 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("electronvolt", "eV", 0.0, 1.60218e-19, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("unified atomic mass unit", "amu", 0.0, 1.66054e-27, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("mile", "mi", 0.0, 1609.34, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("nautical mile", "nmi", 0.0, 1852.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("knot", "kn", 0.0, 0.447039, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("are", "are", 0.0, 100.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("bar", "bar", 0.0, 100000.0, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("angstrom", "\u{c5}", 0.0, 1e-10, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("barn", "b", 0.0, 1.0e-28, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("curie", "Ci", 0.0, 37000000000.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("roentgen", "R", 0.0, 0.000258, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
    ("rem", "rem", 0.0, 0.01, -2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("pound", "lb", 0.0, 0.4535924, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("foot", "ft", 0.0, 0.3048, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("inch", "in", 0.0, 0.0254, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("yard", "yd", 0.0, 0.9144, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("pound-mol", "lbmol", 0.0, 453.5924, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
    ("gallon", "gal", 0.0, 0.003785412, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("pound-force", "lbf", 0.0, 4.448221970785693, -2.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("grain", "gr", 0.0, 6.479891428571429e-05, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("slug", "slug", 0.0, 14.59390392219064, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("fluid ounce", "fl oz", 0.0, 2.957353125e-05, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("quart", "qt", 0.0, 0.000946353, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("pint", "pt", 0.0, 0.0004731765, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("cup", "cup", 0.0, 0.00023658825, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("teaspoon", "tsp", 0.0, 4.928921875e-06, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("tablespoon", "tbsp", 0.0, 1.4786765625e-05, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("ton", "ton", 0.0, 907.1848, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("ounce", "oz", 0.0, 0.028349525, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("torr", "torr", 0.0, 101325.0/760.0, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("pounds per square inch", "psi", 0.0, 6892.857142857143, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("atmosphere", "atm", 0.0, 101325.0, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("calorie", "cal", 0.0, 4.184, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("kilocalorie", "kcal", 0.0, 4184.0, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("erg", "erg", 0.0, 1e-07, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("british thermal unit", "BTU", 0.0, 1054.35, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("horsepower", "hp", 0.0, 745.6999311825133, -3.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("Hartree", "hartree", 0.0, 4.359744722206e-18, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("poise", "P", 0.0, 0.1, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("Rankine", "\u{B0}R", 0.0, 0.5555555555555556, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
    ("degree Fahrenheit", "\u{B0}F", -255.37, 0.5555555555555556, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
    ("year", "y", 0.0, 365.242198781*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("week", "wk", 0.0, 604800.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("month", "mo", 0.0, 2629800.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("rotations per minute", "rpm", 0.0, 0.10471975511965977, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("lightyear", "ly", 0.0, 9460660000000000.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("acre", "ac", 0.0, 0.0002471053814671653, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("hectare", "ha", 0.0, 10000.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("carat", "ct", 0.0, 0.0002, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("watt hour", "Wh", 0.0, 3600.0, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("cent", "¢", 0.0, 0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
    ("darcy", "d", 0.0, 9.869233e-13, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("dyne", "dyn", 0.0, 1e-05, -2.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("astronomical unit", "ua", 0.0, 149598000000.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("sidereal year", "syr", 0.0, 365.256363004*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("anomalistic year", "ayr", 0.0, 365.259636*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("decade", "decade", 0.0, 365.242198781*86400.0*10.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("century", "century", 0.0, 365.242198781*86400.0*100.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("millennium", "millenium", 0.0, 365.242198781*86400.0*1000.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("calendar year", "calendar year", 0.0, 365.0*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("leap year", "leap year", 0.0, 366.0*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("julian year", "julian year", 0.0, 365.25*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("gregorian year", "gregorian year", 0.0, 365.2425*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("parsec", "pc", 0.0, 30856775814913673.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("millimeter of mercury", "mmHg", 0.0, 133.322387415, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("inch of mercury", "inHg", 0.0, 133.322387415*25.4, -2.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("firkin (vol)", "frknv", 0.0, 9.0*0.003785412, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("firkin (mass)", "frknm", 0.0, 90.0*0.4535924, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0), // as seen in the FFF system of measurements
    ("furlong", "flg", 0.0, 660.0*0.3048, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("fortnight", "ftnt", 0.0, 14.0*86400.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("outhouse", "ouths", 0.0, 1.0e-34, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("shed", "shed", 0.0, 1.0e-52, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("martian sol", "sol", 0.0, 88775.244, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("jiffy", "jfy", 0.0, 0.01, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ("light speed", "c", 0.0, 299792458.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
];

// alias, pointer
type Alias = (&'static str, &'static str);

const UNIT_ALIASES: &[Alias] = &[
    ("metre", "meter"),
    ("deg", "degree (angle)"),
    ("degree", "degree (angle)"),
    ("da", "day"),
    ("tropical year", "year"),
    ("solar year", "year"),
    ("common year", "calendar year"),
    ("AU", "astronomical unit"),
    ("micron", "micrometer"),
    ("cc", "milliliter"),
    ("deg C", "degree Celsius"),
    ("deg F", "degree Fahrenheit"),
    ("deg c", "degree Celsius"),
    ("deg f", "degree Fahrenheit"),
    ("rankine", "Rankine"),
    ("Celsius", "degree Celsius"),
    ("Fahrenheit", "degree Fahrenheit"),
    ("celsius", "degree Celsius"),
    ("fahrenheit", "degree Fahrenheit"),
    ("statute mile", "mile"),
    ("firkin", "firkin (mass)"),
    ("speed of light", "light speed"),
    ("lightspeed", "light speed"),
];

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
    fn kmm_hg_search() {
        let _kmm_hg = match search_for_unit_name("kmmHg") {
            Some(unit) => unit,
            None => panic!("kmmHg not found"),
        };
    }
}
