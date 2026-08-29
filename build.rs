#![allow(unused)]

/// This build file takes the unit definitions from unit_definitions
/// and turns them into usable rust files in src\unit\unit_defs

use std::{collections::HashMap, fmt::Write, fs::{self, File}, path::{Component, PathBuf}, ffi::OsStr};

use toml;
use csv;

const UNIT_DEFINITONS_PATH: &str = "unit_definitions";
const PY_UNITS_PATH: [&str; 3] = ["python", "firkin", "units"];
const RS_UNITS_PATH: [&str; 3] = ["src", "unit", "unit_defs"];

fn get_cfg_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir, 
        Component::Normal(UNIT_DEFINITONS_PATH.as_ref()), 
        Component::Normal(file_name.as_ref())
    ].iter().collect()
}

fn get_py_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir, 
        Component::Normal(PY_UNITS_PATH[0].as_ref()), 
        Component::Normal(PY_UNITS_PATH[1].as_ref()), 
        Component::Normal(PY_UNITS_PATH[2].as_ref()), 
        Component::Normal(file_name.as_ref())
    ].iter().collect()
}

fn get_rs_path(file_name: &str) -> PathBuf {
    [
        Component::CurDir, 
        Component::Normal(RS_UNITS_PATH[0].as_ref()), 
        Component::Normal(RS_UNITS_PATH[1].as_ref()), 
        Component::Normal(RS_UNITS_PATH[2].as_ref()), 
        Component::Normal(file_name.as_ref())
    ].iter().collect()
}

/// these definitions have three strings, since the third is for python variable name
type DerivUnitConfigLine = (String, String, String, f64, f64, String);

struct UnitDefNumbers {
    scale: f64,
    base_units: Vec<f64>
}

struct UnitDef{
    name: String,
    abbr: String,
    python_var_name: String,
    offset: f64,
    scale: f64,
    base_units: Vec<f64>,
}

/// Gathers information found in base_units.toml
fn read_base_units() -> toml::Table {
    let config_str = fs::read_to_string(get_cfg_path("base_units.toml")).expect("Failed to read file");
    toml::from_str(&config_str).expect("Failed to parse toml")
}

/// Write to base_units.rs
/// Returns a vector of the base unit names in the order they appear
fn write_base_units_rs(config: &toml::Table) -> HashMap<String, UnitDefNumbers> {

    // get number of base units and create type_str
    let n_base_units = config.len();

    let type_str = "&'static str, &'static str, f64, f64".to_string() + &", f64".repeat(n_base_units);

    // initialize buffers to write data from toml file into
    let mut base_units_str = String::new();

    let mut unit_def_hashmap: HashMap<String, UnitDefNumbers> = HashMap::new();

    for (i, (key, value)) in config.iter().enumerate() {

        // get unit def numbers
        let mut numbers = vec![];

        for _ in 0..n_base_units {
            numbers.push(0.0);
        }

        *numbers.get_mut(i).unwrap() = 1.0;

        // extract abbr from toml
        let as_table = value.as_table().expect("base_units.toml should not have root table");
        let abbr = as_table.get("abbr").expect(("Unit ".to_string() + &key + "  does not have abbr").as_str());

        // write to string buffer
        base_units_str.push_str("\n    (\"");
        base_units_str.push_str(key);
        base_units_str.push_str("\", \"");
        base_units_str.push_str(abbr.as_str().expect(("Unit ".to_string() + &key + "  abbr is not string").as_str()));
        base_units_str.push_str("\", 0.0, 1.0, ");
        for n in numbers.iter() {
            base_units_str.push_str(format!("{:?}, ", n).as_str());
        }
        base_units_str.push_str("),");

        // add numbers to hashmap
        unit_def_hashmap.insert(key.clone(), UnitDefNumbers { scale: 1.0, base_units: numbers });

    }

    let base_units_rs = format!(
        "\
/// This file was generated automatically by the build script. 
/// If you want to add units, edit `unit_definitions\\base_units.toml`
/// If you want to change file layout, edit `build.rs`

#[allow(unused)]
pub(crate) const NUMBER_OF_BASE_UNITS: u8 = {};

#[rustfmt::skip]
pub(crate) const BASE_UNITS: &[({})] = &[{}
];

",
        n_base_units,
        type_str,
        base_units_str
    );

    fs::write(get_rs_path("base_units.rs"), base_units_rs);

    unit_def_hashmap
}

fn write_base_units_py(config: &toml::Table) {
    
    let mut base_units_str = "from firkin import Firkin\n".to_string();

    for (key, value) in config.iter() {

        // extract python_var_name from toml
        let as_table = value.as_table().expect("base_units.toml should not have root table");
        let value = as_table.get("python_var_name").expect(("Unit ".to_string() + &key + "  does not have python_var_name").as_str());
        let python_var_name = value.as_str().expect(("Unit ".to_string() + &key + "  python_var_name not a string").as_str());

        // write to string buffer
        base_units_str.push('\n');
        base_units_str.push_str(python_var_name);
        base_units_str.push_str(" = Firkin.unit('");
        base_units_str.push_str(key.as_str());
        base_units_str.push_str("')" );
    }

    fs::write(get_py_path("base_units.py"), base_units_str);

}

