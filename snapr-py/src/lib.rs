use std::io::Cursor;

use ::snapr::{SnaprBuilder, TileFetcher};
use image::{DynamicImage, ImageFormat, ImageReader};
use pyo3::{create_exception, exceptions::PyException, prelude::*, types::PyByteArray};
use utilities::{to_py_error, to_snapr_error};

mod geo;
mod style;
mod utilities;

#[derive(Debug)]
#[pyclass]
struct Snapr {
    tile_fetcher: Py<PyAny>,
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
        tile_fetcher: Py<PyAny>,
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

    #[pyo3(signature = (geometry, styles = Vec::new()))]
    fn generate_snapshot_from_geometry<'py>(
        &self,
        py: Python<'py>,
        geometry: geo::PyGeometry,
        styles: Vec<style::PyStyle>,
    ) -> PyResult<Bound<'py, PyByteArray>> {
        self.generate_snapshot_from_geometries(py, vec![geometry], styles)
    }

    #[pyo3(signature = (geometries, styles = Vec::new()))]
    fn generate_snapshot_from_geometries<'py>(
        &self,
        py: Python<'py>,
        geometries: Vec<geo::PyGeometry>,
        styles: Vec<style::PyStyle>,
    ) -> PyResult<Bound<'py, PyByteArray>> {
        let tile_fetcher = |coords: &'_ [(i32, i32)],
                            zoom: u8|
         -> Result<Vec<(i32, i32, DynamicImage)>, ::snapr::Error> {
            let mut tiles = Vec::new();

            let coords_and_tiles: Vec<(i32, i32, Py<PyByteArray>)> = self
                .tile_fetcher
                .call1(py, (coords.to_vec(), zoom))
                .and_then(|any| any.extract(py))
                .map_err(to_snapr_error)?;

            for (x, y, tile) in coords_and_tiles {
                let cursor = tile
                    .extract::<Vec<u8>>(py)
                    .map(Cursor::new)
                    .map_err(to_snapr_error)?;

                let image = ImageReader::new(cursor)
                    .with_guessed_format()
                    .map_err(to_snapr_error)?
                    .decode()
                    .map_err(to_snapr_error)?;

                tiles.push((x, y, image));
            }

            Ok(tiles)
        };

        let builder = SnaprBuilder::new()
            .with_tile_fetcher(TileFetcher::Batch(&tile_fetcher))
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
            .into_iter()
            .map(<geo::PyGeometry as Into<::geo::Geometry>>::into)
            .collect();

        let styles = styles
            .into_iter()
            .map(<style::PyStyle as Into<::snapr::drawing::style::Style>>::into)
            .collect::<Vec<_>>();

        let snapshot = snapr
            .generate_snapshot_from_geometries(geometries, &styles)
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
    module.add_class::<Snapr>()?;

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

    module.add_function(wrap_pyfunction!(geo::well_known_text_to_geometry, module)?)?;

    module.add_function(wrap_pyfunction!(
        geo::well_known_texts_to_geometries,
        module
    )?)?;

    module.add_class::<style::PyColor>()?;
    module.add_class::<style::PyColorOptions>()?;
    module.add_class::<style::PyLabel>()?;
    module.add_class::<style::PyLineStyle>()?;
    module.add_class::<style::PyPointStyle>()?;
    module.add_class::<style::PyPolygonStyle>()?;
    module.add_class::<style::PyRepresentation>()?;
    module.add_class::<style::PyShape>()?;
    module.add_class::<style::PyStyle>()?;
    module.add_class::<style::PySvg>()?;

    Ok(())
}
