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

mod types;
mod utilities;

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

    fn generate_snapshot_from_geometry(
        &self,
        py: Python,
        geometry: &Bound<'_, PyDict>,
    ) -> PyResult<Py<PyByteArray>> {
        let geometries = PyList::new_bound(py, [geometry]);
        self.generate_snapshot_from_geometries(&geometries)
    }

    fn generate_snapshot_from_geometries(
        &self,
        geometries: &Bound<'_, PyList>,
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

    module.add_class::<types::PyGeometry>()?;
    module.add_class::<types::PyGeometryCollection>()?;
    module.add_class::<types::PyLine>()?;
    module.add_class::<types::PyLineString>()?;
    module.add_class::<types::PyMultiLineString>()?;
    module.add_class::<types::PyMultiPoint>()?;
    module.add_class::<types::PyMultiPolygon>()?;
    module.add_class::<types::PyPoint>()?;
    module.add_class::<types::PyPolygon>()?;
    module.add_class::<types::PyRect>()?;
    module.add_class::<types::PyTriangle>()?;
    module.add_class::<Snapr>()?;

    Ok(())
}
