# Firkin

Unit-attached numbers for scientific or engineering calculations.

Many scientists and engineers choose to use Python for their everyday calculations, since it strikes a nice balance between ease of use and versatility. However, the lack of unit awareness in calculations can be dangerous, so the use of libraries such as [Unum](https://pypi.org/project/Unum/) or [Pint](https://pypi.org/project/Pint/) is common.

Firkin is an alternative to these, and boasts the following features:

* Written in Rust for speed and reliability
* Arbitrary prefix implementation: `centimeter`, `kilogallon`, `nanoinch of mercury`, and `exajiffy` are all valid Firkin units
* Compatibility for non-absolute temperature scales, so `10 °C` accurately maps to `50 °F`
* Includes physical constants, so you can easily access the `ideal gas constant` and `avogadro's number`
* Broad compatibility with mathematical functions to ensure dimensional consistency
* Extreme ease of use, in both short and long scripts

## Getting started

Currently, Firkin is only available here, as source code. That should change in the future (hopefully!).

To install, clone the repository, make sure you have [Rust](https://rust-lang.org/) and [Maturin](https://github.com/PyO3/maturin) installed, and use maturin to develop the package.

The `Firkin` class is currently the only interface to the library. Each instance of `Firkin` contains both a value and a unit. Instantiate with either of these methods:

```pycon
>>> from firkin import Firkin
>>> meter = Firkin.unit("meter") # 1 meter
>>> c = Firkin.constant("light speed") # 299792458 m/s
```

After defining units, you can use them as normal in basic arithmetic.

```pycon
>>> length = 3 * meter
>>> width = 5.25 * meter
>>> area = length * width
>>> area
15.75 [m2]
>>> volume = 100 * meter ** 3
>>> volume/area
6.349206349206349 [m]
```

One of Firkin's biggest strengths is its ability to coerce both numbers and strings into Firkins.

```pycon
>>> speed = length/"second"
>>> speed
3 [m/s]
>>> longer_length = length + "foot"
>>> longer_length # 3 m + 1 ft
3.3048 [m]
>>> length/width + 2 # plain numbers must be added to a unitless instance
2.571428571428571 []
```

The algorithm used to turn strings into Firkins can accept both names and symbols, prefixes or no prefixes, and multiple units at a time.

```pycon
>>> joule = Firkin.unit("J") 
>>> joule.as_unit("kg.m2/s2") # kilogram meter^2 second^-2
1 [kg.m2/s2]
>>> joule + "kilonewton.meter"
1001 [J]
```

When unit inconsistencies are found, Firkin will raise an error.

```pycon
>>> meter ** 2 - meter
Traceback (most recent call last):
  File <stdin>, line 1, in <module>
    meter ** 2 - meter
    ~~~~~~~~~~~^~~~~~~
TypeError: [m2] and [m] are not compatible
>>> joule ** meter
Traceback (most recent call last):
  File <stdin>, line 1, in <module>
    joule ** meter
    ~~~~~~^^~~~~~~
TypeError: [m2] and [m] are not compatible
>>> some_exponent = "foot" / meter
>>> joule ** some_exponent # allowed, since some_exponent is unitless
1 [J0.3048]
```

When you need to express a Firkin as a certain unit, use the `as_unit` or `as_number` methods.

```pycon
>>> joule.as_unit("W.hr")
0.0002777777777777778 [W.hr]
>>> tire_pressure = 15 * Firkin.unit("psi")
>>> tire_pressure.as_number("kPa") 
103.39285714285714
```

Physical constants can be accessed through `Firkin.constant` as mentioned previously. The speed of light, due to its prevalence as a unit in and of itself in certain fields of physics, is available both as a unit and a constant.

```pycon
>>> c_unit = Firkin.unit("light speed")
>>> c_unit
1 [c]
>>> c_constant = Firkin.constant("light speed")
>>> c_constant
299792458 [m/s]
>>> c_constant.as_unit(c_unit)
1 [c]
>>> ("keV"/c_unit**2).as_unit("amu") # keV/c2 as atomic mass unit
0.000001073545754277516 [amu]
```

See the inbuilt documentation for more detailed information.