fn get_unit_def_from_csv_line(line: DerivUnitConfigLine, unit_def_hashmap: &mut HashMap<String, UnitDefNumbers>) -> UnitDef {
    let (name, abbr, python_var_name, offset, mut scale, derivation) = line;

    let n_base_units = unit_def_hashmap.values().collect::<Vec<_>>()[0].base_units.len();

    let mut base_units = vec![0.0; n_base_units];

    for unit_string in derivation.split("_") {
        if unit_string == "" {
            continue
        }
        if unit_string.contains("^") {

            let v:Vec<&str> = unit_string.split("^").collect();
            let (unit_name, exp) = (v[0], v[1].parse::<f64>().unwrap());
            let unit_numbers = match unit_def_hashmap.get(unit_name) {
                Some(n) => n,
                None => panic!("Error on ln 153, hashmap doesn't have {}, does have {:?}", unit_name, unit_def_hashmap.keys().collect::<Vec<_>>())
            };

            scale *= unit_numbers.scale.powf(exp);

            for i in 0..n_base_units {
                *base_units.get_mut(i).unwrap() += unit_numbers.base_units[i] * exp;
            }

        } else {
            let unit_numbers = unit_def_hashmap.get(unit_string).unwrap();

            scale *= unit_numbers.scale;

            for i in 0..n_base_units {
                *base_units.get_mut(i).unwrap() += unit_numbers.base_units[i];
            }
        }
    }

    // insert new unit into hashmap
    unit_def_hashmap.insert(name.clone(), UnitDefNumbers { scale, base_units: base_units.clone() });

    UnitDef { name, abbr, python_var_name, offset, scale, base_units }

}

fn write_derived_units_rs(unit_def_vec: &Vec<UnitDef>) {

    let mut derived_units_rs = "\
/// This file was generated automatically by the build script. 
/// If you want to add units, edit `unit_definitions\\derived_units.toml`
/// If you want to change file layout, edit `build.rs`

#[rustfmt::skip]
pub(crate) const OTHER_UNITS: &[(&'static str, &'static str, f64, f64".to_string();

    let n_base_units = unit_def_vec[0].base_units.len();

    derived_units_rs.push_str(&", f64".repeat(n_base_units));
    derived_units_rs.push_str(")] = &[");

    for unit_def in unit_def_vec {
        derived_units_rs.push_str(format!("\n    (\"{}\", \"{}\", {:?}, {:?}, ", unit_def.name, unit_def.abbr, unit_def.offset, unit_def.scale).as_str());
        for n in &unit_def.base_units {
            derived_units_rs.push_str(format!("{:?}, ", n).as_str());
        }
        derived_units_rs.push_str("),");
    }

    derived_units_rs.push_str("\n];");

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
        derived_units_py.push_str("')" );
    }

    fs::write(get_py_path("derived_units.py"), derived_units_py);

}

fn write_derived_units(unit_def_hashmap: &mut HashMap<String, UnitDefNumbers>) {

    let mut unit_def_vec: Vec<UnitDef> = vec![];

    let mut derived_unit_reader = csv::Reader::from_path(get_cfg_path("derived_units.csv")).expect("Could not locate derived units config");

    for line in derived_unit_reader.deserialize() {

        let line: DerivUnitConfigLine = line.expect("Could not parse line");
        
        unit_def_vec.push(get_unit_def_from_csv_line(line, unit_def_hashmap));

    }

    write_derived_units_rs(&unit_def_vec);
    write_derived_units_py(&unit_def_vec);

}

fn write_aliases() {

    let mut aliases_reader = csv::Reader::from_path(get_cfg_path("aliases.csv")).expect("Could not locate aliases config");

    let mut aliases_rs = "\
/// This file was generated automatically by the build script. 
/// If you want to add units, edit `unit_definitions\\derived_units.toml`
/// If you want to change file layout, edit `build.rs`

pub(crate) const UNIT_ALIASES: &[(&'static str, &'static str)] = &[".to_string();

    for line in aliases_reader.deserialize() {
        let (alias, name): (String, String) = line.expect("Could not parse line of aliases csv");
        aliases_rs.push_str(format!("\n    (\"{}\", \"{}\"),", alias, name).as_str());
    }

    aliases_rs.push_str("\n];");

    fs::write(get_rs_path("aliases.rs"), aliases_rs);

}

fn main() {

    let cfg_path: PathBuf = [
        Component::CurDir, 
        Component::Normal(UNIT_DEFINITONS_PATH.as_ref()), 
    ].iter().collect();

    println!(
        "cargo::rerun-if-changed={}", 
        cfg_path
            .as_path()
            .display()
    );

    let base_unit_config = read_base_units();

    write_base_units_py(&base_unit_config);

    let mut unit_def_hashmap = write_base_units_rs(&base_unit_config);

    write_derived_units(&mut unit_def_hashmap);
    
    write_aliases();
}