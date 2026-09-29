//! Unit-attached numbers for scientific or engineering calculations.

use pyo3::prelude::*;

mod constant;
mod error;
mod unit;

/// A Python module implemented in Rust.
#[pymodule]
mod firkin {
    use pyo3::{prelude::*, types::*};
    use std::{fmt, ops::Mul};

    use crate::constant::search_for_constant_name;
    use crate::error::FirkinError;
    use crate::unit::LogUnit;
    use crate::unit::UnitCollection;
    use crate::unit::search_for_log_unit_name;

    /// A unit-attached number that handles conversions and consistency checks
    /// during normal arithmetic and general use. Initialize with the ``unit``,
    /// ``constant``, ``empty``, or ``custom`` classmethods, or by importing
    /// prebuilt units from ``firkin.units``.
    #[pyclass(from_py_object)]
    #[derive(Clone)]
    struct Firkin {
        unit_collection: UnitCollection,
        value: f64,
    }

    #[pymethods]
    impl Firkin {
        /// Create a new Firkin instance by searching for a unit name or symbol.
        ///
        /// :param query: The query by which to look up units in the database.
        ///     Unit names and symbols are both allowed. Prefixes such as kilo or
        ///     milli can be attached to unit names, and prefix symbols such as
        ///     M or c can be attached to unit symbols.
        ///
        ///     The query can also contain multiple units, separated by . and / as
        ///     unit representations are. See examples below.
        /// :type query: str
        /// :return: The new Firkin instance.
        /// :rtype: Firkin
        /// :raise LookupError: If the unit name does not correspond to an entry in the database.
        ///
        /// Examples
        /// ^^^^^^^^
        /// >>> from firkin import Firkin
        /// >>> Firkin.unit("liter")
        /// 1 [L]
        /// >>> Firkin.unit("kilomile")
        /// 1 [kmi]
        /// >>> Firkin.unit("megamile").as_unit("mile")
        /// 1000000 [mi]
        /// >>> Firkin.unit("eV")
        /// 1 [eV]
        ///
        /// >>> newton = Firkin.unit("kg.m/s2")
        /// >>> newton
        /// 1 [kg.m/s2]
        /// >>> newton.as_unit("N")
        /// 1 [N]
        /// >>> newton * Firkin.unit("second")**2
        /// 1 [kg.m]
        #[classmethod]
        fn unit(_cls: &Bound<'_, PyType>, query: &str) -> PyResult<Self> {
            let unit = match UnitCollection::from_unit_name(query) {
                Some(unit) => unit,
                None => {
                    return Err(FirkinError::UnitNotFound(query.to_string()).into());
                }
            };

            Ok(Firkin {
                unit_collection: unit,
                value: 1.0,
            })
        }

