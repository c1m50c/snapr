//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Line`], and [`geo::LineString`] primitives.

use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_point_to_pixel_point,
        style::{ColorOptions, Style},
        Drawable,
    },
    Snapr,
};

/// A [`Style`] that can be applied to [`geo::Line`] and [`geo::LineString`] primitives.
#[derive(Clone, Debug, PartialEq)]
pub struct LineStyle {
    pub color_options: ColorOptions,
    pub width: f32,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(196, 196, 196, 255),
                border: Some(4.0),
                ..ColorOptions::default()
            },
            width: 3.0,
        }
    }
}

impl<T> Drawable for geo::Line<T>
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
        let start_point = epsg_4326_point_to_pixel_point(snapr, zoom, center, &self.start_point())?;
        let end_point = epsg_4326_point_to_pixel_point(snapr, zoom, center, &self.end_point())?;
        let style = Style::for_line(styles).unwrap_or_default();

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(start_point.x() as f32, start_point.y() as f32);
        path_builder.line_to(end_point.x() as f32, end_point.y() as f32);

        let line = path_builder
            .finish()
            .ok_or(crate::Error::PathConstruction)?;

        if let Some(border) = style.color_options.border {
            pixmap.stroke_path(
                &line,
                &Paint {
                    shader: Shader::SolidColor(style.color_options.background),
                    anti_alias: style.color_options.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: border,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        pixmap.stroke_path(
            &line,
            &Paint {
                shader: Shader::SolidColor(style.color_options.foreground),
                anti_alias: style.color_options.anti_alias,
                ..Paint::default()
            },
            &Stroke {
                width: style.width,
                ..Stroke::default()
            },
            Transform::default(),
            None,
        );

        self.start_point()
            .draw(snapr, styles, pixmap, center, zoom)?;
        self.end_point().draw(snapr, styles, pixmap, center, zoom)?;

        Ok(())
    }
}

impl<T> Drawable for geo::LineString<T>
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
        let style = Style::for_line(styles).unwrap_or_default();

        let converted_points = self
            .points()
            .flat_map(|point| epsg_4326_point_to_pixel_point(snapr, zoom, center, &point))
            .enumerate();

        let mut path_builder = PathBuilder::new();

        for (index, point) in converted_points {
            if index == 0 {
                path_builder.move_to(point.x() as f32, point.y() as f32);
            } else {
                path_builder.line_to(point.x() as f32, point.y() as f32);
            }
        }

        if let Some(lines) = path_builder.finish() {
            if let Some(border) = style.color_options.border {
                pixmap.stroke_path(
                    &lines,
                    &Paint {
                        shader: Shader::SolidColor(style.color_options.background),
                        anti_alias: style.color_options.anti_alias,
                        ..Paint::default()
                    },
                    &Stroke {
                        width: border,
                        ..Stroke::default()
                    },
                    Transform::default(),
                    None,
                );
            }

            pixmap.stroke_path(
                &lines,
                &Paint {
                    shader: Shader::SolidColor(style.color_options.foreground),
                    anti_alias: style.color_options.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: style.width,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        self.points()
            .try_for_each(|point| point.draw(snapr, styles, pixmap, center, zoom))?;

        Ok(())
    }
}

impl<T> Drawable for geo::MultiLineString<T>
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
        self.into_iter()
            .try_for_each(|line_string| line_string.draw(snapr, styles, pixmap, center, zoom))
    }
}
