use pyo3::prelude::*;

#[pymodule]
fn snapr(_: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
