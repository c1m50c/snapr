//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Line`], and [`geo::LineString`] primitives.

use geo::MapCoords;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_to_pixel,
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

impl Drawable for geo::Line<f64> {
    fn draw(
        &self,
        snapr: &Snapr,
        styles: &[Style],
        pixmap: &mut Pixmap,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), crate::Error> {
        let line = self.map_coords(|coord| epsg_4326_to_pixel(snapr, zoom, center, &coord));
        let style = Style::for_line(styles).unwrap_or_default();

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(line.start.x as f32, line.start.y as f32);
        path_builder.line_to(line.end.x as f32, line.end.y as f32);

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

    fn geometry(&self) -> Option<geo::Geometry<f64>>
    where
        Self: Sized,
    {
        Some(geo::Geometry::Line(self.clone()))
    }
}

impl Drawable for geo::LineString<f64> {
    fn draw(
        &self,
        snapr: &Snapr,
        styles: &[Style],
        pixmap: &mut Pixmap,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), crate::Error> {
        let style = Style::for_line(styles).unwrap_or_default();
        let mut path_builder = PathBuilder::new();

        let line_string = self.map_coords(|coord| epsg_4326_to_pixel(snapr, zoom, center, &coord));

        for (index, point) in line_string.points().enumerate() {
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

    fn geometry(&self) -> Option<geo::Geometry<f64>>
    where
        Self: Sized,
    {
        Some(geo::Geometry::LineString(self.clone()))
    }
}

impl Drawable for geo::MultiLineString<f64> {
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

    fn geometry(&self) -> Option<geo::Geometry<f64>>
    where
        Self: Sized,
    {
        Some(geo::Geometry::MultiLineString(self.clone()))
    }
}
