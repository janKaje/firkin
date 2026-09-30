# Firkin

[![PyPI Version](https://img.shields.io/pypi/v/firkin_units)](https://pypi.org/project/firkin_units/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/janKaje/firkin/CI.yml)](https://github.com/janKaje/firkin/actions)

Unit-attached numbers for scientific or engineering calculations.

> [!NOTE]
> These links are broken, but will be fixed after the 0.3.1 release.

* Getting Started
* User Guide
* API Documentation

## About

**Firkin** is a Python library for scientists and engineers that provides unit-attached numbers for both complex and everyday calculations.

> [!NOTE]
> This project is under active development. Contributions are welcomed.

Scientific and engineering calculations frequently require professionals, researchers, and students alike to constantly be aware of units: unit systems, unit conversions, dimensional analysis, etc. This is at best inconvenient and at worst [disastrous](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter). Firkin aims to reduce the burden of unit systems by integrating them into normal Python scripting in an easy and intuitive way. It's also been written in Rust to improve performance, although thorough benchmarks will need to wait until the library is more complete.

Other notable features include:

* Arbitrary prefix implementation: ``centimeter``, ``kilogallon``, ``nanoinch of mercury``, and ``exajiffy`` are all valid Firkin units
* Compatibility for non-absolute temperature scales, so ``10 °C`` accurately maps to ``50 °F``
* Inbuilt physical constants, so you can easily access the ``ideal gas constant`` and ``avogadro's number``
* Broad compatibility with mathematical functions
* Automatic string parsing: ``meter/"second"`` correctly interprets itself as ``meter/second``
* Compatibility for logarithmic units such as the ``decibel`` and ``semitone``

## Thanks to

Firkin has been heavily inspired by both [Unum](https://pypi.org/project/Unum/) and [fend](https://github.com/printfn/fend). Some implementation details were taken or adapted from both, so many thanks to the creators and contributors of those projects.

## Contributing

Any contributions to Firkin are happily welcomed. Feel free to submit issues or pull requests about:

* Fixing bugs
* Improving documentation
* Expanding tests
* Adding features (see below for ideas/my own future plans)

## Future plans

* Add benchmarks to measure performance against Pint or Unum
* Add support for arbitrary-precision numbers
* Add support for uncertainty, with linear error propagation
* Improve support for numpy ufuncs
* Proper currency handling, account for both inflation and exchange rates
* Python-bound config
* Matplotlib support
