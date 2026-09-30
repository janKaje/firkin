Non-arithmetic Functions
========================

Non-arithmetic functions generally require their inputs to be unitless (i.e. trigonometric functions, logarthims, the exponent of exponentiation).
For such cases, you will need to either ensure the Firkin has no units. 
You can also call the ``as_number`` method to force-remove the units, but this isn't recommended.

>>> from firkin import Firkin
>>> inch = Firkin.unit("inch")
>>> cm = Firkin.unit("cm")
>>> inch**2 # an example of allowed exponentiation
1 [in2]
>>> inch**cm # an example of disallowed exponentiation
Traceback (most recent call last):
File "<stdin>", line 1, in <module>
    inch**cm
TypeError: [cm] and [] are not compatible
>>> unitless = inch/cm
>>> inch**unitless # not generally helpful, but doable
1 [in2.54]

While python's `math` module tends to bypass unit checks by calling the Firkin's ``__float__`` method, numpy does not. 
As such, it's recommended to use numpy's math functions over `math`'s to ensure proper unit checking.

>>> import numpy as np
>>> import math
>>> math.exp(degc)
2.718281828459045
>>> np.exp(degc)
Traceback (most recent call last):
File "<stdin>", line 1, in <module>
    np.exp(degc)
    ^^^^^^^^^^^^
TypeError: [°C] and [] are not compatible
>>> np.log(degc/degf)
0.587786664902119

Currently, only `exp`, `log`, and `log10` are implemented this way. 
For any other functions, consider using the `as_unitless` method:

>>> np.sin((degc/degf).as_unitless())
0.9738476308781953

Support for more mathematical functions is planned for a future update.