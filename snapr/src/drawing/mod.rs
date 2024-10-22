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

/// Converts an [`EPSG:4326`](https://epsg.io/4326) point to one that represents a pixel in a snapshot.
/// Used as a shortcut in converting coordinates during drawing.
pub fn epsg_4326_point_to_pixel_point<T: geo::CoordNum>(
    snapr: &Snapr,
    zoom: u8,
    center: geo::Point<f64>,
    point: &geo::Point<T>,
) -> Result<geo::Point<i32>, crate::Error> {
    let x = point
        .x()
        .to_f64()
        .ok_or(crate::Error::PrimitiveNumberConversion)?;

    let y = point
        .y()
        .to_f64()
        .ok_or(crate::Error::PrimitiveNumberConversion)?;

    Ok(snapr.epsg_4326_to_pixel(zoom, center, geo::point!(x: x, y: y)))
}
