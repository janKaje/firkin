Logarithmic Ratio Units
=======================

The decibel is the most commonly-used and well-known unit that encodes not a physical dimension, phenomenon, or even a set number of items (dozen, mole), but instead a ratio between two other quantities.
Every 10 decibels indicates a 10-fold increase in the quantity: 20 decibels means a 100-fold increase, 30 decibels means a 1000-fold increase, etc.

To implement these units in Firkin, we use a distinct class called the *LogFirkin*. Each LogFirkin can only contain one unit at a time.
When multiplied or divided by a number, or when added to or subtracted from another LogFirkin instance, it remains a LogFirkin. 
In any other case, it resolves the logarithmic ratio and turns into a floating point number.

>>> from firkin.units import decibel as dB, bel, neper
>>> ratio = 21*dB
>>> ratio
21 [dB]
>>> ratio2 = 2*bel
>>> ratio + ratio2
41 [dB]
>>> 2*ratio2
4 [B]
>>> ratio3 = 1.5*neper
>>> ratio3/1.5
1 [Np]
>>> ratio.as_unitless()
125.89254117941687
>>> ratio - 100
25.89254117941687
>>> 1.0/ratio
0.007943282347242805
>>> ratio2**2
10000.0

The LogFirkin class also includes the ``as_unit``, ``as_number``, and ``as_unitless`` methods for converting between log units.
Note that they work slightly differently: ``as_unitless`` resolves the logarithmic ratio, while ``as_number`` does not.

>>> my_ratio = 12*dB
>>> my_ratio
12 [dB]
>>> my_ratio.as_unitless()
15.848931924611145
>>> my_ratio.as_number()
12.0
>>> my_ratio.as_number(bel)
1.2000000000000002

These units are usually accessed through ``firkin.units``. 
They do not natively support prefixing, although that is planned in a future update. 
Currently, the only available log units are the bel, decibel, neper, and semitone.

