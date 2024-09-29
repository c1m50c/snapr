use imageproc::drawing::draw_filled_circle_mut;

use crate::Snapper;

pub use style::Style;

mod style;

pub trait DrawableGeometry {
    fn draw(&self, snapper: &Snapper, image: &mut image::RgbaImage, style: style::Style, center: geo::Point) -> Result<(), crate::Error>;
}

impl<T> DrawableGeometry for geo::Point<T>
where
    T: geo::CoordNum
{
    fn draw(&self, snapper: &Snapper, image: &mut image::RgbaImage, style: style::Style, center: geo::Point) -> Result<(), crate::Error> {
        let x = self.x().to_f64()
            .ok_or(crate::Error::PrimitiveNumberConversion)
            .map(|x| snapper.latitude_to_pixel(center, x))?;

        let y = self.y().to_f64()
            .ok_or(crate::Error::PrimitiveNumberConversion)
            .map(|y| snapper.longitude_to_pixel(center, y))?;

        draw_filled_circle_mut(
            image,
            (x as i32, y as i32),
            3,
            style.background
        );

        draw_filled_circle_mut(
            image,
            (x as i32, y as i32),
            2,
            style.foreground
        );

        Ok(())
    }
}

impl<T> DrawableGeometry for geo::Geometry<T>
where
    T: geo::CoordNum
{
    fn draw(&self, snapper: &Snapper, image: &mut image::RgbaImage, style: style::Style, center: geo::Point) -> Result<(), crate::Error> {
        match self {
            Self::Point(point) => point.draw(snapper, image, style, center),
            _ => unimplemented!(),
        }
    }
}