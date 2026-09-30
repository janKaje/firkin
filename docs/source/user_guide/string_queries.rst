Querying Unit Names
===================

Firkin's unit system is built off of a lookup table, where each unit has previously-defined values for dimensionality, scale, and offset (for certain temperature units). 
When coercing a string into a unit definition, Firkin accepts strings of the following format: *name* or *symbol*, immediately followed by the *exponent*, with units separated by a *period*, and the numerator and denominator separated by a *forward slash*.
The name or symbol can be prefixed. A list of examples of this formatting is shown below.

+------------------------+------------------------+
| Unit(s)                | Some of the possible   |
|                        | appropriate queries    |
+========================+========================+
| moles per liter        | - mol/L                |
|                        | - mole/liter           |
+------------------------+------------------------+
| square millimeter      | - mm2                  |
|                        | - millimeter2          |
+------------------------+------------------------+
| pound-foot (of torque) | - lb.ft                |
|                        | - pound.foot           |
+------------------------+------------------------+
| kilojoules per mol     | - kJ/mol.K             |
| Kelvin                 | - kilojoule/mole.K     |
+------------------------+------------------------+
| Hertz                  | - Hz                   |
|                        | - 1/s                  |
+------------------------+------------------------+

Many units also have certain inbuilt aliases to make lookups easier, i.e. *micron* is equivalent to *micrometer*, *deg C* is equivalent to *degree Celcius*, etc.

All prefixes from the `SI unit system <https://www.bipm.org/en/measurement-units/si-prefixes>`_ have been implemented, as well as the binary power prefixes *kibi-* through *quebi-*, and the mostly-obsolete decimal prefix *myria-*, which denotes a factor of 10\ :sup:`4`.

This algorithm of converting strings into units is also used when :ref:`converting strings on the fly <convert_string_arith>` and in the :ref:`as_unit and as_number methods <as_unit>`.