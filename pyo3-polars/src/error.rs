use std::fmt::{Debug, Formatter};

use polars::prelude::PolarsError;
use polars_core::error::ArrowError;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyIOError, PyIndexError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error)]
pub enum PyPolarsErr {
    #[error(transparent)]
    Polars(#[from] PolarsError),
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    Arrow(#[from] ArrowError),
}

impl std::convert::From<PyPolarsErr> for PyErr {
    fn from(err: PyPolarsErr) -> PyErr {
        let default = || PyRuntimeError::new_err(format!("{:?}", &err));

        use PyPolarsErr::*;
        match &err {
            Polars(err) => match err {
                PolarsError::ComputeError(err) => ComputeError::new_err(err.to_string()),
                PolarsError::NoData(err) => NoDataError::new_err(err.to_string()),
                PolarsError::ShapeMismatch(err) => ShapeError::new_err(err.to_string()),
                PolarsError::SchemaMismatch(err) => SchemaError::new_err(err.to_string()),
                PolarsError::Io(err) => PyIOError::new_err(err.to_string()),
                PolarsError::OutOfBounds(err) => PyIndexError::new_err(err.to_string()),
                PolarsError::InvalidOperation(err) => PyValueError::new_err(err.to_string()),
                PolarsError::ArrowError(err) => ArrowErrorException::new_err(format!("{:?}", err)),
                PolarsError::Duplicate(err) => DuplicateError::new_err(err.to_string()),
                PolarsError::ColumnNotFound(err) => ColumnNotFound::new_err(err.to_string()),
                PolarsError::SchemaFieldNotFound(err) => {
                    SchemaFieldNotFound::new_err(err.to_string())
                }
                PolarsError::StructFieldNotFound(err) => {
                    StructFieldNotFound::new_err(err.to_string())
                }
                PolarsError::StringCacheMismatch(err) => {
                    StringCacheMismatchError::new_err(err.to_string())
                }
            },
            Arrow(err) => ArrowErrorException::new_err(format!("{:?}", err)),
            _ => default(),
        }
    }
}

impl Debug for PyPolarsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PyPolarsErr::*;
        match self {
            Polars(err) => write!(f, "{:?}", err),
            Other(err) => write!(f, "BindingsError: {:?}", err),
            Arrow(err) => write!(f, "{:?}", err),
        }
    }
}

create_exception!(exceptions, ColumnNotFound, PyException);
create_exception!(exceptions, SchemaFieldNotFound, PyException);
create_exception!(exceptions, StructFieldNotFound, PyException);
create_exception!(exceptions, ComputeError, PyException);
create_exception!(exceptions, NoDataError, PyException);
create_exception!(exceptions, ArrowErrorException, PyException);
create_exception!(exceptions, ShapeError, PyException);
create_exception!(exceptions, SchemaError, PyException);
create_exception!(exceptions, DuplicateError, PyException);
create_exception!(exceptions, StringCacheMismatchError, PyException);
