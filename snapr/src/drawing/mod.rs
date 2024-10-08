//! Contains utilities to draw objects on top of map tiles.
//!
//! ## Example
//!
//! ```rust
//! use geo::Point;
//! use snapr::{drawing::{epsg_4326_point_to_pixel_point, Drawable, style::Style}, Error, Snapr};
//! use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Shader, Transform};
//!
//! #[derive(Debug)]
//! struct PointWrapper(Point<f64>);
//!
//! impl Drawable for PointWrapper {
//!     fn draw(&self, snapr: &Snapr, _: &[Style], pixmap: &mut Pixmap, center: Point, zoom: u8) -> Result<(), Error> {
//!         let pixel_point = epsg_4326_point_to_pixel_point(snapr, zoom, center, &self.0)?;
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

impl<T> Drawable for geo::Geometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapr: &Snapr,
        styles: &[Style],
        pixmap: &mut Pixmap,
        center: geo::Point,
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
