//! Contains [`Drawable`](super::Drawable) implementations and [`Styles`](super::style::Style) for [`geo`] primitives.

use tiny_skia::Pixmap;

use super::{Context, Drawable};

pub mod line;
pub mod point;
pub mod polygon;

impl Drawable for geo::Geometry<f64> {
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(pixmap, context),
            Self::Line(geometry) => geometry.draw(pixmap, context),
            Self::LineString(geometry) => geometry.draw(pixmap, context),
            Self::Polygon(geometry) => geometry.draw(pixmap, context),
            Self::MultiPoint(geometry) => geometry.draw(pixmap, context),
            Self::MultiLineString(geometry) => geometry.draw(pixmap, context),
            Self::MultiPolygon(geometry) => geometry.draw(pixmap, context),
            Self::Rect(geometry) => geometry.draw(pixmap, context),
            Self::Triangle(geometry) => geometry.draw(pixmap, context),

            Self::GeometryCollection(geometry) => geometry
                .into_iter()
                .try_for_each(|geometry| geometry.draw(pixmap, context)),
        }
    }
}
