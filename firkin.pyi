from typing import Self

class Firkin:

    """
    A unit-attached number that handles conversions and consistency checks
    during normal arithmetic and general use. Initialize with `Firkin.unit(query)`.

    Examples
    --------
    >>> from firkin import Firkin
    >>> meter = Firkin.unit("meter")
    >>> meter
    1 [m]
    
    >>> length = 10 * meter
    >>> width = 8 * meter
    >>> area = length * width
    >>> area
    80 [m2]

    >>> cm = Firkin.unit("cm")
    >>> area_remove = 1250 * cm ** 2
    >>> area - area_remove
    79.875 [m2]
    
    >>> area - 2 * cm
    Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
        area - 2*cm
    TypeError: [cm] cannot be coerced into [m2]

    ### Compatibility
    
    Firkin instances can do basic arithmetic and exponentiation with numbers,
    other Firkin instances, or strings. In the case of strings, it will be
    transformed into a Firkin instance the same way Firkin.unit() uses. If
    no corresponding unit is found, an error will be thrown.

    >>> joule = Firkin.unit("joule")
    >>> joule / "second"
    1 [J/s]

    >>> joule / "sssecond"
    Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
        joule / "sssecond"
    TypeError: unsupported operand type(s) for /: 'firkin.Firkin' and 'str'
    
    >>> joule + 2 * joule
    3 [J]
    
    >>> joule + 3
    Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
        joule + 3
    TypeError: [] cannot be coerced into [J]

    Likewise, when coercing a Firkin instance into a different unit or
    non-unit number, strings, numbers, and Firkins can all be used.

    >>> joule.as_unit("W.hr")
    0.0002777777777777778 [hr.W]

    >>> newton = Firkin.unit("newton")
    >>> joule.as_number(newton * meter)
    1

    ### Temperatures

    Temperature units that can go below zero (most commonly Celsius and 
    Fahrenheit) act in a unique way. By themselves, they usually act as 
    absolute temperatures, meaning you can convert between them in a way that
    makes sense:

    >>> degc = Firkin.unit("deg C")
    >>> degf = Firkin.unit("deg F")
    >>> (10*degc).as_unit(degf)
    50.00399999999995 [°F]

    When used in addition or subtraction, the right hand sides become relative
    temperatures:

    >>> 10*degf + 10*degc
    28 [°F]

    And when the unit contains anything other than a single temperature unit, 
    it acts as a relative temperature:

    >>> (10*degc/"s").as_unit("deg F/s") # degrees per second
    18 [°F/s]

    ### Non-arithmetic math functions

    Non-arithmetic math functions such as trigonometric functions, 
    exponentiation, and logarithms don't accept units in some or all of their 
    arguments. For such cases, you will need to either ensure the Firkin
    has no units, or call `as_number()` on the instance to remove the units.

    >>> inch = Firkin.unit("inch")
    >>> cm = Firkin.unit("cm")
    >>> inch**2 # an example of allowed exponentiation
    1 [in2]

    >>> inch**cm # an example of disallowed exponentiation
    Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
        inch**cm
    TypeError: [cm] and [] are not 
    
    >>> unitless = inch/cm
    >>> inch**unitless # not generally helpful, but doable
    1 [in2.54]

    While python's `math` module tends to bypass unit checks by converting
    everything to a float, numpy does not. As such, it's recommended to use
    numpy's math functions over `math`'s to ensure proper unit checking.

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

    Currently, only `exp`, `log`, and `log10` are implemented this way. For any
    other functions, consider using the .as_unitless() method:

    >>> np.sin((degc/degf).as_unitless())
    0.9738476308781953
    """

    @classmethod
    def unit(cls, query:str) -> Self: 
        """
        Create a new Firkin instance by searching for a unit name or symbol.

        Parameters
        ----------
        query : str
            The query by which to look up units in the database.
            Unit names and symbols are both allowed. Prefixes such as kilo or
            milli can be attached to unit names, and prefix symbols such as
            M or c can be attached to unit symbols.

            The query can also contain multiple units, separated by . and / as
            unit representations are. See examples below.

        Returns
        -------
        Firkin
            The new Firkin instance.

        Raises
        ------
        LookupError
            If the unit name does not correspond to an entry in the database.

        Examples
        --------
        >>> from firkin import Firkin
        >>> Firkin.unit("liter")
        1 [L]
        >>> Firkin.unit("kilomile")
        1 [kmi]
        >>> Firkin.unit("megamile").as_unit("mile")
        1000000 [mi]
        >>> Firkin.unit("eV")
        1 [eV]

        >>> newton = Firkin.unit("kg.m/s2")
        >>> newton
        1 [kg.m/s2]
        >>> newton.as_unit("N")
        1 [N]
        >>> newton * Firkin.unit("second")**2
        1 [kg.m]
        """
        ...

    @classmethod
    def constant(cls, query:str) -> Self: 
        """
        Create a new Firkin instance by searching for the name of a constant.

        Parameters
        ----------
        query : str
            The query by which to find the constant. Generally the constant's
            name, but can be an abbreviation (i.e. `c` for `light speed`)

        Returns
        -------
        Firkin
            The new Firkin instance.

        Raises
        ------
        LookupError
            If the constant name does not correspond to an entry in the database.

        Examples
        --------
        >>> from firkin import Firkin
        >>> Firkin.constant("gas constant")
        8.31446261815324 [J/K.mol]

        Due to the fact that the speed of light is sometimes used as a unit in
        certain fields of physics, it can be used as either a unit or a 
        constant.

        >>> c_unit = Firkin.unit("light speed")
        >>> c_unit
        1 [c]
        >>> c_constant = Firkin.constant("light speed")
        >>> c_constant
        299792458 [m/s]
        >>> c_constant.as_unit(c_unit)
        1 [c]
        >>> ("eV"/c_unit**2).as_unit("kg")
        0.0000000000000000000000000000000000017826656668079864 [kg]
        """
        ...

    def as_unit(self, other:Self|float|int|str) -> Self: 
        """
        Returns a unit idential to self, but with the units of other.

        Parameters
        ----------
        other : Firkin, float, int, str
            The units to coerce self into. Strings will attempt to use .unit()
            algorithm, and numbers will be considered unitless.

        Returns
        -------
        Firkin
            The coerced value of self.

        Raises
        ------
        TypeError
            If the units of self and other are incompatible.
        """
        ...

    def as_number(self, other:Self|float|int|str|None=None) -> float: 
        """
        Similar to the .as_unit() method, but returns itself as a number.

        Parameters
        ----------
        other : Firkin, float, int, str, None, default None
            The units to coerce self into. If None, will return without 
            altering the units. Strings will attempt to use .unit() algorithm, 
            and numbers will be considered unitless.

        Returns
        -------
        float
            The numerical value of self, in the units of other.

        Raises
        ------
        TypeError
            If the units of self and other are incompatible.
        """
        ...

    def as_unitless(self) -> float: 
        """
        If the object is unitless, returns its numerical value. Otherwise an 
        error is raised.

        Returns
        -------
        float
            The numerical value of self.

        Raises
        ------
        TypeError
            If self is not unitless.
        """
        ...

    def descriptive(self) -> str: 
        """
        Returns a more descriptive version of the usual unit string, with unit 
        symbols replaced by unit names.
        """
        ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def __mul__(self, other:Self|float|int|str) -> Self: ...

    def __rmul__(self, other:Self|float|int|str) -> Self: ...

    def __div__(self, other:Self|float|int|str) -> Self: ...

    def __truediv__(self, other:Self|float|int|str) -> Self: ...

    def __rdiv__(self, other:Self|float|int|str) -> Self: ...

    def __rtruediv__(self, other:Self|float|int|str) -> Self: ...

    def __pos__(self) -> Self: ...

    def __neg__(self) -> Self: ...

    def __add__(self, other:Self|float|int|str) -> Self: ...

    def __radd__(self, other:Self|float|int|str) -> Self: ...

    def __sub__(self, other:Self|float|int|str) -> Self: ...

    def __rsub__(self, other:Self|float|int|str) -> Self: ...

    def __pow__(self, other:Self|float|int|str, modulus:float|int|None=None) -> Self: ...

    def __rpow__(self, other:Self|float|int|str, modulus:float|int|None=None) -> Self: ...

    def __lt__(self, other:Self|float|int|str) -> Self: ...

    def __le__(self, other:Self|float|int|str) -> Self: ...

    def __gt__(self, other:Self|float|int|str) -> Self: ...

    def __ge__(self, other:Self|float|int|str) -> Self: ...

    def __eq__(self, other:Self|float|int|str) -> Self: ...

    def __ne__(self, other:Self|float|int|str) -> Self: ...

    def __abs__(self) -> Self: ...

    def __int__(self) -> int: ...

    def __float__(self) -> float: ...

    def __round__(self, ndigits:int=None) -> Self: ...

    def __exp__(self) -> float: ...

    def __log__(self) -> float: ...

    def __log10__(self) -> float: ...