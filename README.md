# Firkin

[![PyPI Version](https://img.shields.io/pypi/v/firkin_units)](https://pypi.org/project/firkin_units/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/janKaje/firkin/CI.yml)](https://github.com/janKaje/firkin/actions)

Unit-attached numbers for scientific or engineering calculations.

Many scientists and engineers choose to use Python for their everyday calculations, since it strikes a nice balance between ease of use and versatility. Since Python doesn't natively include unit awareness, libraries such as [Unum](https://pypi.org/project/Unum/) or [Pint](https://pypi.org/project/Pint/) are used to make calculations both easier and safer.

Firkin is such a library, and boasts the following features:

* Written in Rust for speed and reliability
* Arbitrary prefix implementation: `centimeter`, `kilogallon`, `nanoinch of mercury`, and `exajiffy` are all valid Firkin units
* Compatibility for non-absolute temperature scales, so `10 °C` accurately maps to `50 °F`
* Includes physical constants, so you can easily access the `ideal gas constant` and `avogadro's number`
* Broad compatibility with mathematical functions to ensure dimensional consistency
* Flexibility and ease of use

## Installation

Firkin can be installed using pip:

```console
pip install firkin-units
```

## Getting started

The library is centered around the `Firkin` class. Each instance of `Firkin` contains both a value and a unit. To get a unit in Python, you can either import from `firkin.units` (contains all non-prefixed units, usually by their full name) or do a text search with the `Firkin.unit()` method.

```pycon
>>> from firkin.units import ampere, deg_C, firkin_mass
>>> ampere
1 [A]
>>> from firkin import Firkin
>>> kJ = Firkin.unit("kilojoule")
>>> kJ
1 [kJ]
```

You can access the constants through `firkin.constants` or the `Firkin.constant()` method.

```pycon
>>> from firkin.constants import faraday_const, electron_charge
>>> faraday_const
96485.33212331001 [C/mol]
>>> Firkin.constant("avogadro")
602214076000000000000000 [mol]
```

After defining units, you can use them as normal in basic arithmetic.

```pycon
>>> from firkin.units import meter
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
>>> length/width + 2 # plain numbers can only be added to a unitless instance
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

## Real-world example

Say you want to analyze the rate of heat transfer across a single-pane window. You measure the window's area and thickness, which are 4.5 sq ft and 1/4 inch, respectively. Outside it's 100 °F and inside you keep it cool at 70 °F.

```py
from firkin import Firkin
from firkin.units import foot, inch, deg_F, watt

# define window area
window_area = 4.5 * foot ** 2
window_width = 0.25 * inch

# define temperatures
outside_temperature = 100 * deg_F
inside_temperature = 70 * deg_F
```

You find the thermal conductivity of the glass online. Using your engineering judgment, you assume some convection coefficients.

```python
# define thermal conductivity
glass_thermal_conductivity = 1.05 * watt / "m.K" # 1.05 W/m.K

# assume values for convection coefficients
inside_convection_coefficient = 2 * watt / "m2.K" # 2 W/m2.K
outside_convection_coefficient = 10 * watt / "m2.K" # 10 W/m2.K
```

All that's left is the final calculation:

```python
# calculate thermal resistances 
inside_resistance = 1/inside_convection_coefficient/window_area
window_resistance = window_width/glass_thermal_conductivity/window_area
outside_resistance = 1/outside_convection_coefficient/window_area

# sum
overall_resistance = inside_resistance + window_resistance + outside_resistance

# calculate heat flow
heat_flow = (outside_temperature - inside_temperature)/overall_resistance

print(heat_flow.as_unit("W"))

# Output: 11.496997564233522 [W]
```

Seems like it might be time to invest in some better-insulated windows.

## Thanks to

Firkin was heavily inspired by both [Unum](https://pypi.org/project/Unum/) and [fend](https://github.com/printfn/fend). Some implementation details were taken or adapted from both, so many thanks to the creators and contributors of those projects.

## Contributing

Any contributions to Firkin are happily welcomed. Feel free to submit issues or pull requests to:

* Fix bugs
* Improve documentation
* Add features (see below for ideas/my own future plans)

## Future plans

* Proper support for logarithmic units
* Auto simplification, especially for units of different prefixes
* Improve case-sensitiveness for queries
* Add LaTeX formatting support
* Add support for arbitrary-precision numbers
* Add support for uncertainty, with basic error propagation