        /** Create a new Firkin instance by searching for the name of a constant.

        :param query: The query by which to find the constant. Generally the constant's
            name, but can be an abbreviation (i.e. `c` for `light speed`)
        :type query: str
        :return: The new Firkin instance
        :rtype: Firkin
        :raise LookupError: If the constant name does not correspond to an entry in the database.

        Examples
        ^^^^^^^^
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
        >>> ("keV"/c_unit**2).as_unit("amu") # keV/c2 as atomic mass unit
        0.000001073545754277516 [amu] */
        #[classmethod]
        fn constant(_cls: &Bound<'_, PyType>, query: &str) -> PyResult<Self> {
            match Firkin::constant_query_internal(query) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        /// Returns a Firkin with value 1 and no units attached.
        #[classmethod]
        fn empty(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
            Ok(Firkin::empty_unit())
        }

        /** Defines a custom Firkin unit. 
        
        Note that this custom unit cannot be 
        accessed using string lookups, and is solely tied to the Firkin 
        returned by this method. The unit also cannot be a non-absolute
        temperature unit.
        
        Base units can not be created using this 
        method either—if the definition does not include units, the custom unit
        will be considered dimensionless.

        :param name: The name of the new unit.
        :type name: str
        :param abbr: The abbreviation or unit symbol for the new unit.
        :type abbr: str
        :param definition: The definition of the custom unit.
        :type definition: Firkin, LogFirkin, float, int, or str
        :return: The newly made custom unit.
        :rtype: Firkin

        Examples
        ^^^^^^^^
        >>> usd = Firkin.unit("USD")
        >>> eur = Firkin.custom("euro", "EUR", 1.16537 * usd)
        >>> eur
        1 [EUR]
        >>> usd.as_unit(eur)
        0.858096570188009 [EUR] */
        #[classmethod]
        fn custom(
            _cls: &Bound<'_, PyType>,
            name: String,
            abbr: String,
            definition: UnitCoercible,
        ) -> PyResult<Self> {
            let definition: Firkin = definition.into();
            let definition: UnitCollection = definition.into();
            let unit = definition.to_single_unit(name, abbr);
            Ok(Firkin {
                unit_collection: UnitCollection::coerce_unit_to_collection(unit),
                value: 1.0,
            })
        }

        /** Returns a unit idential to self, but with the units of other.

        :param other: The units to coerce self into. Strings will attempt to use .unit()
            algorithm, and numbers will be considered unitless.
        :type other: Firkin, LogFirkin, float, int, str
        :return: The unit identical to self with the units of other.
        :rtype: Firkin
        :raise TypeError: If the units of self and other are incompatible. */
        fn as_unit(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            match self.as_unit_internal(&other.unit_collection) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        /** Similar to the .as_unit() method, but returns itself as a number.

        :param other: 
            The units to coerce self into. If None, will return without 
            altering the units. Strings will attempt to use .unit() algorithm, 
            and numbers will be considered unitless.
        :type other: Firkin, LogFirkin, float, int, str, None, default None
        :param scale: If true, returns the equivalent of (self/other).as_unitless().
        :type scale: bool, default False
        :return: The numerical value of self, in the units of other.
        :rtype: float
        :raise TypeError: If the units of self and other are incompatible.

        Examples
        ^^^^^^^^

        >>> from firkin import Firkin
        >>> usd = Firkin.unit("USD")
        >>> gbp = 1.35851 * usd
        >>> amt = 123.45 * usd
        >>> amt.as_number(gbp) # trying to convert from usd to gbp
        123.45
        >>> amt.as_number(gbp, True) # correct response
        90.87161669770558
        >>> (amt/gbp).as_unitless() # equivalent to prev
        90.87161669770558
        >>> amt/gbp # returns a unitless Firkin instance
        90.87161669770558 [] */
        #[pyo3(signature = (other=None, scale=false))]
        fn as_number(&self, other: Option<UnitCoercible>, scale: bool) -> PyResult<f64> {
            match other {
                Some(other) => {
                    let other: Firkin = other.into();
                    match self.as_number_internal(
                        &other.unit_collection,
                        if scale { other.value } else { 1.0 },
                    ) {
                        Ok(r) => Ok(r),
                        Err(e) => Err(e.into()),
                    }
                }
                None => Ok(self.value),
            }
        }

        /** If the object is unitless, returns its numerical value. Otherwise an 
        error is raised.

        :return: The numerical value of self.
        :rtype: float
        :raise TypeError: If self is not unitless. */
        fn as_unitless(&self) -> PyResult<f64> {
            match self.as_number_internal(&UnitCollection::empty_collection(), 1.0) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        /** Similar to as_unit, but takes no arguments and instead returns self as
        base units (SI units + USD for currency) */
        fn as_base_units(&self) -> PyResult<Firkin> {
            match self.as_unit_internal(&self.unit_collection.get_base_units()) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        /** Returns a copy of self, with the value rounded to n_sig_figs
        significant figures. Similar to self.__round__(). */
        fn round_sfig(&mut self, n_sig_figs: i32) -> PyResult<Firkin> {
            self.__round__(Some(
                n_sig_figs - 1 - (self.value.abs().log10().floor() as i32),
            ))
        }

        /// Simplifies the units of self, removing dimensionless unit subsets.
        fn simplify(&self) -> PyResult<Firkin> {
            match self.simplify_internal() {
                Ok(i) => Ok(i),
                Err(e) => Err(e.into()),
            }
        }

        /** Simplifies the units of self, removing dimensionless unit subsets.
        Modifies self in place without returning anything. */
        fn simplify_inplace(&mut self) -> PyResult<()> {
            self.simplify_inplace_internal()?;
            Ok(())
        }

        fn __str__(&mut self) -> PyResult<String> {
            self.ss_inpl_internal()?;
            Ok(format!("{}", self))
        }

        fn __repr__(&mut self) -> PyResult<String> {
            self.__str__()
        }

        fn __mul__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            Ok(Firkin {
                unit_collection: self.unit_collection.clone() * other.unit_collection.clone(),
                value: self.value * other.value,
            }
            .simple_simplify_internal()?)
        }

        fn __rmul__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__mul__(other)
        }

        fn __div__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            Ok(Firkin {
                unit_collection: self.unit_collection.clone() / other.unit_collection.clone(),
                value: self.value / other.value,
            }
            .simple_simplify_internal()?)
        }

        fn __truediv__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__div__(other)
        }

