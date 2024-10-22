//! Contains [`Drawable`](super::Drawable) implementations and [`Styles`](super::style::Style) for [`geo`] primitives.

use tiny_skia::Pixmap;

use crate::Snapr;

use super::{style::Style, Drawable};

pub mod line;
pub mod point;
pub mod polygon;

impl<T> Drawable for geo::Geometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapr: &Snapr,
        styles: &[Style],
        pixmap: &mut Pixmap,
        center: geo::Point<f64>,
        zoom: u8,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::Line(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::LineString(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::Polygon(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::MultiPoint(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::MultiLineString(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::MultiPolygon(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),

            Self::GeometryCollection(geometry) => geometry
                .into_iter()
                .try_for_each(|geometry| geometry.draw(snapr, styles, pixmap, center, zoom)),

            Self::Rect(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
            Self::Triangle(geometry) => geometry.draw(snapr, styles, pixmap, center, zoom),
        }
    }
}
