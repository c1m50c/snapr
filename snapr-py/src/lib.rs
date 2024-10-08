use std::io::Cursor;

use ::snapr::SnaprBuilder;
use image::{DynamicImage, ImageReader};
use pyo3::{
    create_exception,
    exceptions::PyException,
    prelude::*,
    types::{PyByteArray, PyDict, PyFunction, PyList},
};
use utilities::{to_py_error, to_snapr_error};

mod utilities;

/// Python-variant of the [`snapr::TileFetcher`](::snapr::TileFetcher) type.
type PyTileFetcher = fn(i32, i32, u8) -> PyResult<Py<PyByteArray>>;

#[derive(Debug)]
#[pyclass]
struct Snapr {
    tile_fetcher: Py<PyFunction>,
    tile_size: u32,
    height: u32,
    width: u32,
    zoom: Option<u8>,
}

#[pymethods]
impl Snapr {
    #[new]
    #[pyo3(signature = (tile_fetcher, tile_size=256, height=600, width=800, zoom=None))]
    fn new(
        tile_fetcher: Py<PyFunction>,
        tile_size: u32,
        height: u32,
        width: u32,
        zoom: Option<u8>,
    ) -> Self {
        Self {
            tile_fetcher,
            tile_size,
            height,
            width,
            zoom,
        }
    }

    fn generate_snapshot_from_geometry(&self, geometry: Py<PyDict>) -> PyResult<Py<PyByteArray>> {
        todo!("Convert `geometry` to a `Py<PyList>` and pass it to `self.generate_snapshot_from_geometries`")
    }

    fn generate_snapshot_from_geometries(
        &self,
        geometries: Py<PyList>,
    ) -> PyResult<Py<PyByteArray>> {
        let tile_fetcher = |x, y, zoom| -> Result<DynamicImage, ::snapr::Error> {
            let image_bytes = Python::with_gil(|py| -> PyResult<Vec<u8>> {
                let bytes: Py<PyByteArray> = self
                    .tile_fetcher
                    .call1(py, (x, y, zoom))
                    .and_then(|any| any.extract(py))?;

                bytes.extract(py)
            });

            let cursor = match image_bytes {
                Ok(bytes) => Cursor::new(bytes),

                Err(err) => {
                    return Err(to_snapr_error(err));
                }
            };

            let image = ImageReader::new(cursor)
                .with_guessed_format()
                .map_err(to_snapr_error)?
                .decode()
                .map_err(to_snapr_error)?;

            Ok(image)
        };

        let builder = SnaprBuilder::new()
            .with_tile_fetcher(&tile_fetcher)
            .with_tile_size(self.tile_size)
            .with_height(self.height)
            .with_width(self.width);

        let snapr = match self.zoom {
            Some(zoom) => {
                let builder = builder.with_zoom(zoom);
                builder.build().map_err(to_py_error)
            }

            None => builder.build().map_err(to_py_error),
        }?;

        todo!("Iterate over over `geometries` and convert them to a `Vec<StyledGeometry>`")
    }
}

create_exception!(snapr, SnaprError, PyException);

#[pymodule]
fn snapr(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("SnaprError", py.get_type_bound::<SnaprError>())?;
    module.add_class::<Snapr>()?;

    Ok(())
}
