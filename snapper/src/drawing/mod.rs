use imageproc::drawing::{draw_filled_circle_mut, draw_line_segment_mut};

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

impl<T> DrawableGeometry for geo::LineString<T>
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
        let mut points = self.points().peekable();

        while let Some(start_point) = points.next() {
            if let Some(end_point) = points.peek() {
                let start_point = epsg_4326_point_to_pixel_point(snapper, center, &start_point)?;
                let end_point = epsg_4326_point_to_pixel_point(snapper, center, &end_point)?;

                draw_line_segment_mut(
                    image,
                    (start_point.x() as f32, start_point.y() as f32),
                    (end_point.x() as f32, end_point.y() as f32),
                    style.background,
                );
            }

            start_point.draw(snapper, image, style, center)?;
        }

        Ok(())
    }
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
        let point = epsg_4326_point_to_pixel_point(snapper, center, self)?;

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
            Self::LineString(line_string) => line_string.draw(snapper, image, style, center),
            _ => todo!("Implement drawing function for all `geo::Geometry` variants"),
        }
    }
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
