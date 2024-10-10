use std::io::Cursor;

use ::snapr::SnaprBuilder;
use image::{DynamicImage, ImageFormat, ImageReader};
use pyo3::{
    create_exception,
    exceptions::PyException,
    prelude::*,
    types::{PyByteArray, PyFunction, PyList},
};
use utilities::{to_py_error, to_snapr_error};

mod geo;
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

    fn generate_snapshot_from_geometry<'py>(
        &self,
        py: Python<'py>,
        geometry: &Bound<'_, geo::PyGeometry>,
    ) -> PyResult<Bound<'py, PyByteArray>> {
        let geometries = PyList::new_bound(py, [geometry]);
        self.generate_snapshot_from_geometries(py, &geometries)
    }

    fn generate_snapshot_from_geometries<'py>(
        &self,
        py: Python<'py>,
        geometries: &Bound<'_, PyList>,
    ) -> PyResult<Bound<'py, PyByteArray>> {
        let tile_fetcher = |x, y, zoom| -> Result<DynamicImage, ::snapr::Error> {
            let image_bytes = Python::with_gil(|py| -> PyResult<Vec<u8>> {
                let bytes: Py<PyByteArray> = self
                    .tile_fetcher
                    .call1(py, (x, y, zoom))
                    .and_then(|any| any.extract(py))?;

                bytes.extract(py)
            });

            let cursor = image_bytes.map(Cursor::new).map_err(to_snapr_error)?;

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

        let geometries = geometries
            .iter()
            .flat_map(|any| any.extract::<geo::PyGeometry>())
            .map(|geometry| <geo::PyGeometry as Into<::geo::Geometry>>::into(geometry))
            .collect();

        let snapshot = snapr
            .generate_snapshot_from_geometries(geometries, &[])
            .map_err(to_py_error)?;

        // Estimated size of an 800x600 PNG snapshot is `1.44MB`
        let mut bytes = Vec::with_capacity(1_440_000);

        snapshot
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .map_err(to_py_error)?;

        Ok(PyByteArray::new_bound(py, &bytes))
    }
}

create_exception!(snapr, SnaprError, PyException);

#[pymodule]
fn snapr(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("SnaprError", py.get_type_bound::<SnaprError>())?;

    module.add_class::<geo::PyGeometry>()?;
    module.add_class::<geo::PyGeometryCollection>()?;
    module.add_class::<geo::PyLine>()?;
    module.add_class::<geo::PyLineString>()?;
    module.add_class::<geo::PyMultiLineString>()?;
    module.add_class::<geo::PyMultiPoint>()?;
    module.add_class::<geo::PyMultiPolygon>()?;
    module.add_class::<geo::PyPoint>()?;
    module.add_class::<geo::PyPolygon>()?;
    module.add_class::<geo::PyRect>()?;
    module.add_class::<geo::PyTriangle>()?;
    module.add_class::<Snapr>()?;

    Ok(())
}
