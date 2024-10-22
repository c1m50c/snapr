//! Contains [`Drawable`](super::Drawable) implementations and [`Styles`](super::style::Style) for [`geo`] primitives.

use tiny_skia::Pixmap;

use super::{Drawable, DrawingState};

pub mod line;
pub mod point;
pub mod polygon;

impl Drawable for geo::Geometry<f64> {
    fn draw(&self, pixmap: &mut Pixmap, state: &DrawingState) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(pixmap, state),
            Self::Line(geometry) => geometry.draw(pixmap, state),
            Self::LineString(geometry) => geometry.draw(pixmap, state),
            Self::Polygon(geometry) => geometry.draw(pixmap, state),
            Self::MultiPoint(geometry) => geometry.draw(pixmap, state),
            Self::MultiLineString(geometry) => geometry.draw(pixmap, state),
            Self::MultiPolygon(geometry) => geometry.draw(pixmap, state),
            Self::Rect(geometry) => geometry.draw(pixmap, state),
            Self::Triangle(geometry) => geometry.draw(pixmap, state),

            Self::GeometryCollection(geometry) => geometry
                .into_iter()
                .try_for_each(|geometry| geometry.draw(pixmap, state)),
        }
    }
}
