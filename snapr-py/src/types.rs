use std::ops::{Deref, DerefMut};

use pyo3::{prelude::*, types::PyList};

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

macro_rules! impl_geo_wrapper {
    ($base: ident, $variant: ident, $class: literal) => {
        #[derive(Clone, Debug, PartialEq)]
        #[pyclass(name = $class)]
        pub struct $variant(geo::$base<f64>);

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

        impl Into<PyGeometry> for $variant {
            fn into(self) -> PyGeometry {
                PyGeometry::$base(self)
            }
        }
    };
}

impl_geo_wrapper!(Point, PyPoint, "Point");

#[pymethods]
impl PyPoint {
    #[new]
    fn new(latitude: f64, longitude: f64) -> Self {
        let point = geo::point!(x: latitude, y: longitude);
        Self(point)
    }
}

impl_geo_wrapper!(Line, PyLine, "Line");

#[pymethods]
impl PyLine {
    #[new]
    fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        let start = geo::coord! { x: start.0, y: start.1 };
        let end = geo::coord! { x: end.0, y: end.1 };

        Self(geo::Line::new(start, end))
    }
}

impl_geo_wrapper!(LineString, PyLineString, "LineString");

#[pymethods]
impl PyLineString {
    #[new]
    fn new(points: Bound<'_, PyList>) -> Self {
        let points = points
            .iter()
            .flat_map(|any| any.extract::<(f64, f64)>())
            .map(|(x, y)| geo::coord!(x: x, y: y))
            .collect();

        Self(geo::LineString::new(points))
    }
}

impl_geo_wrapper!(Polygon, PyPolygon, "Polygon");

#[pymethods]
impl PyPolygon {
    #[new]
    fn new(exterior: PyRef<PyLineString>, interiors: Bound<'_, PyList>) -> Self {
        let interiors = interiors
            .iter()
            .flat_map(|any| any.extract::<PyLineString>())
            .map(|line_string| line_string.0)
            .collect();

        Self(geo::Polygon::new(exterior.clone().0, interiors))
    }
}

impl_geo_wrapper!(MultiPoint, PyMultiPoint, "MultiPoint");

#[pymethods]
impl PyMultiPoint {
    #[new]
    fn new(points: Bound<'_, PyList>) -> Self {
        let points = points
            .iter()
            .flat_map(|any| any.extract::<(f64, f64)>())
            .map(|(x, y)| geo::point!(x: x, y: y))
            .collect();

        Self(geo::MultiPoint::new(points))
    }
}

impl_geo_wrapper!(MultiLineString, PyMultiLineString, "MultiLineString");

#[pymethods]
impl PyMultiLineString {
    #[new]
    fn new(line_strings: Bound<'_, PyList>) -> Self {
        let line_strings = line_strings
            .iter()
            .flat_map(|any| any.extract::<PyLineString>())
            .map(|line_string| line_string.0)
            .collect();

        Self(geo::MultiLineString::new(line_strings))
    }
}

impl_geo_wrapper!(MultiPolygon, PyMultiPolygon, "MultiPolygon");

#[pymethods]
impl PyMultiPolygon {
    #[new]
    fn new(polygons: Bound<'_, PyList>) -> Self {
        let polygons = polygons
            .iter()
            .flat_map(|any| any.extract::<PyPolygon>())
            .map(|polygon| polygon.0)
            .collect();

        Self(geo::MultiPolygon::new(polygons))
    }
}

impl_geo_wrapper!(
    GeometryCollection,
    PyGeometryCollection,
    "GeometryCollection"
);

#[pymethods]
impl PyGeometryCollection {
    #[new]
    fn new(geometries: Bound<'_, PyList>) -> Self {
        let geometries = geometries
            .iter()
            .flat_map(|any| any.extract::<PyGeometry>())
            .map(|geometry| <PyGeometry as Into<geo::Geometry>>::into(geometry))
            .collect::<Vec<_>>();

        Self(geo::GeometryCollection::from(geometries))
    }
}

impl_geo_wrapper!(Rect, PyRect, "Rect");

#[pymethods]
impl PyRect {
    #[new]
    fn new(corner_1: (f64, f64), corner_2: (f64, f64)) -> Self {
        Self(geo::Rect::new(corner_1, corner_2))
    }
}

impl_geo_wrapper!(Triangle, PyTriangle, "Triangle");

#[pymethods]
impl PyTriangle {
    #[new]
    fn new(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> Self {
        Self(geo::Triangle::new(
            geo::coord! {x: a.0, y: a.1},
            geo::coord! {x: b.0, y: b.1},
            geo::coord! {x: c.0, y: c.1},
        ))
    }
}
