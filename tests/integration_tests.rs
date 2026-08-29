use pyo3::prelude::*;

fn run_closure<F>(closure: F) -> PyResult<()>
    where 
    F: for<'py> FnOnce(Python<'py>) -> PyResult<()> {
    Python::initialize();

    match Python::attach(closure) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn unit_lookups() -> PyResult<()> {
    run_closure(|py| {py.run(cr#"from firkin import Firkin
usd = Firkin.unit('USD')
tesla = Firkin.unit('tesla')
ugal = Firkin.unit('microgallon')
cal = Firkin.unit('cal')
cP = Firkin.unit('cP')
micron = Firkin.unit('micron')
kilometre = Firkin.unit('kilometre')
myriaAU = Firkin.unit('myriaAU')
assert str(myriaAU) == '1 [myriaua]' "#, None, None)
    })
}

#[test]
fn mulitple_unit_lookups() -> PyResult<()> {
    run_closure(|py| {py.run(cr#"from firkin import Firkin
newton = Firkin.unit('kg.m/s2')
assert str(newton) == '1 [kg.m/s2]' "#, None, None)
    })
}

#[test]
fn constant_lookups() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
gasconst = Firkin.constant("gas constant")
c_constant = Firkin.constant("light speed")
assert str(gasconst) == '8.31446261815324 [J/K.mol]'"#, None, None)
    })
}

#[test]
fn empty() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
empty = Firkin.empty()
assert str(empty) == '1 []'"#, None, None)
    })
}

#[test]
fn as_unit() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
joule = Firkin.unit('joule')
meter = Firkin.unit('meter')
newton = Firkin.unit('newton')
assert str(joule.as_unit("W.hr")) == '0.0002777777777777778 [W.hr]'
assert str(joule.as_unit(newton * meter)) == '1 [N.m]'"#, None, None)
    })
}

#[test]
fn nonabs_temp() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
degc = Firkin.unit('deg C')
degf = Firkin.unit('deg f')
assert str((10*degc).as_unit(degf)) == '50.00399999999995 [°F]', f"Was actually {(10*degc).as_unit(degf)}"
assert str(10*degf + 10*degc) == '28 [°F]', f"Was actually {10*degf + 10*degc}"
assert str((10*degc/"s").as_unit("deg F/s")) == '18 [°F/s]', f"Was actually {(10*degc/"s").as_unit("deg F/s")}""#, None, None)
    })
}

#[test]
fn as_number() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
usd = Firkin.unit("USD")
gbp = 1.35851 * usd
amt = 123.45 * usd
assert str(amt.as_number(gbp)) == '123.45'
assert str(amt.as_number(gbp, True)) == '90.87161669770558'
assert (amt/gbp).as_unitless() == amt.as_number(gbp, True)"#, None, None)
    })
}

#[test]
fn as_unitless() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
tsp = Firkin.unit('tsp')
tbsp = Firkin.unit('tbsp')
ratio = 4 * tsp / 17 / tbsp
ratio.as_unitless()
try:
    (4*tsp).as_unitless()
    raise ValueError('NOT UNITLESS')
except TypeError:
    pass"#, None, None)
    })
}

#[test]
fn as_base_units() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
tsp = Firkin.unit('tsp')
assert str(tsp.as_base_units()) == '0.00000492892159375 [m3]', f"Was actually {tsp.as_base_units()}""#, None, None)
    })
}

#[test]
fn descriptive() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
kdyn = Firkin.unit('kilodyne')
assert str(kdyn) == '1 [kdyn]'
assert str(kdyn.descriptive()) == '1 [kilodyne]'"#, None, None)
    })
}

#[test]
fn round_sfig() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
year = Firkin.unit('year')
assert str((year.as_unit('calendar year'))) == '1.0006635583041097 [calendar year]', f"Was actually {year.as_unit('calendar year')}"
assert str((year.as_unit('calendar year')).round_sfig(2)) == '1 [calendar year]'
assert str((year.as_unit('calendar year')).round_sfig(5)) == '1.0007 [calendar year]'
assert str((year.as_unit('calendar year')).round_sfig(7)) == '1.000664 [calendar year]'
"#, None, None)
    })
}

#[test]
fn consistency_checks() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
m2 = Firkin.unit('m2')

try:
    1 * m2 - "cm"
    raise ValueError('SUBTRACTED cm FROM m2')
except TypeError:
    pass

try:
    m2 / "meeeter"
    raise ValueError('DIVIDED meeeter FROM m2')
except TypeError:
    pass

try:
    m2 - 5.0
    raise ValueError('SUBTRACTED 5 FROM m2')
except TypeError:
    pass

inch = Firkin.unit('inch')

try:
    m2 ** inch
    raise ValueError('RAISED m2 TO THE inch')
except TypeError:
    pass

"#, None, None)
    })
}

#[test]
fn numpy_consistency_checks() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin import Firkin
try:
    import numpy as np
except ImportError:
    import math as np
m2 = Firkin.unit('m2')
inch = Firkin.unit('inch')
np.exp(inch**2/m2)

try:
    np.exp(inch/m2)
    raise ValueError('EXP ON inch/m2')
except TypeError:
    pass

"#, None, None)
    })
}

#[test]
fn unit_imports() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin.units import (
    ampere,
    USD,
    mole,
    weber,
    liter,
    light_speed
)

assert str(USD) == "1 [USD]"

"#, None, None)
    })
}

#[test]
fn constant_imports() -> PyResult<()> {
    run_closure(|py| {
        py.run(cr#"from firkin.constants import (
    reduced_planck_const,
    gas_const,
    vacuum_permeability
)

assert str(gas_const) == "8.31446261815324 [J/K.mol]"

"#, None, None)
    })
}