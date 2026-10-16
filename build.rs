#![allow(unused)]

/// This build file takes the unit definitions from unit_definitions
/// and turns them into usable rust files in src\unit\unit_defs
use std::{
    cmp::Ordering,
    collections::HashMap,
    ffi::OsStr,
    fmt::Write,
    fs::{self, File},
    ops::Index,
    path::{Component, PathBuf},
};

use csv;
use gcd::{binary_u64, binary_u128};
use regex::regex;

const UNIT_DEFINITONS_PATH: &str = "unit_definitions";
const PY_UNITS_PATH: [&str; 3] = ["python", "firkin", "units"];
const RS_UNITS_PATH: [&str; 3] = ["src", "unit", "unit_defs"];
const PY_CONST_PATH: [&str; 2] = ["python", "firkin"];
const RS_CONST_PATH: [&str; 2] = ["src", "constant"];

fn get_cfg_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir,
        Component::Normal(UNIT_DEFINITONS_PATH.as_ref()),
        Component::Normal(file_name.as_ref()),
    ]
    .iter()
    .collect()
}

fn get_py_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir,
        Component::Normal(PY_UNITS_PATH[0].as_ref()),
        Component::Normal(PY_UNITS_PATH[1].as_ref()),
        Component::Normal(PY_UNITS_PATH[2].as_ref()),
        Component::Normal(file_name.as_ref()),
    ]
    .iter()
    .collect()
}

fn get_rs_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir,
        Component::Normal(RS_UNITS_PATH[0].as_ref()),
        Component::Normal(RS_UNITS_PATH[1].as_ref()),
        Component::Normal(RS_UNITS_PATH[2].as_ref()),
        Component::Normal(file_name.as_ref()),
    ]
    .iter()
    .collect()
}

fn get_py_const_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir,
        Component::Normal(PY_CONST_PATH[0].as_ref()),
        Component::Normal(PY_CONST_PATH[1].as_ref()),
        Component::Normal(file_name.as_ref()),
    ]
    .iter()
    .collect()
}

fn get_rs_const_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir,
        Component::Normal(RS_CONST_PATH[0].as_ref()),
        Component::Normal(RS_CONST_PATH[1].as_ref()),
        Component::Normal(file_name.as_ref()),
    ]
    .iter()
    .collect()
}

/// these definitions have three strings, since the third is for python variable name
type DerivUnitConfigLine = (String, String, String, String, String, String);
type BaseUnitConfigLine = (String, String, String);
type LogUnitConfigLine = (String, String, String, f64, f64);
type ConstConfigLine = (String, f64, String, String);

struct UnitDefNumbers {
    scale: String,
    base_units: Vec<i32>,
}

struct UnitDef {
    name: String,
    abbr: String,
    python_var_name: String,
    offset: String,
    scale: String,
    base_units: Vec<i32>,
}

fn i32_to_small_rat(n: i32) -> &'static str {
    match n {
        0 => "&SR::ZERO, ",
        1 => "&SR::ONE, ",
        -1 => "&SR::NEG_ONE, ",
        2 => "&SR::TWO, ",
        -2 => "&SR::NEG_TWO, ",
        3 => "&SR::THREE, ",
        -3 => "&SR::NEG_THREE, ",
        4 => "&SR::FOUR, ",
        -4 => "&SR::NEG_FOUR, ",
        _ => "&SR::NONE, ",
    }
}

fn string_to_num_wrapper(input: String) -> String {
    if let Ok(i) = input.parse::<i64>() {
        return match i {
            0 => "&NW::ZERO".into(),
            1 => "&NW::ONE".into(),
            _ => format!("&NW::Int(IW::SmallInt({}))", i),
        };
    }

    let mut result = LittleNumWrapper::Int(1);

    for capture in regex!(r"(\A|\*|/)([-+]?\d+)\.?(\d*)[eE]?([-+]?\d*)").captures_iter(&input) {
        if capture[0].contains(['.', 'e', 'E']) {
            match &capture[1] {
                "/" => {
                    result = result
                        / (&capture[0])
                            .trim_start_matches('/')
                            .parse::<f64>()
                            .unwrap()
                            .into()
                }
                _ => {
                    result = result
                        * (&capture[0])
                            .trim_start_matches('*')
                            .parse::<f64>()
                            .unwrap()
                            .into()
                }
            }
        } else {
            // should only be integers now
            match &capture[1] {
                "/" => result = result / (&capture[2]).parse::<i64>().unwrap().into(),
                _ => result = result * (&capture[2]).parse::<i64>().unwrap().into(),
            }
        }
    }

    match result {
        Int(i) => format!("&NW::Int(IW::SmallInt({}))", i),
        Rat(r) => format!(
            "&NW::Rat(RW::SmallRat(SR::new({}, {}, {})))",
            r.numerator,
            r.denominator,
            r.is_positive()
        ),
        Float(f) => format!("&NW::Float(FW::SmallFloat({:e}))", f),
    }
}

