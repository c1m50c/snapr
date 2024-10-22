//! Contains utilities to draw objects on top of map tiles.

use style::Style;
use tiny_skia::Pixmap;

use crate::Snapr;

pub mod geometry;
pub mod style;

#[cfg(feature = "svg")]
pub mod svg;

/// Represents a _drawable_ object.
///
/// A [`Drawable`] object will _draw_ to the given `pixmap` based on the given arguments.
pub trait Drawable {
    /// Function that's called when its time for an object to be drawn.
    /// See [`Drawable`] for more details.
    fn draw(
        &self,
        snapr: &Snapr,
        styles: &[Style],
        pixmap: &mut Pixmap,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), crate::Error>;

    /// Returns the underlying [`Geometry`](geo::Geometry) that the [`Drawable`] represents.
    /// If the [`Drawable`] type doesn't represent something spatial, i.e. something that shouldn't be accounted for when calculating map bounds, then this method returns [`None`].
    fn geometry(&self) -> Option<geo::Geometry<f64>> {
        None
    }
}

/// Converts an [`EPSG:4326`](https://epsg.io/4326) coordinate to one that represents a pixel in a snapshot.
/// Used as a shortcut in converting coordinates during drawing.
pub fn epsg_4326_to_pixel(
    snapr: &Snapr,
    zoom: u8,
    center: geo::Point<f64>,
    coord: &geo::Coord<f64>,
) -> geo::Coord<i32> {
    let epsg_3857_point = Snapr::epsg_4326_to_epsg_3857(zoom, geo::Point::from(*coord))
        - Snapr::epsg_4326_to_epsg_3857(zoom, center);

    geo::coord!(
        x: (epsg_3857_point.x().fract() * snapr.tile_size as f64 + snapr.width as f64 / 2.0).round() as i32,
        y: (epsg_3857_point.y().fract() * snapr.tile_size as f64 + snapr.height as f64 / 2.0).round() as i32,
    )
}
