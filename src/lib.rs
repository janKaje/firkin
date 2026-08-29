// #![allow(unused)]

use pyo3::prelude::*;

mod error;
mod unit;
mod constant;

/// A Python module implemented in Rust.
#[pymodule]
mod firkin {
    use pyo3::{prelude::*, types::*};
    use std::{fmt, ops::Mul};

    use crate::error::FirkinError;
    use crate::unit::UnitCollection;
    use crate::constant::search_for_constant_name;

    /// Unit! yippee
    #[pyclass(from_py_object)]
    #[derive(Clone)]
    struct Firkin {
        unit_collection: UnitCollection,
        value: f64,
    }

    #[pymethods]
    impl Firkin {
        #[classmethod]
        fn unit(_cls: &Bound<'_, PyType>, unit_name_or_symbol: &str) -> PyResult<Self> {
            let unit = match UnitCollection::from_unit_name(unit_name_or_symbol) {
                Some(unit) => unit,
                None => {
                    return Err(FirkinError::UnitNotFound(unit_name_or_symbol.to_string()).into());
                }
            };

            Ok(Firkin {
                unit_collection: unit,
                value: 1.0,
            })
        }

        #[classmethod]
        fn constant(_cls: &Bound<'_, PyType>, constant_name: &str) -> PyResult<Self> {
            match Firkin::constant_query_internal(constant_name) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into())
            }
        }

        #[classmethod]
        fn empty(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
            Ok(Firkin::empty_unit())
        }

        #[classmethod]
        fn custom(_cls: &Bound<'_, PyType>, name: String, abbr: String, definition: UnitCoercible) -> PyResult<Self> {
            let definition: Firkin = definition.into();
            let definition: UnitCollection = definition.into();
            let unit = definition.to_single_unit(name, abbr);
            Ok(Firkin {
                unit_collection: UnitCollection::coerce_unit_to_collection(unit),
                value: 1.0
            })
        }

        fn as_unit(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            match self.as_unit_internal(&other.unit_collection) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        #[pyo3(signature = (other=None, scale=false))]
        fn as_number(&self, other: Option<UnitCoercible>, scale: bool) -> PyResult<f64> {
            match other {
                Some(other) => {
                    let other: Firkin = other.into();
                    match self.as_number_internal(
                        &other.unit_collection,
                        if scale {other.value} else {1.0}
                    ) {
                        Ok(r) => Ok(r),
                        Err(e) => Err(e.into()),
                    }
                }
                None => Ok(self.value),
            }
        }

        fn as_unitless(&self) -> PyResult<f64> {
            match self.as_number_internal(&UnitCollection::empty_collection(), 1.0) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        fn as_base_units(&self) -> PyResult<Firkin> {
            match self.as_unit_internal(&self.unit_collection.get_base_units()) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        fn round_sfig(&mut self, n_sig_figs: i32) -> PyResult<Firkin> {
            self.__round__(Some(n_sig_figs-1-(self.value.abs().log10().floor() as i32)))
        }

        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{}", self))
        }

        fn __repr__(&self) -> PyResult<String> {
            self.__str__()
        }

        fn __mul__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            Ok(Firkin {
                unit_collection: self.unit_collection.clone() * other.unit_collection.clone(),
                value: self.value * other.value,
            })
        }

        fn __rmul__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__mul__(other)
        }

        fn __div__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            Ok(Firkin {
                unit_collection: self.unit_collection.clone() / other.unit_collection.clone(),
                value: self.value / other.value,
            })
        }

        fn __truediv__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__div__(other)
        }

        fn __rdiv__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            Ok(Firkin {
                unit_collection: other.unit_collection.clone() / self.unit_collection.clone(),
                value: other.value / self.value,
            })
        }

        fn __rtruediv__(&self, other: UnitCoercible) -> PyResult<Firkin> {
            self.__rdiv__(other)
        }

        fn __pos__(&self) -> PyResult<Firkin> {
            Ok(self.clone())
        }

        fn __neg__(&self) -> PyResult<Firkin> {
            Ok(Firkin {
                unit_collection: self.unit_collection.clone(),
                value: -self.value,
            })
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
                })
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
                })
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
                })
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
            })
        }

        fn __rpow__(&self, other: UnitCoercible, _modulus: Option<PyNumber>) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            let exponent = self.as_number_internal(&UnitCollection::empty_collection(), 1.0)?;
            Ok(Firkin {
                unit_collection: other.unit_collection.pow(exponent),
                value: other.value.powf(exponent),
            })
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

        #[pyo3(signature=(ndigits=None))]
        fn __round__(&self, ndigits: Option<i32>) -> PyResult<Firkin> {
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

        fn descriptive(&self) -> PyResult<String> {
            Ok(format!(
                "{} {}",
                self.value,
                self.unit_collection.as_descriptive_string()
            ))
        }
    }

    impl Firkin {
        fn as_unit_internal(&self, other: &UnitCollection) -> Result<Firkin, FirkinError> {
            if let Some(scale_diff) = self
                .unit_collection
                .equivalent_scale_diff(&other)
            {
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

        fn as_number_internal(&self, other: &UnitCollection, scale: f64) -> Result<f64, FirkinError> {
            match self.as_unit_internal(other) {
                Ok(u) => Ok(u.value/scale),
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
                        None => return Err(FirkinError::UnitNotFound(const_def.2.to_string()))
                    },
                    value: const_def.1
                })
            }
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
            write!(f, "{} {}", self.value, self.unit_collection.as_string())?;

            Ok(())
        }
    }

    impl std::convert::From<Firkin> for UnitCollection {
        fn from(input: Firkin) -> UnitCollection {
            UnitCollection { single_units: input.unit_collection.single_units, base_units: input.unit_collection.base_units, scale: input.value * input.unit_collection.scale }
        }
    }

    enum UnitCoercible {
        Firkin(Firkin),
        Float(f64),
        Int(i32),
        StrQuery(String),
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
            }
        }
    }

    impl FromPyObject<'_, '_> for UnitCoercible {
        type Error = PyErr;

        fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(f) = obj.cast::<Firkin>() {
                Ok(UnitCoercible::Firkin(f.extract::<Firkin>()?))
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

    enum PyNumber {
        Float(f64),
        Int(i32),
    }

    impl FromPyObject<'_, '_> for PyNumber {
        type Error = PyErr;

        fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(f) = obj.cast::<PyFloat>() {
                Ok(PyNumber::Float(f.extract::<f64>()?))
            } else if let Ok(f) = obj.cast::<PyInt>() {
                Ok(PyNumber::Int(f.extract::<i32>()?))
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
            }
        }
    }
}