        fn __rdiv__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            Ok(Firkin {
                unit_collection: other.unit_collection.clone() / self.unit_collection.clone(),
                value: other.value / self.value,
            }
            .simple_simplify_internal()?)
        }

        fn __rtruediv__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__rdiv__(other)
        }

        fn __pos__(&self) -> PyResult<Firkin> {
            Ok(self.simple_simplify_internal()?)
        }

        fn __neg__(&self) -> PyResult<Firkin> {
            Ok(Firkin {
                unit_collection: self.unit_collection.clone(),
                value: -self.value,
            }
            .simple_simplify_internal()?)
        }

        fn __add__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            if let Some(scale_diff) = self
                .unit_collection
                .equivalent_scale_diff(&other.unit_collection)
            {
                Ok(Firkin {
                    unit_collection: self.unit_collection.clone(),
                    value: self.value + other.value * scale_diff,
                }
                .simple_simplify_internal()?)
            } else {
                Err(FirkinError::IncompatibleUnits {
                    first: self.unit_collection.as_string(),
                    second: other.unit_collection.as_string(),
                }
                .into())
            }
        }

        fn __radd__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__add__(other)
        }

        fn __sub__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            if let Some(scale_diff) = self
                .unit_collection
                .equivalent_scale_diff(&other.unit_collection)
            {
                Ok(Firkin {
                    unit_collection: self.unit_collection.clone(),
                    value: self.value - other.value * scale_diff,
                }
                .simple_simplify_internal()?)
            } else {
                Err(FirkinError::IncompatibleUnits {
                    first: self.unit_collection.as_string(),
                    second: other.unit_collection.as_string(),
                }
                .into())
            }
        }

        fn __rsub__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            if let Some(scale_diff) = self
                .unit_collection
                .equivalent_scale_diff(&other.unit_collection)
            {
                Ok(Firkin {
                    unit_collection: self.unit_collection.clone(),
                    value: other.value - self.value * scale_diff,
                }
                .simple_simplify_internal()?)
            } else {
                Err(FirkinError::IncompatibleUnits {
                    first: self.unit_collection.as_string(),
                    second: other.unit_collection.as_string(),
                }
                .into())
            }
        }

        fn __pow__(&self, exponent: UnitCoercible, _modulus: Option<PyNumber>) -> PyResult<Firkin> {
            let exponent: Firkin = exponent.into();
            let exponent = exponent.as_number_internal(&UnitCollection::empty_collection(), 1.0)?;
            Ok(Firkin {
                unit_collection: self.unit_collection.pow(exponent),
                value: self.value.powf(exponent),
            }
            .simple_simplify_internal()?)
        }

        fn __rpow__(&self, other: UnitCoercible, _modulus: Option<PyNumber>) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            let exponent = self.as_number_internal(&UnitCollection::empty_collection(), 1.0)?;
            Ok(Firkin {
                unit_collection: other.unit_collection.pow(exponent),
                value: other.value.powf(exponent),
            }
            .simple_simplify_internal()?)
        }

        fn __lt__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value < other.as_unit_internal(&self.unit_collection)?.value)
        }

        fn __le__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value <= other.as_unit_internal(&self.unit_collection)?.value)
        }

        fn __gt__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value > other.as_unit_internal(&self.unit_collection)?.value)
        }

        fn __ge__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value >= other.as_unit_internal(&self.unit_collection)?.value)
        }

        fn __eq__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value == other.as_unit_internal(&self.unit_collection)?.value)
        }

        fn __ne__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value != other.as_unit_internal(&self.unit_collection)?.value)
        }

        fn __abs__(&self) -> PyResult<Firkin> {
            Ok(Firkin {
                unit_collection: self.unit_collection.clone(),
                value: self.value.abs(),
            })
        }

        fn __int__(&self) -> PyResult<i32> {
            Ok(self.value as i32)
        }

        fn __float__(&self) -> PyResult<f64> {
            Ok(self.value)
        }

        /// Round the internal value to ndigits decimal places.
        #[pyo3(signature=(ndigits=None))]
        fn __round__(&mut self, ndigits: Option<i32>) -> PyResult<Firkin> {
            self.ss_inpl_internal()?;
            match ndigits {
                Some(n) => {
                    let mul = 10.0f64.powi(n);
                    Ok(Firkin {
                        unit_collection: self.unit_collection.clone(),
                        value: (self.value * mul).round() / mul,
                    })
                }
                None => Ok(Firkin {
                    unit_collection: self.unit_collection.clone(),
                    value: self.value.round(),
                }),
            }
        }

        fn __exp__(&self) -> PyResult<f64> {
            let exponent = self.as_number_internal(&UnitCollection::empty_collection(), 1.0)?;
            Ok(exponent.exp())
        }

        fn exp(&self) -> PyResult<f64> {
            self.__exp__()
        }

        fn __log__(&self) -> PyResult<f64> {
            let exponent = self.as_number_internal(&UnitCollection::empty_collection(), 1.0)?;
            Ok(exponent.ln())
        }

        fn log(&self) -> PyResult<f64> {
            self.__log__()
        }

        fn __log10__(&self) -> PyResult<f64> {
            let exponent = self.as_number_internal(&UnitCollection::empty_collection(), 1.0)?;
            Ok(exponent.log10())
        }

        fn log10(&self) -> PyResult<f64> {
            self.__log10__()
        }

        /** Returns a more descriptive version of the usual unit string, with unit 
        symbols replaced by unit names. */
        fn descriptive(&mut self) -> PyResult<String> {
            self.ss_inpl_internal()?;
            Ok(format!(
                "{} {}",
                self.value,
                self.unit_collection.as_descriptive_string()
            ))
        }

        /// Returns a LaTeX-formatted string representing the object.
        fn latex(&mut self) -> PyResult<String> {
            self.ss_inpl_internal()?;
            Ok(format!(
                "{}~{}",
                self.value,
                self.unit_collection.as_latex()
            ))
        }
    }

    impl Firkin {
        fn as_unit_internal(&self, other: &UnitCollection) -> Result<Firkin, FirkinError> {
            if let Some(scale_diff) = self.unit_collection.equivalent_scale_diff(&other) {
                if let Some(self_offset) = self.unit_collection.offset()
                    && let Some(other_offset) = other.offset()
                {
                    Ok(Firkin {
                        unit_collection: other.clone(),
                        value: {
                            (self.value * self.unit_collection.scale - self_offset + other_offset)
                                / other.scale
                        },
                    })
                } else {
                    Ok(Firkin {
                        unit_collection: other.clone(),
                        value: self.value / scale_diff,
                    })
                }
            } else {
                Err(FirkinError::IncompatibleUnits {
                    first: self.unit_collection.as_string(),
                    second: other.as_string(),
                })
            }
        }

        fn as_number_internal(
            &self,
            other: &UnitCollection,
            scale: f64,
        ) -> Result<f64, FirkinError> {
            match self.as_unit_internal(other) {
                Ok(u) => Ok(u.value / scale),
                Err(e) => Err(e),
            }
        }

        fn empty_unit() -> Firkin {
            Firkin {
                unit_collection: UnitCollection::empty_collection(),
                value: 1.0,
            }
        }

        fn constant_query_internal(query: &str) -> Result<Firkin, FirkinError> {
            match search_for_constant_name(query) {
                None => Err(FirkinError::ConstantNotFound(query.to_string())),
                Some(const_def) => Ok(Firkin {
                    unit_collection: match UnitCollection::from_unit_name(const_def.2) {
                        Some(r) => r,
                        None => return Err(FirkinError::UnitNotFound(const_def.2.to_string())),
                    },
                    value: const_def.1,
                }),
            }
        }

        fn simplify_internal(&self) -> Result<Firkin, FirkinError> {
            self.as_unit_internal(&self.unit_collection.simplify())
        }

        fn simple_simplify_internal(&self) -> Result<Firkin, FirkinError> {
            self.as_unit_internal(&self.unit_collection.simple_simplify())
        }

        fn simplify_inplace_internal(&mut self) -> Result<(), FirkinError> {
            let new = self.simplify_internal()?;
            self.unit_collection = new.unit_collection;
            self.value = new.value;
            Ok(())
        }

        fn ss_inpl_internal(&mut self) -> Result<(), FirkinError> {
            let new = self.simple_simplify_internal()?;
            self.unit_collection = new.unit_collection;
            self.value = new.value;
            Ok(())
        }
    }

    impl Mul for Firkin {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            Firkin {
                unit_collection: self.unit_collection * rhs.unit_collection,
                value: self.value * rhs.value,
            }
        }
    }

    impl std::convert::From<f64> for Firkin {
        fn from(input: f64) -> Firkin {
            Firkin {
                unit_collection: UnitCollection::empty_collection(),
                value: input,
            }
        }
    }

    impl std::convert::From<i32> for Firkin {
        fn from(input: i32) -> Firkin {
            Firkin {
                unit_collection: UnitCollection::empty_collection(),
                value: input as f64,
            }
        }
    }

    impl fmt::Display for Firkin {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let simplified = match self.simplify_internal() {
                Ok(f) => f,
                Err(_) => return Err(fmt::Error),
            };

            write!(
                f,
                "{} {}",
                simplified.value,
                simplified.unit_collection.as_string()
            )?;

            Ok(())
        }
    }

    impl std::convert::From<Firkin> for UnitCollection {
        fn from(input: Firkin) -> UnitCollection {
            UnitCollection {
                single_units: input.unit_collection.single_units,
                base_units: input.unit_collection.base_units,
                scale: input.value * input.unit_collection.scale,
            }
        }
    }

    enum UnitCoercible {
        Firkin(Firkin),
        Float(f64),
        Int(i32),
        StrQuery(String),
        LogFirkin(LogFirkin),
    }

    impl std::convert::From<UnitCoercible> for Firkin {
        fn from(input: UnitCoercible) -> Firkin {
            match input {
                UnitCoercible::Firkin(f) => f.clone(),
                UnitCoercible::Float(f) => Firkin {
                    unit_collection: UnitCollection::empty_collection(),
                    value: f,
                },
                UnitCoercible::Int(i) => Firkin {
                    unit_collection: UnitCollection::empty_collection(),
                    value: i as f64,
                },
                UnitCoercible::StrQuery(q) => {
                    if let Some(u) = UnitCollection::from_unit_name(q.as_str()) {
                        Firkin {
                            unit_collection: u,
                            value: 1.0,
                        }
                    } else {
                        Firkin {
                            unit_collection: UnitCollection::empty_collection(),
                            value: 1.0,
                        }
                    }
                }
                UnitCoercible::LogFirkin(l) => Firkin {
                    unit_collection: UnitCollection::empty_collection(),
                    value: l.resolve(),
                },
            }
        }
    }

    impl FromPyObject<'_, '_> for UnitCoercible {
        type Error = PyErr;

        fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(f) = obj.cast::<Firkin>() {
                Ok(UnitCoercible::Firkin(f.extract::<Firkin>()?))
            } else if let Ok(f) = obj.cast::<LogFirkin>() {
                Ok(UnitCoercible::LogFirkin(f.extract::<LogFirkin>()?))
            } else if let Ok(f) = obj.cast::<PyFloat>() {
                Ok(UnitCoercible::Float(f.extract::<f64>()?))
            } else if let Ok(i) = obj.cast::<PyInt>() {
                Ok(UnitCoercible::Int(i.extract::<i32>()?))
            } else if let Ok(q) = obj.cast::<PyString>() {
                let q = q.extract::<String>()?;
                if let Some(_u) = UnitCollection::from_unit_name(q.as_str()) {
                    Ok(UnitCoercible::StrQuery(q))
                } else {
                    Err(FirkinError::UnitNotFound(q).into())
                }
            } else {
                Err(FirkinError::CannotConvertToFirkin(obj.to_string()).into())
            }
        }
    }

    #[derive(IntoPyObject)]
    enum PyNumber {
        Float(f64),
        Int(i32),
        LogFirkin(LogFirkin),
    }

    impl FromPyObject<'_, '_> for PyNumber {
        type Error = PyErr;

        fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(f) = obj.cast::<PyFloat>() {
                Ok(PyNumber::Float(f.extract::<f64>()?))
            } else if let Ok(f) = obj.cast::<PyInt>() {
                Ok(PyNumber::Int(f.extract::<i32>()?))
            } else if let Ok(f) = obj.cast::<LogFirkin>() {
                Ok(PyNumber::LogFirkin(f.extract::<LogFirkin>()?))
            } else {
                Err(FirkinError::CannotConvertToNumber(obj.to_string()).into())
            }
        }
    }

    impl std::convert::From<PyNumber> for f64 {
        fn from(input: PyNumber) -> f64 {
            match input {
                PyNumber::Float(f) => f,
                PyNumber::Int(i) => i as f64,
                PyNumber::LogFirkin(l) => l.resolve(),
            }
        }
    }

    /** A unit-attached number specifically for units of logarithmic ratios. Can
    only contain one unit at a time.

    When multiplied or divided by a number, or when added to or subtracted from
    another LogFirkin instance, remains a LogFirkin. In any other case,
    resolves the logarithmic ratio and turns into a float.

    Generally, these will be accessed through ``firkin.units``. If desired, 
    they can also be accessed through the ``LogFirkin.unit`` classmethod.

    Examples
    ^^^^^^^^
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
    10000.0 */
    #[pyclass(from_py_object)]
    #[derive(Clone)]
    struct LogFirkin {
        unit: LogUnit,
        value: f64,
    }

    #[pymethods]
    impl LogFirkin {
        /** Create a new LogFirkin instance by searching for a unit name or symbol.

        :param query: The query by which to look up the unit. Currently, only the neper
            (Np), bel (B), and decibel (dB) are supported.
        :type query: str
        :return: The new LogFirkin instance.
        :rtype: LogFirkin */
        #[classmethod]
        fn unit(_cls: &Bound<'_, PyType>, query: &str) -> PyResult<Self> {
            let unit = match search_for_log_unit_name(query) {
                Some(unit) => unit,
                None => {
                    return Err(
                        FirkinError::LogUnitNotFound(query.to_string()).into(),
                    );
                }
            };
            Ok(LogFirkin { unit, value: 1.0 })
        }

        /// Converts self into the units of other.
        fn as_unit(&self, other: LogFirkin) -> PyResult<LogFirkin> {
            Ok(self.as_unit_internal(other))
        }

        /** Returns the unit value of self, optionally converted to another unit
        using the other parameter. Note that this does not resolve the
        logarithmic ratio like as_unitless does.

        :param other: The units to coerce self into. If None, will simply
            return the internal value with no changes.
        :type other: LogFirkin or None, default None

        Examples
        ^^^^^^^^
        >>> from firkin.units import decibel as dB, bel
        >>> my_ratio = 12*dB
        >>> my_ratio
        12 [dB]
        >>> my_ratio.as_unitless()
        15.848931924611145
        >>> my_ratio.as_number()
        12.0
        >>> my_ratio.as_number(bel)
        1.2000000000000002 */
        #[pyo3(signature = (other=None))]
        fn as_number(&self, other: Option<LogFirkin>) -> PyResult<f64> {
            match other {
                Some(l) => Ok(self.as_unit_internal(l).value),
                None => Ok(self.value),
            }
        }

        /** Returns the instance as a unitless number, converting the unit into an
        appropriate logarithmic ratio.

        Examples
        ^^^^^^^^
        >>> from firkin.units import decibel as dB
        >>> ratio = 21*dB
        >>> ratio.as_unitless()
        125.89254117941687 */
        fn as_unitless(&self) -> PyResult<f64> {
            Ok(self.resolve())
        }

        /// Returns the equivalent of `as_unitless`, rounded to the given number of
        /// significant figures.
        fn round_sfig(&mut self, n_sig_figs: i32) -> PyResult<f64> {
            self.__round__(Some(
                n_sig_figs - 1 - (self.value.abs().log10().floor() as i32),
            ))
        }

        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{}", self))
        }

        fn __repr__(&self) -> PyResult<String> {
            self.__str__()
        }

        fn __mul__(&self, other: PyNumber) -> PyResult<PyNumber> {
            match other {
                PyNumber::Float(f) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: self.value * f,
                })),
                PyNumber::Int(i) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: self.value * i as f64,
                })),
                PyNumber::LogFirkin(l) => Ok(PyNumber::Float(self.resolve() * l.resolve())),
            }
        }

        fn __rmul__(&self, other: PyNumber) -> PyResult<PyNumber> {
            self.__mul__(other)
        }

        fn __div__(&self, other: PyNumber) -> PyResult<PyNumber> {
            match other {
                PyNumber::Float(f) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: self.value / f,
                })),
                PyNumber::Int(i) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: self.value / i as f64,
                })),
                PyNumber::LogFirkin(l) => Ok(PyNumber::Float(self.resolve() / l.resolve())),
            }
        }

        fn __truediv__(&self, other: PyNumber) -> PyResult<PyNumber> {
            self.__div__(other)
        }

        fn __rdiv__(&self, other: PyNumber) -> PyResult<f64> {
            match other {
                PyNumber::Float(f) => Ok(f / self.resolve()),
                PyNumber::Int(i) => Ok(i as f64 / self.resolve()),
                PyNumber::LogFirkin(l) => Ok(l.resolve() / self.resolve()),
            }
        }

        fn __rtruediv__(&self, other: PyNumber) -> PyResult<f64> {
            self.__rdiv__(other)
        }

        fn __pos__(&self) -> PyResult<LogFirkin> {
            Ok(self.clone())
        }

        fn __neg__(&self) -> PyResult<LogFirkin> {
            Ok(LogFirkin {
                unit: self.unit.clone(),
                value: -self.value,
            })
        }

        fn __add__(&self, other: PyNumber) -> PyResult<PyNumber> {
            match other {
                PyNumber::Float(f) => Ok(PyNumber::Float(f + self.resolve())),
                PyNumber::Int(i) => Ok(PyNumber::Float(i as f64 + self.resolve())),
                PyNumber::LogFirkin(l) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: self.value + l.resolve().log(self.unit.scale),
                })),
            }
        }

        fn __radd__(&self, other: PyNumber) -> PyResult<PyNumber> {
            self.__add__(other)
        }

        fn __sub__(&self, other: PyNumber) -> PyResult<PyNumber> {
            match other {
                PyNumber::Float(f) => Ok(PyNumber::Float(self.resolve() - f)),
                PyNumber::Int(i) => Ok(PyNumber::Float(self.resolve() - i as f64)),
                PyNumber::LogFirkin(l) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: self.value - l.resolve().log(self.unit.scale),
                })),
            }
        }

        fn __rsub__(&self, other: PyNumber) -> PyResult<PyNumber> {
            match other {
                PyNumber::Float(f) => Ok(PyNumber::Float(f - self.resolve())),
                PyNumber::Int(i) => Ok(PyNumber::Float(i as f64 - self.resolve())),
                PyNumber::LogFirkin(l) => Ok(PyNumber::LogFirkin(LogFirkin {
                    unit: self.unit.clone(),
                    value: l.resolve().log(self.unit.scale) - self.value,
                })),
            }
        }

        fn __pow__(&self, exponent: PyNumber, _modulus: Option<PyNumber>) -> PyResult<f64> {
            let exponent: f64 = exponent.into();
            Ok(self.resolve().powf(exponent))
        }

        fn __rpow__(&self, other: PyNumber, _modulus: Option<PyNumber>) -> PyResult<f64> {
            let other: f64 = other.into();
            Ok(other.powf(self.resolve()))
        }

        fn __lt__(&self, other: PyNumber) -> PyResult<bool> {
            let other: f64 = other.into();
            Ok(self.resolve() < other)
        }

        fn __le__(&self, other: PyNumber) -> PyResult<bool> {
            let other: f64 = other.into();
            Ok(self.resolve() <= other)
        }

        fn __gt__(&self, other: PyNumber) -> PyResult<bool> {
            let other: f64 = other.into();
            Ok(self.resolve() > other)
        }

        fn __ge__(&self, other: PyNumber) -> PyResult<bool> {
            let other: f64 = other.into();
            Ok(self.resolve() >= other)
        }

        fn __eq__(&self, other: PyNumber) -> PyResult<bool> {
            let other: f64 = other.into();
            Ok(self.resolve() == other)
        }

        fn __ne__(&self, other: PyNumber) -> PyResult<bool> {
            let other: f64 = other.into();
            Ok(self.resolve() != other)
        }

        fn __abs__(&self) -> PyResult<f64> {
            Ok(self.resolve().abs())
        }

        fn __int__(&self) -> PyResult<i32> {
            Ok(self.resolve() as i32)
        }

        fn __float__(&self) -> PyResult<f64> {
            Ok(self.resolve())
        }

        #[pyo3(signature=(ndigits=None))]
        fn __round__(&self, ndigits: Option<i32>) -> PyResult<f64> {
            match ndigits {
                Some(n) => {
                    let mul = 10.0f64.powi(n);
                    Ok((self.resolve() * mul).round() / mul)
                }
                None => Ok(self.resolve().round()),
            }
        }

        fn __exp__(&self) -> PyResult<f64> {
            Ok(self.resolve().exp())
        }

        fn exp(&self) -> PyResult<f64> {
            self.__exp__()
        }

        fn __log__(&self) -> PyResult<f64> {
            Ok(self.resolve().ln())
        }

        fn log(&self) -> PyResult<f64> {
            self.__log__()
        }

        fn __log10__(&self) -> PyResult<f64> {
            Ok(self.resolve().log10())
        }

        fn log10(&self) -> PyResult<f64> {
            self.__log10__()
        }
    }

    impl LogFirkin {
        fn resolve(&self) -> f64 {
            self.unit.scale.powf(self.value)
        }

        fn as_unit_internal(&self, other: LogFirkin) -> LogFirkin {
            LogFirkin {
                value: self.resolve().log(other.unit.scale),
                unit: other.unit,
            }
        }
    }

    impl fmt::Display for LogFirkin {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} [{}]", self.value, self.unit.abbr)?;

            Ok(())
        }
    }
}
