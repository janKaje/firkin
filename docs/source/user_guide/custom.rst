Custom Units
============

Custom Firkin units can be created with the `custom` classmethod:

>>> usd = Firkin.unit("USD")
>>> eur = Firkin.custom("euro", "EUR", 1.16537 * usd)
>>> eur
1 [EUR]
>>> usd.as_unit(eur)
0.858096570188009 [EUR]

Custom units created this way do not share all of the functionality of other Firkin units, as this would require a rewrite and recompile of the lookup tables in the underlying Python package.
Keep the following caveats in mind:

* Custom units cannot be accessed using string lookups
* If the Python variable containing the custom unit is deleted, the custom unit is out of scope and no longer accessible
* Custom units cannot be non-absolute temperature scales
* Base units can not be created using this method—if the definition does not include units, the custom unit will be considered dimensionless.

If you'd like to permanently add a new unit to Firkin's lookup tables, the process is relatively simple (as long as you know how to make forks and pull requests on GitHub). 
The definitions are contained in the GitHub repository under ``unit_definitions``. 
Simply fork the repository, insert the new unit definition(s) into the appropriate file(s), and open a pull request. 
Once the pull request is merged, the new unit will be available in the next Firkin package update.