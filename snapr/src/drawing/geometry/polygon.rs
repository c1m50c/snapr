//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Polygon`], [`geo::Rect`], and [`geo::Triangle`] primitives.

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_point_to_pixel_point,
        style::{ColorOptions, Style},
        Drawable,
    },
    Snapr,
};

/// A [`Style`] that can be applied to [`geo::Polygon`], [`geo::Rect`], and [`geo::Triangle`] primitives.
#[derive(Clone, Debug, PartialEq)]
pub struct PolygonStyle {
    pub color_options: ColorOptions,
}

impl Default for PolygonStyle {
    fn default() -> Self {
        Self {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(248, 248, 248, 64),
                border: None,
                ..ColorOptions::default()
            },
        }
    }
}

impl<T> Drawable for geo::Polygon<T>
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
        let line_style = Style::for_line(styles).unwrap_or_default();
        let polygon_style = Style::for_polygon(styles).unwrap_or_default();

        let converted_points = self
            .exterior()
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

        path_builder.close();

        if let Some(lines) = path_builder.finish() {
            pixmap.fill_path(
                &lines,
                &Paint {
                    shader: Shader::SolidColor(polygon_style.color_options.foreground),
                    anti_alias: polygon_style.color_options.anti_alias,
                    ..Paint::default()
                },
                FillRule::default(),
                Transform::default(),
                None,
            );

            if let Some(border) = line_style.color_options.border {
                pixmap.stroke_path(
                    &lines,
                    &Paint {
                        shader: Shader::SolidColor(line_style.color_options.background),
                        anti_alias: line_style.color_options.anti_alias,
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
                    shader: Shader::SolidColor(line_style.color_options.foreground),
                    anti_alias: line_style.color_options.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: line_style.width,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        self.exterior()
            .points()
            .try_for_each(|point| point.draw(snapr, styles, pixmap, center, zoom))?;

        Ok(())
    }
}

impl<T> Drawable for geo::MultiPolygon<T>
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
            .try_for_each(|polygon| polygon.draw(snapr, styles, pixmap, center, zoom))
    }
}

impl<T> Drawable for geo::Rect<T>
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
        self.to_polygon().draw(snapr, styles, pixmap, center, zoom)
    }
}

impl<T> Drawable for geo::Triangle<T>
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
        self.to_polygon().draw(snapr, styles, pixmap, center, zoom)
    }
}
