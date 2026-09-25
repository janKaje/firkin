.. Firkin documentation master file, created by
   sphinx-quickstart on Fri Sep 25 12:29:27 2026.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Firkin documentation
====================

**Firkin** is a Python library for scientists and engineers that provides unit-attached numbers for both complex and everyday calculations. 

.. note::

   This project is under active development. Contributions are happily welcomed.

Scientific and engineering calculations frequently require professionals, researchers, and students alike to constantly be aware of units: unit systems, unit conversions, dimensional analysis, etc. This is at best inconvenient and at worst `disastrous <https://en.wikipedia.org/wiki/Mars_Climate_Orbiter>`_. 
Firkin aims to reduce the burden of unit systems, by integrating them into normal Python scripting in an easy and intuitive way. It's also been written in Rust to improve performance, although thorough benchmarks will need to wait until the library is more complete.

Other notable features include:

* Arbitrary prefix implementation: ``centimeter``, ``kilogallon``, ``nanoinch of mercury``, and ``exajiffy`` are all valid Firkin units
* Compatibility for non-absolute temperature scales, so ``10 °C`` accurately maps to ``50 °F``
* Inbuilt physical constants, so you can easily access the ``ideal gas constant`` and ``avogadro's number``
* Broad compatibility with mathematical functions
* Automatic string parsing: ``meter/"second"`` correctly interprets itself as ``meter/second``
* Compatibility for logarithmic units such as the ``decibel`` and ``semitone``

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   Getting Started <getting_started>
   User Guide <user_guide/index>
   API Documentation <api/index>