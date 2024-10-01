use tiny_skia::Pixmap;

use crate::Snapper;

use super::styled_geo::StyledGeometry;

pub trait Drawable {
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error>;
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
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Line(geometry) => geometry.draw(snapper, pixmap, center),
            Self::LineString(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Polygon(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiPoint(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiLineString(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiPolygon(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Rect(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Triangle(geometry) => geometry.draw(snapper, pixmap, center),
        }
    }
}
