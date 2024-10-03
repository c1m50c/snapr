//! Stylable and [`Drawable`] wrappers around [`geo`] primitive types.

use tiny_skia::{Path, PathBuilder, Pixmap};

use crate::{drawing::Drawable, Snapper};

pub use line::*;
pub use point::*;
pub use polygon::*;

pub mod line;
pub mod point;
pub mod polygon;

/// Represents an easily [`Drawable`] _shape_.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f32 },
}

impl Shape {
    /// Converts the [`Shape`] to a [`Path`] modeling the selected variant.
    pub fn to_path(&self, x: f32, y: f32) -> Result<Path, crate::Error> {
        let mut path_builder = PathBuilder::new();

        match self {
            Self::Circle { radius } => {
                path_builder.push_circle(x, y, *radius);
            }
        }

        path_builder.finish().ok_or(crate::Error::PathConstruction)
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Circle { radius: 4.0 }
    }
}

/// Contains variants for each stylable [`geo`] primitive type wrapper.
#[derive(Clone, Debug, PartialEq)]
pub enum StyledGeometry<T: geo::CoordNum = f64> {
    Point(point::StyledPoint<T>),
    Line(line::StyledLine<T>),
    LineString(line::StyledLineString<T>),
    Polygon(polygon::StyledPolygon<T>),
    MultiPoint(point::StyledMultiPoint<T>),
    MultiLineString(line::StyledMultiLineString<T>),
    MultiPolygon(polygon::StyledMultiPolygon<T>),
    Rect(polygon::StyledRect<T>),
    Triangle(polygon::StyledTriangle<T>),
}

impl<T> Drawable for StyledGeometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::Line(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::LineString(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::Polygon(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::MultiPoint(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::MultiLineString(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::MultiPolygon(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::Rect(geometry) => geometry.draw(snapper, pixmap, center, zoom),
            Self::Triangle(geometry) => geometry.draw(snapper, pixmap, center, zoom),
        }
    }
}

// FIXME: The below `Into` implementation should probably be a `From` implementation.
// We don't currently represent a styled variant of `GeometryCollection`, but we probably should.

#[allow(clippy::from_over_into)]
impl<T: geo::CoordNum> Into<geo::Geometry<T>> for StyledGeometry<T> {
    fn into(self) -> geo::Geometry<T> {
        match self {
            Self::Point(geometry) => geo::Geometry::Point(geometry.0),
            Self::Line(geometry) => geo::Geometry::Line(geometry.0),
            Self::LineString(geometry) => geo::Geometry::LineString(geometry.0),
            Self::Polygon(geometry) => geo::Geometry::Polygon(geometry.0),
            Self::MultiPoint(geometry) => geo::Geometry::MultiPoint(geometry.0),
            Self::MultiLineString(geometry) => geo::Geometry::MultiLineString(geometry.0),
            Self::MultiPolygon(geometry) => geo::Geometry::MultiPolygon(geometry.0),
            Self::Rect(geometry) => geo::Geometry::Rect(geometry.0),
            Self::Triangle(geometry) => geo::Geometry::Triangle(geometry.0),
        }
    }
}

mod macros {
    /// Macro for implementing requirements for a styled geometry type.
    macro_rules! impl_styled {
        ($base: ident, $styled: ident, $options: ident) => {
            #[derive(Clone, Debug, PartialEq)]
            #[doc = concat!("Wrapper around [`", stringify!($base), "`](geo::", stringify!($base), ") that enables styling with [`", stringify!($options), "`].")]
            pub struct $styled<T: geo::CoordNum = f64>(
                pub geo::$base<T>,
                pub crate::drawing::style::Style<$options, geo::$base<T>>,
            );

            impl<T: geo::CoordNum> From<geo::$base<T>> for $styled<T> {
                fn from(value: geo::$base<T>) -> Self {
                    Self(value, crate::drawing::style::Style::default())
                }
            }

            impl<T: geo::CoordNum> From<geo::$base<T>>
                for crate::drawing::style::geo::StyledGeometry<T>
            {
                fn from(value: geo::$base<T>) -> Self {
                    Self::$base($styled(value, crate::drawing::style::Style::default()))
                }
            }

            #[allow(clippy::from_over_into)]
            impl<T: geo::CoordNum> Into<crate::drawing::style::geo::StyledGeometry<T>>
                for $styled<T>
            {
                fn into(self) -> crate::drawing::style::geo::StyledGeometry<T> {
                    crate::drawing::style::geo::StyledGeometry::$base(self)
                }
            }

            impl<T: geo::CoordNum> std::ops::Deref for $styled<T> {
                type Target = geo::$base<T>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T: geo::CoordNum> std::ops::DerefMut for $styled<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        };
    }

    pub(super) use impl_styled;
}
