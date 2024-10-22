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
/// See [`drawing`](self) for more details.
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
}

/// Converts an [`EPSG:4326`](https://epsg.io/4326) coordinate to one that represents a pixel in a snapshot.
/// Used as a shortcut in converting coordinates during drawing.
pub fn epsg_4326_to_pixel(
    snapr: &Snapr,
    zoom: u8,
    center: geo::Point<f64>,
    coord: &geo::Coord<f64>,
) -> geo::Coord<i32> {
    snapr
        .epsg_4326_to_pixel(zoom, center, geo::point!(x: coord.x, y: coord.y))
        .into()
}
