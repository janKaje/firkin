Formatting
==========

Generally, when a Firkin is printed or otherwise turned into a string, it uses the unit symbols or abbreviations for each underlying unit, like so:

>>> from firkin.units import watt, hour
>>> energy_consumption = 15 * watt * hour
>>> energy_consumption
15 [W.hr]

You can use the ``descriptive`` method to instead use the unit names:

>>> energy_consumption.descriptive()
15 [hour.watt]

Or, to get the unit in LaTeX format:

>>> energy_consumption.latex()
15~\mathrm{W~hr}

Python's ``round`` function works well with Firkins, or you can use the ``round_sfig`` method to round to some number of significant figures:

>>> energy_consumption /= 7
>>> energy_consumption
2.142857142857143 [W.hr]
>>> round(energy_consumption, 3)
2.143 [W.hr]
>>> energy_consumption.round_sfig(3)
2.14 [W.hr]

