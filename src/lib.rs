// #![allow(unused)]

use pyo3::prelude::*;

mod error;
mod unit;

/// A Python module implemented in Rust.
#[pymodule]
mod firkin {
    use pyo3::{prelude::*, types::*};
    use std::{fmt, ops::Mul};

    use crate::error::FirkinError;
    use crate::unit::UnitCollection;

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

        fn as_unit(&self, other: UnitCoercible) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            match self.as_unit_internal(&other) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
        }

        #[pyo3(signature = (other=None))]
        fn as_number(&self, other: Option<UnitCoercible>) -> PyResult<f64> {
            match other {
                Some(other) => {
                    let other: Firkin = other.into();
                    match self.as_number_internal(&other) {
                        Ok(r) => Ok(r),
                        Err(e) => Err(e.into()),
                    }
                }
                None => Ok(self.value),
            }
        }

        fn as_unitless(&self) -> PyResult<f64> {
            match self.as_number_internal(&Firkin::empty_unit()) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            }
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
            let exponent = exponent.as_number_internal(&Firkin::empty_unit())?;
            Ok(Firkin {
                unit_collection: self.unit_collection.pow(exponent),
                value: self.value.powf(exponent),
            })
        }

        fn __rpow__(&self, other: UnitCoercible, _modulus: Option<PyNumber>) -> PyResult<Firkin> {
            let other: Firkin = other.into();
            let exponent = self.as_number_internal(&Firkin::empty_unit())?;
            Ok(Firkin {
                unit_collection: other.unit_collection.pow(exponent),
                value: other.value.powf(exponent),
            })
        }

        fn __lt__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value < other.as_unit_internal(&self)?.value)
        }

        fn __le__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value <= other.as_unit_internal(&self)?.value)
        }

        fn __gt__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value > other.as_unit_internal(&self)?.value)
        }

        fn __ge__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value >= other.as_unit_internal(&self)?.value)
        }

        fn __eq__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value == other.as_unit_internal(&self)?.value)
        }

        fn __ne__(&self, other: UnitCoercible) -> PyResult<bool> {
            let other: Firkin = other.into();
            Ok(self.value != other.as_unit_internal(&self)?.value)
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
            let exponent = self.as_number_internal(&Firkin::empty_unit())?;
            Ok(exponent.exp())
        }

        fn exp(&self) -> PyResult<f64> {
            self.__exp__()
        }

        fn __log__(&self) -> PyResult<f64> {
            let exponent = self.as_number_internal(&Firkin::empty_unit())?;
            Ok(exponent.ln())
        }

        fn log(&self) -> PyResult<f64> {
            self.__log__()
        }

        fn __log10__(&self) -> PyResult<f64> {
            let exponent = self.as_number_internal(&Firkin::empty_unit())?;
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
        fn as_unit_internal(&self, other: &Firkin) -> Result<Firkin, FirkinError> {
            if let Some(scale_diff) = self
                .unit_collection
                .equivalent_scale_diff(&other.unit_collection)
            {
                if let Some(self_offset) = self.unit_collection.offset()
                    && let Some(other_offset) = other.unit_collection.offset()
                {
                    Ok(Firkin {
                        unit_collection: other.unit_collection.clone(),
                        value: {
                            (self.value * self.unit_collection.scale - self_offset + other_offset)
                                / other.unit_collection.scale
                        },
                    })
                } else {
                    Ok(Firkin {
                        unit_collection: other.unit_collection.clone(),
                        value: self.value / scale_diff,
                    })
                }
            } else {
                Err(FirkinError::IncompatibleUnits {
                    first: self.unit_collection.as_string(),
                    second: other.unit_collection.as_string(),
                })
            }
        }

        fn as_number_internal(&self, other: &Firkin) -> Result<f64, FirkinError> {
            match self.as_unit_internal(other) {
                Ok(u) => Ok(u.value),
                Err(e) => Err(e),
            }
        }

        fn empty_unit() -> Firkin {
            Firkin {
                unit_collection: UnitCollection::empty_collection(),
                value: 1.0,
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
