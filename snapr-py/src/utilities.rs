/// Utility function to convert an `err` to a [`pyo3::PyErr`].
pub(crate) fn to_py_error<E: Into<anyhow::Error>>(err: E) -> pyo3::PyErr {
    let converted = format!("{}", err.into());
    crate::SnaprError::new_err(converted)
}

/// Utility function to convert an `err` to a [`snapr::Error`].
pub(crate) fn to_snapr_error<E: Into<anyhow::Error>>(err: E) -> snapr::Error {
    snapr::Error::Unknown(err.into())
}
