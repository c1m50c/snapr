use std::ops::{Deref, DerefMut};

use pyo3::prelude::*;

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
    fn new(start: PyPoint, end: PyPoint) -> Self {
        Self(geo::Line::new(start.0, end.0))
    }
}

impl_geo_wrapper!(LineString, PyLineString, "LineString");

#[pymethods]
impl PyLineString {
    #[new]
    fn new(points: Vec<PyPoint>) -> Self {
        let coords = points.into_iter().map(|x| x.0.into()).collect();
        Self(geo::LineString::new(coords))
    }
}

impl_geo_wrapper!(Polygon, PyPolygon, "Polygon");

#[pymethods]
impl PyPolygon {
    #[new]
    fn new(exterior: PyLineString, interiors: Vec<PyLineString>) -> Self {
        let interiors = interiors.into_iter().map(PyLineString::into).collect();
        Self(geo::Polygon::new(exterior.0, interiors))
    }
}

impl_geo_wrapper!(MultiPoint, PyMultiPoint, "MultiPoint");

#[pymethods]
impl PyMultiPoint {
    #[new]
    fn new(points: Vec<PyPoint>) -> Self {
        let points = points.into_iter().map(PyPoint::into).collect();
        Self(geo::MultiPoint::new(points))
    }
}

impl_geo_wrapper!(MultiLineString, PyMultiLineString, "MultiLineString");

#[pymethods]
impl PyMultiLineString {
    #[new]
    fn new(line_strings: Vec<PyLineString>) -> Self {
        let line_strings = line_strings.into_iter().map(PyLineString::into).collect();
        Self(geo::MultiLineString::new(line_strings))
    }
}

impl_geo_wrapper!(MultiPolygon, PyMultiPolygon, "MultiPolygon");

#[pymethods]
impl PyMultiPolygon {
    #[new]
    fn new(polygons: Vec<PyPolygon>) -> Self {
        let polygons = polygons.into_iter().map(PyPolygon::into).collect();
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
    fn new(geometries: Vec<PyGeometry>) -> Self {
        Self(geo::GeometryCollection::from(geometries))
    }
}

impl_geo_wrapper!(Rect, PyRect, "Rect");

#[pymethods]
impl PyRect {
    #[new]
    fn new(corner_1: PyPoint, corner_2: PyPoint) -> Self {
        Self(geo::Rect::new(corner_1.0, corner_2.0))
    }
}

impl_geo_wrapper!(Triangle, PyTriangle, "Triangle");

#[pymethods]
impl PyTriangle {
    #[new]
    fn new(a: PyPoint, b: PyPoint, c: PyPoint) -> Self {
        Self(geo::Triangle::new(
            geo::coord! {x: a.x(), y: a.y()},
            geo::coord! {x: b.x(), y: b.y()},
            geo::coord! {x: c.x(), y: c.y()},
        ))
    }
}