/// Gathers information found in base_units.toml
fn read_base_units() -> Vec<BaseUnitConfigLine> {
    let mut unit_def_vec: Vec<BaseUnitConfigLine> = vec![];

    let mut base_unit_reader = csv::Reader::from_path(get_cfg_path("base_units.csv"))
        .expect("Could not locate base units config");

    for line in base_unit_reader.deserialize() {
        let line: BaseUnitConfigLine = line.expect("Could not parse line");

        unit_def_vec.push(line);
    }

    unit_def_vec
}

/// Write to base_units.rs
/// Returns a vector of the base unit names in the order they appear
fn write_base_units_rs(base_unit_vec: &Vec<BaseUnitConfigLine>) -> HashMap<String, UnitDefNumbers> {
    // get number of base units and create type_str
    let n_base_units = base_unit_vec.len();

    // initialize buffers to write data from toml file into
    let mut base_units_str = String::new();

    let mut unit_def_hashmap: HashMap<String, UnitDefNumbers> = HashMap::new();

    for (i, item) in base_unit_vec.iter().enumerate() {
        // get unit def numbers
        let mut numbers = vec![];

        for _ in 0..n_base_units {
            numbers.push(0);
        }

        *numbers.get_mut(i).unwrap() = 1;

        // extract abbr from toml
        let name = &item.0;
        let abbr = &item.1;

        // write to string buffer
        base_units_str.push_str("\n    (\"");
        base_units_str.push_str(name);
        base_units_str.push_str("\", \"");
        base_units_str.push_str(abbr);
        base_units_str.push_str("\", &NW::ZERO, &NW::ONE, [");
        for n in numbers.iter() {
            base_units_str.push_str(i32_to_small_rat(*n))
            // base_units_str.push_str(format!("{:?}, ", n).as_str());
        }
        base_units_str.push_str("]),");

        // add numbers to hashmap
        unit_def_hashmap.insert(
            name.clone(),
            UnitDefNumbers {
                scale: "1".into(),
                base_units: numbers,
            },
        );
    }

    let base_units_rs = format!(
        "\
/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\\base_units.csv`
/// If you want to change file layout, edit `build.rs`
use crate::num::{{number::NumberWrapper as NW, smallrational::SmallRational as SR}};

pub(crate) const NUMBER_OF_BASE_UNITS: usize = {};

#[rustfmt::skip]
pub(crate) type UnitDefStatic = (&'static str, &'static str, &'static NW, &'static NW, [&'static SR; NUMBER_OF_BASE_UNITS]);

#[rustfmt::skip]
pub(crate) const BASE_UNITS: &[UnitDefStatic] = &[{}
];
",
        n_base_units, base_units_str
    );

    fs::write(get_rs_path("base_units.rs"), base_units_rs);

    unit_def_hashmap
}

fn write_base_units_py(base_unit_vec: &Vec<BaseUnitConfigLine>) {
    let mut base_units_str = "from firkin import Firkin\n".to_string();

    for (name, abbr, python_var_name) in base_unit_vec.iter() {
        // write to string buffer
        base_units_str.push('\n');
        base_units_str.push_str(python_var_name);
        base_units_str.push_str(" = Firkin.unit('");
        base_units_str.push_str(name.as_str());
        base_units_str.push_str("')");
    }

    fs::write(get_py_path("base_units.py"), base_units_str);
}

fn get_unit_def_from_csv_line(
    line: DerivUnitConfigLine,
    unit_def_hashmap: &mut HashMap<String, UnitDefNumbers>,
) -> UnitDef {
    let (name, abbr, python_var_name, offset, mut scale, derivation) = line;

    let n_base_units = unit_def_hashmap.values().collect::<Vec<_>>()[0]
        .base_units
        .len();

    let mut base_units = vec![0; n_base_units];

    for unit_string in derivation.split("_") {
        if unit_string == "" {
            continue;
        }
        if unit_string.contains("^") {
            let v: Vec<&str> = unit_string.split("^").collect();
            let (unit_name, exp) = (v[0], v[1].parse::<i32>().unwrap());
            let unit_numbers = match unit_def_hashmap.get(unit_name) {
                Some(n) => n,
                None => panic!(
                    "Error on ln 153, hashmap doesn't have {}, does have {:?}",
                    unit_name,
                    unit_def_hashmap.keys().collect::<Vec<_>>()
                ),
            };

            scale.push('*');
            scale.push_str(&unit_numbers.scale);
            scale.push('^');
            scale.push_str(&exp.to_string());

            for i in 0..n_base_units {
                *base_units.get_mut(i).unwrap() += unit_numbers.base_units[i] * exp;
            }
        } else {
            let unit_numbers = unit_def_hashmap.get(unit_string).unwrap();

            scale.push('*');
            scale.push_str(&unit_numbers.scale);

            for i in 0..n_base_units {
                *base_units.get_mut(i).unwrap() += unit_numbers.base_units[i];
            }
        }
    }

    // insert new unit into hashmap
    unit_def_hashmap.insert(
        name.clone(),
        UnitDefNumbers {
            scale: scale.clone(),
            base_units: base_units.clone(),
        },
    );

    UnitDef {
        name,
        abbr,
        python_var_name,
        offset,
        scale,
        base_units,
    }
}

fn write_derived_units_rs(unit_def_vec: &Vec<UnitDef>) {
    let mut derived_units_rs = "\
/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\\derived_units.csv`
/// If you want to change file layout, edit `build.rs`
use super::base_units::UnitDefStatic;
use crate::num::{{
    number::NumberWrapper as NW, 
    smallrational::SmallRational as SR, 
    int::IntegerWrapper as IW, 
    rat::RationalWrapper as RW,
    float::FloatWrapper as FW,
}};

#[rustfmt::skip]
pub(crate) const OTHER_UNITS: &[UnitDefStatic] = &["
        .to_string();

    for unit_def in unit_def_vec {
        derived_units_rs.push_str(
            format!(
                "\n    (\"{}\", \"{}\", {}, {}, [",
                unit_def.name,
                unit_def.abbr,
                string_to_num_wrapper(unit_def.offset.clone()),
                string_to_num_wrapper(unit_def.scale.clone())
            )
            .as_str(),
        );
        for n in &unit_def.base_units {
            derived_units_rs.push_str(i32_to_small_rat(*n));
        }
        derived_units_rs.push_str("]),");
    }

    derived_units_rs.push_str("\n];\n");

    fs::write(get_rs_path("derived_units.rs"), derived_units_rs);
}

fn write_derived_units_py(unit_def_vec: &Vec<UnitDef>) {
    let mut derived_units_py = "from firkin import Firkin\n".to_string();

    for u_def in unit_def_vec {
        // write to string buffer
        derived_units_py.push('\n');
        derived_units_py.push_str(u_def.python_var_name.as_str());
        derived_units_py.push_str(" = Firkin.unit('");
        derived_units_py.push_str(u_def.name.as_str());
        derived_units_py.push_str("')");
    }

    fs::write(get_py_path("derived_units.py"), derived_units_py);
}

fn write_derived_units(unit_def_hashmap: &mut HashMap<String, UnitDefNumbers>) {
    let mut unit_def_vec: Vec<UnitDef> = vec![];

    let mut derived_unit_reader = csv::Reader::from_path(get_cfg_path("derived_units.csv"))
        .expect("Could not locate derived units config");

    for line in derived_unit_reader.deserialize() {
        let line: DerivUnitConfigLine = line.expect("Could not parse line");

        unit_def_vec.push(get_unit_def_from_csv_line(line, unit_def_hashmap));
    }

    write_derived_units_rs(&unit_def_vec);
    write_derived_units_py(&unit_def_vec);
}

fn write_aliases() {
    let mut aliases_reader = csv::Reader::from_path(get_cfg_path("aliases.csv"))
        .expect("Could not locate aliases config");

    let mut aliases_rs = "\
/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\\aliases.csv`
/// If you want to change file layout, edit `build.rs`

pub(crate) const UNIT_ALIASES: &[(&'static str, &'static str)] = &["
        .to_string();

    for line in aliases_reader.deserialize() {
        let (alias, name): (String, String) = line.expect("Could not parse line of aliases csv");
        aliases_rs.push_str(format!("\n    (\"{}\", \"{}\"),", alias, name).as_str());
    }

    aliases_rs.push_str("\n];\n");

    fs::write(get_rs_path("aliases.rs"), aliases_rs);
}

fn write_log_units_rs(log_unit_vec: &Vec<LogUnitConfigLine>) {
    let mut log_units_rs = "\
/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\\log_units.csv`
/// If you want to change file layout, edit `build.rs`

#[rustfmt::skip]
pub(crate) const LOG_UNITS: &[(&'static str, &'static str, f64, f64)] = &["
        .to_string();

    for (name, abbr, python_var_name, scale_factor, scale_factor_per) in log_unit_vec.iter() {
        log_units_rs.push_str(
            format!(
                "\n    (\"{}\", \"{}\", {:?}, {:?}),",
                name, abbr, scale_factor, scale_factor_per
            )
            .as_str(),
        );
    }

    log_units_rs.push_str("\n];\n");

    fs::write(get_rs_path("log_units.rs"), log_units_rs);
}

fn write_log_units_py(log_unit_vec: &Vec<LogUnitConfigLine>) {
    let mut log_units_py = "from firkin import LogFirkin\n".to_string();

    for (name, abbr, python_var_name, scale_factor, scale_factor_per) in log_unit_vec.iter() {
        log_units_py
            .push_str(format!("\n{} = LogFirkin.unit(\"{}\")", python_var_name, name,).as_str());
    }

    fs::write(get_py_path("log_units.py"), log_units_py);
}

fn write_log_units() {
    let mut unit_def_vec: Vec<LogUnitConfigLine> = vec![];

    let mut derived_unit_reader = csv::Reader::from_path(get_cfg_path("log_units.csv"))
        .expect("Could not locate log units config");

    for line in derived_unit_reader.deserialize() {
        let line: LogUnitConfigLine = line.expect("Could not parse line");

        unit_def_vec.push(line);
    }

    write_log_units_rs(&unit_def_vec);

    write_log_units_py(&unit_def_vec);
}

fn write_constant_defs_rs(const_def_vec: &Vec<ConstConfigLine>) {
    let mut const_defs_rs = "\
/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\\constants.csv`
/// If you want to change file layout, edit `build.rs`

#[rustfmt::skip]
pub(crate) const CONSTANTS: &[(&'static str, f64, &'static str)] = &["
        .to_string();

    for (name, value, units, python_var_name) in const_def_vec.iter() {
        const_defs_rs
            .push_str(format!("\n    (\"{}\", {:?}, \"{}\"),", name, value, units,).as_str());
    }

    const_defs_rs.push_str("\n];\n");

    fs::write(get_rs_const_path("constant_defs.rs"), const_defs_rs);
}

fn write_constant_defs_py(const_def_vec: &Vec<ConstConfigLine>) {
    let mut log_units_py = "from firkin import Firkin\n".to_string();

    for (name, value, units, python_var_name) in const_def_vec.iter() {
        log_units_py
            .push_str(format!("\n{} = Firkin.constant(\"{}\")", python_var_name, name,).as_str());
    }

    fs::write(get_py_const_path("constants.py"), log_units_py);
}

fn write_constant_defs() {
    let mut const_def_vec: Vec<ConstConfigLine> = vec![];

    let mut const_def_reader = csv::Reader::from_path(get_cfg_path("constants.csv"))
        .expect("Could not locate log units config");

    for line in const_def_reader.deserialize() {
        let line: ConstConfigLine = line.expect("Could not parse line");

        const_def_vec.push(line);
    }

    write_constant_defs_rs(&const_def_vec);

    write_constant_defs_py(&const_def_vec);
}

fn write_constant_aliases() {
    let mut aliases_reader = csv::Reader::from_path(get_cfg_path("constant_aliases.csv"))
        .expect("Could not locate aliases config");

    let mut aliases_rs = "\
/// This file was generated automatically by the build script.
/// If you want to add units, edit `unit_definitions\\aliases.csv`
/// If you want to change file layout, edit `build.rs`

pub(crate) const CONSTANT_ALIASES: &[(&'static str, &'static str)] = &["
        .to_string();

    for line in aliases_reader.deserialize() {
        let (alias, name): (String, String) =
            line.expect("Could not parse line of constant_aliases.csv");
        aliases_rs.push_str(format!("\n    (\"{}\", \"{}\"),", alias, name).as_str());
    }

    aliases_rs.push_str("\n];\n");

    fs::write(get_rs_const_path("aliases.rs"), aliases_rs);
}

fn main() {
    let cfg_path: PathBuf = [
        Component::CurDir,
        Component::Normal(UNIT_DEFINITONS_PATH.as_ref()),
    ]
    .iter()
    .collect();

    println!("cargo::rerun-if-changed={}", cfg_path.as_path().display());

    let base_unit_config = read_base_units();

    write_base_units_py(&base_unit_config);

    let mut unit_def_hashmap = write_base_units_rs(&base_unit_config);

    write_derived_units(&mut unit_def_hashmap);

    write_aliases();

    write_log_units();

    write_constant_defs();

    write_constant_aliases();
}

// redefinitions for building purposes
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Sign {
    Positive,
    Negative,
}

impl From<Sign> for i128 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => -1,
        }
    }
}

use Sign::{Negative, Positive};

#[derive(Clone, Copy, Hash, Debug)]
struct SmallRational {
    pub numerator: u64,
    pub denominator: u64,
    pub sign: Sign,
}

impl SmallRational {
    pub(crate) const ZERO: SmallRational = SmallRational {
        numerator: 0,
        denominator: 1,
        sign: Positive,
    };

    pub(crate) const ONE: SmallRational = SmallRational {
        numerator: 1,
        denominator: 1,
        sign: Positive,
    };

    pub(crate) const NEG_ONE: SmallRational = SmallRational {
        numerator: 1,
        denominator: 1,
        sign: Negative,
    };

    pub(crate) const TWO: SmallRational = SmallRational {
        numerator: 2,
        denominator: 1,
        sign: Positive,
    };

    pub(crate) const NEG_TWO: SmallRational = SmallRational {
        numerator: 2,
        denominator: 1,
        sign: Negative,
    };

    pub(crate) const THREE: SmallRational = SmallRational {
        numerator: 3,
        denominator: 1,
        sign: Positive,
    };

    pub(crate) const NEG_THREE: SmallRational = SmallRational {
        numerator: 3,
        denominator: 1,
        sign: Negative,
    };

    pub(crate) const FOUR: SmallRational = SmallRational {
        numerator: 4,
        denominator: 1,
        sign: Positive,
    };

    pub(crate) const NEG_FOUR: SmallRational = SmallRational {
        numerator: 4,
        denominator: 1,
        sign: Negative,
    };

    pub(crate) const fn new(numerator: u64, denominator: u64, is_positive: bool) -> Self {
        if denominator == 0 {
            panic!("denominator cannot be zero");
        }
        if numerator == 0 {
            return SmallRational {
                numerator: 0,
                denominator: 1,
                sign: Positive,
            };
        }
        let result = SmallRational {
            numerator,
            denominator,
            sign: { if is_positive { Positive } else { Negative } },
        };
        result.reduce_inplace()
    }

    pub(crate) const fn try_new(
        numerator: u128,
        denominator: u128,
        is_positive: bool,
    ) -> Option<Self> {
        if denominator == 0 {
            panic!("denominator cannot be zero");
        }
        if numerator == 0 {
            return Some(SmallRational {
                numerator: 0,
                denominator: 1,
                sign: Positive,
            });
        }
        let gcd = binary_u128(numerator, denominator);
        let numerator = numerator / gcd;
        let denominator = denominator / gcd;

        if numerator > u64::MAX as u128 || denominator > u64::MAX as u128 {
            None // overflow
        } else {
            Some(
                SmallRational {
                    numerator: numerator as u64,
                    denominator: denominator as u64,
                    sign: { if is_positive { Positive } else { Negative } },
                }
                .reduce_inplace(),
            )
        }
    }

    pub(crate) fn is_positive(&self) -> bool {
        return self.sign == Positive;
    }

    pub(crate) fn is_negative(&self) -> bool {
        return self.sign == Negative;
    }

    pub(crate) const fn is_zero(&self) -> bool {
        return self.numerator == 0;
    }

    pub(crate) const fn is_integer(&self) -> bool {
        return self.denominator == 1;
    }

    const fn reduce_inplace(mut self) -> Self {
        let gcd = binary_u64(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
        self
    }

    pub(crate) fn reduce(&self) -> Self {
        let new = self.clone();
        new.reduce_inplace()
    }

    pub(crate) fn checked_pow(&self, other: i32) -> Option<Self> {
        if other >= 0 {
            SmallRational::try_new(
                (self.numerator as u128).checked_pow(other.unsigned_abs())?,
                (self.denominator as u128).checked_pow(other.unsigned_abs())?,
                self.is_positive() || other % 2 == 0,
            )
        } else {
            SmallRational::try_new(
                (self.denominator as u128).checked_pow(other.unsigned_abs())?,
                (self.numerator as u128).checked_pow(other.unsigned_abs())?,
                self.is_positive() || other % 2 == 0,
            )
        }
    }

    pub(crate) fn checked_add<RhsAdd>(&self, rhs: RhsAdd) -> Option<Self>
    where
        SmallRational: From<RhsAdd>,
    {
        let rhs: SmallRational = rhs.into();
        let snum = self.numerator as u128;
        let sden = self.denominator as u128;
        let rnum = rhs.numerator as u128;
        let rden = rhs.denominator as u128;
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                true,
            ),
            (Positive, Negative) => {
                let rhsabs = rhs.abs();
                if self == &rhsabs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if self > &rhsabs {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                } else {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                }
            }
            (Negative, Positive) => {
                let selfabs = self.abs();
                if selfabs == rhs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if selfabs < rhs {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                } else {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                }
            }
            (Negative, Negative) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                false,
            ),
        }
    }

    pub(crate) fn checked_sub<RhsSub>(&self, rhs: RhsSub) -> Option<Self>
    where
        SmallRational: From<RhsSub>,
    {
        let rhs: SmallRational = rhs.into();
        let snum = self.numerator as u128;
        let sden = self.denominator as u128;
        let rnum = rhs.numerator as u128;
        let rden = rhs.denominator as u128;
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => {
                if self == &rhs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if self > &rhs {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                } else {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                }
            }
            (Positive, Negative) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                true,
            ),
            (Negative, Positive) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                false,
            ),
            (Negative, Negative) => {
                if self == &rhs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if self < &rhs {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                } else {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                }
            }
        }
    }

    pub(crate) fn checked_mul<RhsMul>(&self, rhs: RhsMul) -> Option<Self>
    where
        SmallRational: From<RhsMul>,
    {
        let rhs: SmallRational = rhs.into();
        SmallRational::try_new(
            (self.numerator as u128).checked_mul(rhs.numerator as u128)?,
            (self.denominator as u128).checked_mul(rhs.denominator as u128)?,
            self.sign == rhs.sign,
        )
    }

    pub(crate) fn checked_div<RhsDiv>(&self, rhs: RhsDiv) -> Option<Self>
    where
        SmallRational: From<RhsDiv>,
    {
        let rhs: SmallRational = rhs.into();
        SmallRational::try_new(
            (self.numerator as u128).checked_mul(rhs.denominator as u128)?,
            (self.denominator as u128).checked_mul(rhs.numerator as u128)?,
            self.sign == rhs.sign,
        )
    }

    pub(crate) fn get_whole_number_part(&self) -> i128 {
        ((self.numerator - (self.numerator % self.denominator)) / self.denominator) as i128
            * i128::from(self.sign)
    }

    pub(crate) fn get_remainder(&self) -> Self {
        SmallRational::new(
            self.numerator % self.denominator,
            self.denominator,
            self.is_positive(),
        )
    }

    pub(crate) fn floor(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else if *self >= SmallRational::ZERO {
            self.get_whole_number_part()
        } else {
            self.get_whole_number_part() - 1
        }
    }

    pub(crate) fn ceil(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else if *self >= SmallRational::ZERO {
            self.get_whole_number_part() + 1
        } else {
            self.get_whole_number_part()
        }
    }

    pub(crate) fn round(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else {
            let rem = self.get_remainder();
            if rem > SmallRational::ZERO && rem < SmallRational::new(1, 2, true)
                || rem < SmallRational::ZERO && rem > SmallRational::new(1, 2, false)
            {
                self.get_whole_number_part()
            } else {
                self.get_whole_number_part() + i128::from(self.sign)
            }
        }
    }

    pub(crate) fn abs(&self) -> Self {
        SmallRational {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: Positive,
        }
    }

    fn coerce_to_denom_round(&self, denom: u64) -> SmallRational {
        let floord = self.numerator / self.denominator;
        let rem = self.numerator % self.denominator;
        if rem <= denom / 2 {
            // round down
            SmallRational::new(floord * denom, denom, self.is_positive())
        } else {
            SmallRational::new(floord * denom + 1, denom, self.is_positive())
        }
    }
}

