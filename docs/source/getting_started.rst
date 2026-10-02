.. _getting_started:

Getting Started
===============

.. _installation:

Installation
------------

Firkin is available through PyPI (pip):

.. code-block:: console

    $ pip install firkin_units

Accessing the units
-------------------

In your Python project, you can import units from the ``firkin.units`` submodule, which has all non-prefixed units already created for you, or perform a unit search using the ``Firkin.unit`` method.

>>> from firkin.units import ampere
>>> ampere
1 [A]
>>> from firkin import Firkin
>>> kJ = Firkin.unit("kilojoule") # You could also use "kJ" here
>>> kJ
1 [kJ]

Firkin is built primarily around the ``Firkin`` class, which contains two things: a value (number) and a unit collection. 
When used in Python arithmetic, the units do their own math under the hood to keep everything consistent.

>>> from firkin.units import meter
>>> length = 3 * meter
>>> width = 5.25 * meter
>>> area = length * width
>>> area
15.75 [m2]
>>> volume = 100 * meter ** 3
>>> volume/area
6.349206349206349 [m]

Firkin also comes with several physical constants built in, which can be accessed through ``firkin.constants`` or the ``Firkin.constant`` classmethod. 

>>> from firkin.constants import faraday_const, electron_charge
>>> faraday_const
96485.33212331001 [C/mol]
>>> Firkin.constant("avogadro")
602214076000000000000000 [mol]

.. _convert_string_arith:

Firkin can also convert strings into units on the fly, like so:

>>> volume/10/"kilometer"
0.01 [m2]
>>> length + "foot" # 3 m + 1 ft
3.3048 [m]

Note that you'll need to remember order of operations, as doing any other arithmetic to the string might cause errors:

>>> volume/10/"kilometer"**2
Traceback (most recent call last):
File "<stdin>", line 1, in <module>
volume/10/"kilometer"**2
          ~~~~~~~~~~~^^~
TypeError: unsupported operand type(s) for ** or pow(): 'str' and 'int'
>>> volume/10/"kilometer2" # this, however, should work fine
0.000009999999999999999 [m]

Unit Conversions
----------------

.. _as_unit:

as_unit, into_unit
^^^^^^^^^^^^^^^^^^

The primary way to convert a Firkin to a different unit is the ``as_unit`` method. You'll need to supply a set of units, whether it's with variables or a string query.

>>> from firkin.units import inch
>>> volume.as_unit(inch**3)
6102374.409473228 [in3]
>>> volume.as_unit("gal") # US customary gallon
26417.205235814843 [gal]

String queries can also be collections of units, formatted like so:

>>> kJ.as_unit("g.mile2/minute2")
1389.9677707528049 [g.mi2/min2]

The ``into_unit`` method does not return anything, instead interally modifying the variable to match the new units.

>>> volume.into_unit("gal")
>>> volume
26417.205235814843 [gal]

as_number, as_unitless
^^^^^^^^^^^^^^^^^^^^^^

These two methods return a plain number rather than a Firkin. The ``as_number`` method also needs a set of units, and the ``as_unitless`` will only work if the quantity is dimensionless overall. In essence, ``as_unitless`` is just a shorthand for ``as_number(1)``.

>>> kJ.as_number("kW.hr")
0.0002777777777777778
>>> (kJ/"kW.hr").as_unitless()
0.0002777777777777778

as_base_units
^^^^^^^^^^^^^

This method converts the Firkin into the 8 base units that represent the basic dimensions of measurement. 7 come from SI and there's one extra one for currency:

* second (s)
* meter (m)
* kilogram (kg)
* ampere (A)
* kelvin (K)
* mole (mol)
* candela (cd)
* USD (USD)

>>> from firkin.units import horsepower
>>> horsepower.as_base_units()
745.6998715822701 [kg.m2/s3]

Practical Example
-----------------

Say you want to analyze the rate of heat transfer across a single-pane window. You measure the window's area and thickness, which are 4.5 sq ft and 1/4 inch, respectively. Outside it's 100 °F and inside you keep it cool at 70 °F.

.. code-block:: python

    from firkin import Firkin
    from firkin.units import foot, inch, deg_F, watt

    # define window area
    window_area = 4.5 * foot ** 2
    window_width = 0.25 * inch

    # define temperatures
    outside_temperature = 100 * deg_F
    inside_temperature = 70 * deg_F

You find the thermal conductivity of the glass online. Using your engineering judgment, you assume some convection coefficients.

.. code-block:: python

    # define thermal conductivity
    glass_thermal_conductivity = 1.05 * watt / "m.K" # 1.05 W/m.K

    # assume values for convection coefficients
    inside_convection_coefficient = 2 * watt / "m2.K" # 2 W/m2.K
    outside_convection_coefficient = 10 * watt / "m2.K" # 10 W/m2.K

All that's left is the final calculation:

.. code-block:: python

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

Seems like it might be time to invest in some better-insulated windows.

That should be enough information to get you started with using Firkin. For more in-depth information, see the :doc:`user_guide/index` or :doc:`api/firkin`