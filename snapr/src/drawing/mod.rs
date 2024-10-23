//! Contains utilities to draw objects on top of map tiles.

use tiny_skia::Pixmap;

use crate::Snapr;

pub mod geometry;
pub mod style;

#[cfg(feature = "svg")]
pub mod svg;

/// Passed to [`Drawable::draw`] calls, represents the _[`Context`]_ of those calls.
#[derive(Debug)]
pub struct Context<'a> {
    pub snapr: &'a Snapr<'a>,
    pub center: geo::Point<f64>,
    pub zoom: u8,
}

impl<'a> Context<'a> {
    /// Converts an [`EPSG:4326`](https://epsg.io/4326) coordinate to one that represents a pixel in a snapshot.
    /// Used as a shortcut in converting coordinates during drawing.
    pub fn epsg_4326_to_pixel(&self, coord: &geo::Coord<f64>) -> geo::Coord<i32> {
        let epsg_3857_point = Snapr::epsg_4326_to_epsg_3857(self.zoom, geo::Point::from(*coord))
            - Snapr::epsg_4326_to_epsg_3857(self.zoom, self.center);

        geo::coord!(
            x: (epsg_3857_point.x().fract() * self.snapr.tile_size as f64 + self.snapr.width as f64 / 2.0).round() as i32,
            y: (epsg_3857_point.y().fract() * self.snapr.tile_size as f64 + self.snapr.height as f64 / 2.0).round() as i32,
        )
    }
}

/// Represents a _drawable_ object.
///
/// A [`Drawable`] object will _draw_ to the given `pixmap` based on the given arguments.
pub trait Drawable {
    /// Function that's called when its time for an object to be drawn.
    /// See [`Drawable`] for more details.
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error>;
}
