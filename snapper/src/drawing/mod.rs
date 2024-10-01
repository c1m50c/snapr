use tiny_skia::Pixmap;

use crate::Snapper;

pub mod style;

pub trait Drawable {
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error>;
}

fn epsg_4326_point_to_pixel_point<T: geo::CoordNum>(
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