impl From<i8> for SmallRational {
    fn from(value: i8) -> Self {
        SmallRational {
            numerator: value.unsigned_abs().into(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<i16> for SmallRational {
    fn from(value: i16) -> Self {
        SmallRational {
            numerator: value.unsigned_abs().into(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<i32> for SmallRational {
    fn from(value: i32) -> Self {
        SmallRational {
            numerator: value.unsigned_abs().into(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<i64> for SmallRational {
    fn from(value: i64) -> Self {
        SmallRational {
            numerator: value.unsigned_abs(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<isize> for SmallRational {
    fn from(value: isize) -> Self {
        SmallRational {
            numerator: value.unsigned_abs() as u64,
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<u8> for SmallRational {
    fn from(value: u8) -> Self {
        SmallRational {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u16> for SmallRational {
    fn from(value: u16) -> Self {
        SmallRational {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u32> for SmallRational {
    fn from(value: u32) -> Self {
        SmallRational {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u64> for SmallRational {
    fn from(value: u64) -> Self {
        SmallRational {
            numerator: value,
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<usize> for SmallRational {
    fn from(value: usize) -> Self {
        SmallRational {
            numerator: value as u64,
            denominator: 1,
            sign: Positive,
        }
    }
}

impl<Num, Den> From<(Num, Den)> for SmallRational
where
    i64: From<Num> + From<Den>,
{
    fn from(value: (Num, Den)) -> Self {
        let num: i64 = value.0.into();
        let den: i64 = value.1.into();
        SmallRational::new(
            num.unsigned_abs(),
            den.unsigned_abs(),
            num.is_positive() == den.is_positive(),
        )
    }
}

impl From<&SmallRational> for SmallRational {
    fn from(value: &SmallRational) -> Self {
        *value
    }
}

impl TryFrom<i128> for SmallRational {
    type Error = &'static str;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        match SmallRational::try_new(value.unsigned_abs(), 1, value.is_positive()) {
            Some(res) => Ok(res),
            None => Err("Overflow"),
        }
    }
}

impl TryFrom<f64> for SmallRational {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else if value > i128::MAX as f64 || value < i128::MIN as f64 {
            Err("Value is too big")
        } else if value != value.round() {
            Err("Value is not integer.") // SWITCH LATER todo!()
        } else {
            SmallRational::try_from(value.round() as i128)
        }
    }
}

impl From<Sign> for f64 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1.0,
            Negative => -1.0,
        }
    }
}

impl From<SmallRational> for f64 {
    fn from(value: SmallRational) -> Self {
        f64::from(value.sign) * (value.numerator as f64) / (value.denominator as f64)
    }
}

impl PartialEq for SmallRational {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            return true;
        }
        self.numerator == other.numerator
            && other.denominator == self.denominator
            && self.sign == other.sign
    }
}

impl Eq for SmallRational {}

impl PartialOrd for SmallRational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.numerator == 0 && other.numerator == 0 {
            return Some(Ordering::Equal); // -0 and 0 are equal
        }
        match (&self.sign, &other.sign) {
            (Positive, Positive) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Some(Ordering::Equal)
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (Positive, Negative) => Some(Ordering::Greater),
            (Negative, Positive) => Some(Ordering::Less),
            (Negative, Negative) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Some(Ordering::Equal)
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl Ord for SmallRational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.numerator == 0 && other.numerator == 0 {
            return Ordering::Equal; // -0 and 0 are equal
        }
        match (&self.sign, &other.sign) {
            (Positive, Positive) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Ordering::Equal
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Positive, Negative) => Ordering::Greater,
            (Negative, Positive) => Ordering::Less,
            (Negative, Negative) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Ordering::Equal
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

macro_rules! arith_binary_rational {
    (
        $checked_method:ident
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<SmallRational> for SmallRational {
            type Output = SmallRational;

            fn $method(self, rhs: SmallRational) -> Self::Output {
                match self.$checked_method(rhs) {
                    Some(result) => result,
                    None => panic!(
                        "Overflow: {} method, self: {:?}, rhs: {:?}",
                        stringify!($method),
                        self,
                        rhs
                    ),
                }
            }
        }

        impl $Imp<i64> for SmallRational {
            type Output = SmallRational;

            fn $method(self, rhs: i64) -> Self::Output {
                match self.$checked_method(rhs) {
                    Some(result) => result,
                    None => panic!(
                        "Overflow: {} method, self: {:?}, rhs: {:?}",
                        stringify!($method),
                        self,
                        rhs
                    ),
                }
            }
        }

        impl $ImpAssign<SmallRational> for SmallRational {
            fn $method_assign(&mut self, rhs: SmallRational) {
                *self = self.$method(rhs)
            }
        }
    };
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, Sub, SubAssign};

arith_binary_rational! {
    checked_add
    Add { add }
    AddAssign { add_assign }
}

arith_binary_rational! {
    checked_sub
    Sub { sub }
    SubAssign { sub_assign }
}

arith_binary_rational! {
    checked_mul
    Mul { mul }
    MulAssign { mul_assign }
}

arith_binary_rational! {
    checked_div
    Div { div }
    DivAssign { div_assign }
}

#[derive(Debug)]
enum LittleNumWrapper {
    Int(i64),
    Rat(SmallRational),
    Float(f64),
}

use LittleNumWrapper::{Float, Int, Rat};

impl Mul for LittleNumWrapper {
    type Output = LittleNumWrapper;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Int(i), Int(i2)) => Int(i * i2),
            (Int(i), Rat(r)) => Rat(r * i),
            (Int(i), Float(f)) => Float((i as f64) * f),
            (Rat(r), Int(i)) => Rat(r * i),
            (Rat(r), Rat(r2)) => Rat(r * r2),
            (Rat(r), Float(f)) => Float(f64::from(r) * f),
            (Float(f), Int(i)) => Float(f * (i as f64)),
            (Float(f), Rat(r)) => Float(f * f64::from(r)),
            (Float(f), Float(f2)) => Float(f * f2),
        }
    }
}

impl Div for LittleNumWrapper {
    type Output = LittleNumWrapper;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Int(i), Int(i2)) => Rat(SmallRational::from(i) / SmallRational::from(i2)),
            (Int(i), Rat(r)) => Rat(SmallRational::from(i) / r),
            (Int(i), Float(f)) => Float((i as f64) / f),
            (Rat(r), Int(i)) => Rat(r / SmallRational::from(i)),
            (Rat(r), Rat(r2)) => Rat(r / r2),
            (Rat(r), Float(f)) => Float(f64::from(r) / f),
            (Float(f), Int(i)) => Float(f / (i as f64)),
            (Float(f), Rat(r)) => Float(f / f64::from(r)),
            (Float(f), Float(f2)) => Float(f / f2),
        }
    }
}

impl From<i64> for LittleNumWrapper {
    fn from(value: i64) -> Self {
        Int(value)
    }
}

impl From<f64> for LittleNumWrapper {
    fn from(value: f64) -> Self {
        if value.round() == value
            && value < ((1_i64 << 53) as f64)
            && value > (-(1_i64 << 53) as f64)
        {
            Int(value as i64)
        } else {
            Float(value)
        }
    }
}

impl From<SmallRational> for LittleNumWrapper {
    fn from(value: SmallRational) -> Self {
        Rat(value)
    }
}
