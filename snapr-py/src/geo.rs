use std::ops::{Deref, DerefMut};

use pyo3::prelude::*;
use wkt::TryFromWkt;

use crate::SnaprError;

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Geometry")]
pub enum PyGeometry {
    Point(PyPoint),
    Line(PyLine),
    LineString(PyLineString),
    Polygon(PyPolygon),
    MultiPoint(PyMultiPoint),
    MultiLineString(PyMultiLineString),
    MultiPolygon(PyMultiPolygon),
    GeometryCollection(PyGeometryCollection),
    Rect(PyRect),
    Triangle(PyTriangle),
}

#[pymethods]
impl PyGeometry {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[allow(clippy::from_over_into)]
impl Into<geo::Geometry> for PyGeometry {
    fn into(self) -> geo::Geometry {
        match self {
            Self::Point(geometry) => geo::Geometry::Point(geometry.0),
            Self::Line(geometry) => geo::Geometry::Line(geometry.0),
            Self::LineString(geometry) => geo::Geometry::LineString(geometry.0),
            Self::Polygon(geometry) => geo::Geometry::Polygon(geometry.0),
            Self::MultiPoint(geometry) => geo::Geometry::MultiPoint(geometry.0),
            Self::MultiLineString(geometry) => geo::Geometry::MultiLineString(geometry.0),
            Self::MultiPolygon(geometry) => geo::Geometry::MultiPolygon(geometry.0),
            Self::GeometryCollection(geometry) => geo::Geometry::GeometryCollection(geometry.0),
            Self::Rect(geometry) => geo::Geometry::Rect(geometry.0),
            Self::Triangle(geometry) => geo::Geometry::Triangle(geometry.0),
        }
    }
}

#[derive(Clone, Debug, FromPyObject, PartialEq)]
pub enum PyPointOrTuple {
    Point(PyPoint),
    Tuple((f64, f64)),
}

#[allow(clippy::from_over_into)]
impl Into<PyPoint> for PyPointOrTuple {
    fn into(self) -> PyPoint {
        match self {
            Self::Point(point) => point,
            Self::Tuple((x, y)) => PyPoint::new(x, y),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<geo::Point<f64>> for PyPointOrTuple {
    fn into(self) -> geo::Point<f64> {
        let py_point = <Self as Into<PyPoint>>::into(self);
        py_point.0
    }
}

macro_rules! impl_geo_wrapper {
    ($base: ident, $variant: ident, $class: literal, $new: item) => {
        #[derive(Clone, Debug, PartialEq)]
        #[pyclass(name = $class)]
        pub struct $variant(geo::$base<f64>);

        #[pymethods]
        impl $variant {
            #[new] $new

            fn __repr__(&self) -> String {
                format!("{self:?}")
            }
        }

        impl Deref for $variant {
            type Target = geo::$base<f64>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $variant {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<geo::$base<f64>> for $variant {
            fn from(value: geo::$base<f64>) -> Self {
                Self(value)
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<geo::$base<f64>> for $variant {
            fn into(self) -> geo::$base<f64> {
                self.0
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<PyGeometry> for $variant {
            fn into(self) -> PyGeometry {
                PyGeometry::$base(self)
            }
        }
    };
}

impl_geo_wrapper!(
    Point,
    PyPoint,
    "Point",
    fn new(latitude: f64, longitude: f64) -> Self {
        let point = geo::point!(x: latitude, y: longitude);
        Self(point)
    }
);

impl_geo_wrapper!(
    Line,
    PyLine,
    "Line",
    fn new(start: PyPointOrTuple, end: PyPointOrTuple) -> Self {
        Self(geo::Line::new::<geo::Point>(start.into(), end.into()))
    }
);

impl_geo_wrapper!(
    LineString,
    PyLineString,
    "LineString",
    fn new(points: Vec<PyPointOrTuple>) -> Self {
        let coords = points
            .into_iter()
            .map(|x| <PyPointOrTuple as Into<geo::Point>>::into(x).into())
            .collect();

        Self(geo::LineString::new(coords))
    }
);

impl_geo_wrapper!(
    Polygon,
    PyPolygon,
    "Polygon",
    fn new(exterior: PyLineString, interiors: Vec<PyLineString>) -> Self {
        let interiors = interiors.into_iter().map(PyLineString::into).collect();
        Self(geo::Polygon::new(exterior.0, interiors))
    }
);

impl_geo_wrapper!(
    MultiPoint,
    PyMultiPoint,
    "MultiPoint",
    fn new(points: Vec<PyPointOrTuple>) -> Self {
        let points = points.into_iter().map(PyPointOrTuple::into).collect();
        Self(geo::MultiPoint::new(points))
    }
);

impl_geo_wrapper!(
    MultiLineString,
    PyMultiLineString,
    "MultiLineString",
    fn new(line_strings: Vec<PyLineString>) -> Self {
        let line_strings = line_strings.into_iter().map(PyLineString::into).collect();
        Self(geo::MultiLineString::new(line_strings))
    }
);

impl_geo_wrapper!(
    MultiPolygon,
    PyMultiPolygon,
    "MultiPolygon",
    fn new(polygons: Vec<PyPolygon>) -> Self {
        let polygons = polygons.into_iter().map(PyPolygon::into).collect();
        Self(geo::MultiPolygon::new(polygons))
    }
);

impl_geo_wrapper!(
    GeometryCollection,
    PyGeometryCollection,
    "GeometryCollection",
    fn new(geometries: Vec<PyGeometry>) -> Self {
        Self(geo::GeometryCollection::from(geometries))
    }
);

impl_geo_wrapper!(
    Rect,
    PyRect,
    "Rect",
    fn new(corner_1: PyPointOrTuple, corner_2: PyPointOrTuple) -> Self {
        Self(geo::Rect::new::<geo::Point>(
            corner_1.into(),
            corner_2.into(),
        ))
    }
);

impl_geo_wrapper!(
    Triangle,
    PyTriangle,
    "Triangle",
    fn new(a: PyPointOrTuple, b: PyPointOrTuple, c: PyPointOrTuple) -> Self {
        let (a, b, c): (geo::Point, geo::Point, geo::Point) = (a.into(), b.into(), c.into());

        Self(geo::Triangle::new(
            geo::coord! {x: a.x(), y: a.y()},
            geo::coord! {x: b.x(), y: b.y()},
            geo::coord! {x: c.x(), y: c.y()},
        ))
    }
);

#[pyfunction]
pub fn well_known_text_to_geometry(well_known_text: String) -> PyResult<PyGeometry> {
    let geometry = geo::Geometry::<f64>::try_from_wkt_str(&well_known_text)
        .map_err(|err| SnaprError::new_err(err.to_string()))?;

    match geometry {
        geo::Geometry::Point(geometry) => Ok(PyPoint(geometry).into()),
        geo::Geometry::Line(geometry) => Ok(PyLine(geometry).into()),
        geo::Geometry::LineString(geometry) => Ok(PyLineString(geometry).into()),
        geo::Geometry::Polygon(geometry) => Ok(PyPolygon(geometry).into()),
        geo::Geometry::MultiPoint(geometry) => Ok(PyMultiPoint(geometry).into()),
        geo::Geometry::MultiLineString(geometry) => Ok(PyMultiLineString(geometry).into()),
        geo::Geometry::MultiPolygon(geometry) => Ok(PyMultiPolygon(geometry).into()),
        geo::Geometry::GeometryCollection(geometry) => Ok(PyGeometryCollection(geometry).into()),
        geo::Geometry::Rect(geometry) => Ok(PyRect(geometry).into()),
        geo::Geometry::Triangle(geometry) => Ok(PyTriangle(geometry).into()),
    }
}

#[pyfunction]
pub fn well_known_texts_to_geometries(well_known_texts: Vec<String>) -> PyResult<Vec<PyGeometry>> {
    let well_known_texts = well_known_texts
        .into_iter()
        .map(well_known_text_to_geometry)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(well_known_texts)
}
