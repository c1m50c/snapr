use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::Snapper;

use super::epsg_4326_point_to_pixel_point;

pub trait Drawable {
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error>;
}

impl<T> Drawable for geo::LineString<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let mut points = self
            .points()
            .flat_map(|point| epsg_4326_point_to_pixel_point(snapper, center, &point));

        let Some(start_point) = points.next() else {
            return Ok(());
        };

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(start_point.x() as f32, start_point.y() as f32);

        points.for_each(|point| {
            path_builder.line_to(point.x() as f32, point.y() as f32);
        });

        if let Some(line) = path_builder.finish() {
            pixmap.stroke_path(
                &line,
                &Paint {
                    shader: Shader::SolidColor(Color::from_rgba8(26, 26, 26, 255)),
                    anti_alias: true,
                    ..Paint::default()
                },
                &Stroke {
                    width: 3.0,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );

            pixmap.stroke_path(
                &line,
                &Paint {
                    shader: Shader::SolidColor(Color::from_rgba8(200, 200, 200, 255)),
                    anti_alias: true,
                    ..Paint::default()
                },
                &Stroke {
                    width: 2.0,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        for point in self.points() {
            point.draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}

impl<T> Drawable for geo::Point<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let point = epsg_4326_point_to_pixel_point(snapper, center, self)?;

        let mut path_builder = PathBuilder::new();
        path_builder.push_circle(point.x() as f32, point.y() as f32, 4.0);

        if let Some(circle) = path_builder.finish() {
            pixmap.fill_path(
                &circle,
                &Paint {
                    shader: Shader::SolidColor(Color::from_rgba8(248, 248, 248, 255)),
                    anti_alias: true,
                    ..Paint::default()
                },
                FillRule::default(),
                Transform::default(),
                None,
            );

            pixmap.stroke_path(
                &circle,
                &Paint {
                    shader: Shader::SolidColor(Color::from_rgba8(26, 26, 26, 255)),
                    anti_alias: true,
                    ..Paint::default()
                },
                &Stroke {
                    width: 1.0,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        Ok(())
    }
}

impl<T> Drawable for geo::Geometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(point) => point.draw(snapper, pixmap, center),
            Self::LineString(line_string) => line_string.draw(snapper, pixmap, center),
            _ => todo!("Implement drawing function for all `geo::Geometry` variants"),
        }
    }
}
