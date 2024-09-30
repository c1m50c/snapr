use imageproc::drawing::draw_filled_circle_mut;

use crate::Snapper;

pub use style::Style;

mod style;

pub trait DrawableGeometry {
    fn draw(
        &self,
        snapper: &Snapper,
        image: &mut image::RgbaImage,
        style: &style::Style,
        center: geo::Point,
    ) -> Result<(), crate::Error>;
}

impl<T> DrawableGeometry for geo::Point<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        image: &mut image::RgbaImage,
        style: &style::Style,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let x = self
            .x()
            .to_f64()
            .ok_or(crate::Error::PrimitiveNumberConversion)?;

        let y = self
            .y()
            .to_f64()
            .ok_or(crate::Error::PrimitiveNumberConversion)?;

        let point = snapper.epsg_4326_to_pixel(center, geo::point!(x: x, y: y));

        draw_filled_circle_mut(image, (point.x(), point.y()), 4, style.background);
        draw_filled_circle_mut(image, (point.x(), point.y()), 3, style.foreground);

        Ok(())
    }
}

impl<T> DrawableGeometry for geo::Geometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        image: &mut image::RgbaImage,
        style: &style::Style,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(point) => point.draw(snapper, image, style, center),
            _ => todo!("Implement drawing function for all `geo::Geometry` variants"),
        }
    }
}
