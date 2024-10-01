//! Contains utilities to draw objects on top of map tiles.
//!
//! ## Example
//!
//! ```rust
//! use geo::Point;
//! use snapper::{drawing::{epsg_4326_point_to_pixel_point, Drawable}, Error, Snapper};
//! use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Shader, Transform};
//!
//! #[derive(Debug)]
//! struct PointWrapper(Point<f64>);
//!
//! impl Drawable for PointWrapper {
//!     fn draw(&self, snapper: &Snapper, pixmap: &mut Pixmap, center: Point) -> Result<(), Error> {
//!         let pixel_point = epsg_4326_point_to_pixel_point(snapper, center, &self.0)?;
//!
//!         let mut path_builder = PathBuilder::new();
//!         path_builder.push_circle(0.0, 0.0, 3.0);
//!
//!          pixmap.fill_path(
//!             &path_builder.finish().unwrap(),
//!             &Paint {
//!                 shader: Shader::SolidColor(Color::from_rgba8(255, 0, 0, 255)),
//!                 ..Default::default()
//!             },
//!             FillRule::default(),
//!             Transform::default(),
//!             None,
//!         );
//!
//!         Ok(())
//!     }
//! }
//! ```

use tiny_skia::Pixmap;

use crate::Snapper;

pub mod style;

/// Represents a _drawable_ object.
///
/// A [`Drawable`] object will _draw_ to the given `pixmap` based on the `snapper` and `center` arguments.
/// See [`drawing`](self) for more details.
pub trait Drawable {
    /// Function that's called when its time for an object to be drawn.
    /// See [`Drawable`] for more details.
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error>;
}

/// Converts an [`EPSG:4326`](https://epsg.io/4326) point to one that represents a pixel in a snapshot.
/// Used as a shortcut in converting coordinates during drawing.
pub fn epsg_4326_point_to_pixel_point<T: geo::CoordNum>(
    snapper: &Snapper,
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

    Ok(snapper.epsg_4326_to_pixel(center, geo::point!(x: x, y: y)))
}
