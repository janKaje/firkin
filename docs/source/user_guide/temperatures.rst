Temperatures
============

Certain temperature scales such as Celsius and Fahrenheit can have measurements below zero, something that generally doesn't happen with units.
After all, you can't have an object with negative length. 
These temperature scales are considered *non-absolute*, and dealing with them can lead to some tricky situations. 
10°C is equal to 50°F, but when the temperature goes up by 10°C, it only goes up by 18°F. 
It takes roughly 42 J to raise a gram of water by 10°C, but trying to raise its temperature by 283.15 K would take much, much more energy.

Different unit libraries and calculators have different ways to circumvent this problem: some have a built-in difference between actual/absolute and delta temperatures, and some ignore it entirely and will only deal in absolute temperature scales.
Firkin aims to be both convenient and accurate, so it has its own special way of dealing with these scales.

When a Firkin only contains a single temperature unit, it's assumed that the temperature is absolute, and converting between scales is possible:

>>> from firkin import Firkin
>>> degc = Firkin.unit("deg C")
>>> degf = Firkin.unit("deg F")
>>> (10*degc).as_unit(degf)
50.00399999999995 [°F]

When temperatures are used in addition or subtraction, the right hand side becomes a relative temperature:

>>> 10*degf + 10*degc
28 [°F]

And when the unit collection contains anything other than a single temperature unit, it acts as a relative temperature:

>>> (10*degc/"s").as_unit("deg F/s") # degrees per second
18 [°F/s]

This approach is based on my personal experience using temperatures in calculations. 
I hope that this aligns well with the needs of others in most every case, but if you find that your own use case needs an edge case addressed or a different ruleset entirely, please let me know and we can work to find a good solution.
