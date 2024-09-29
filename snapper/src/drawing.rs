use image::Rgba;
use imageproc::drawing::draw_filled_circle_mut;

pub trait DrawableGeometry {
    fn draw(&self, image: &mut image::RgbaImage) -> Result<(), crate::Error>;
}

impl<T> DrawableGeometry for geo::Point<T>
where
    T: geo::CoordNum
{
    fn draw(&self, image: &mut image::RgbaImage) -> Result<(), crate::Error> {
        let x = self.x().to_u32()
            .ok_or(crate::Error::PrimitiveNumberConversion)?;

        let y = self.y().to_u32()
            .ok_or(crate::Error::PrimitiveNumberConversion)?;

        draw_filled_circle_mut(
            image,
            (x as i32, y as i32),
            2,
            Rgba([255, 0, 0, 255])
        );

        Ok(())
    }
}

impl<T> DrawableGeometry for geo::Geometry<T>
where
    T: geo::CoordNum
{
    fn draw(&self, image: &mut image::RgbaImage) -> Result<(), crate::Error> {
        match self {
            Self::Point(point) => point.draw(image),
            _ => unimplemented!(),
        }
    }
}