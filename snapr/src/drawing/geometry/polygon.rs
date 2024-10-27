//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Polygon`], [`geo::Rect`], and [`geo::Triangle`] primitives.

use std::fmt;

use geo::MapCoords;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::drawing::{
    style::{ColorOptions, Effect, Styleable, Styled},
    Context, Drawable,
};

use super::{line::LineStringStyle, macros::impl_styled_geo, point::PointStyle};

/// A [`Style`] that can be applied to [`geo::Polygon`], [`geo::Rect`], and [`geo::Triangle`] primitives.
#[derive(Clone)]
pub struct PolygonStyle<'a> {
    pub color_options: ColorOptions,
    pub effect: Option<Effect<'a, geo::Polygon<f64>, Self>>,
    pub line_style: LineStringStyle<'a>,
    pub point_style: PointStyle<'a>,
}

impl<'a> fmt::Debug for PolygonStyle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!($style))
            .field("color_options", &self.color_options)
            .field("line_style", &self.line_style)
            .field("point_style", &self.point_style)
            .finish()
    }
}

impl<'a> Default for PolygonStyle<'a> {
    fn default() -> Self {
        Self {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(248, 248, 248, 64),
                border: None,
                ..ColorOptions::default()
            },
            effect: None,
            line_style: LineStringStyle::default(),
            point_style: PointStyle::default(),
        }
    }
}

impl_styled_geo!(
    Polygon,
    PolygonStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        let style = match &self.style.effect {
            Some(effect) => {
                &(effect
                    .clone()
                    .apply(self.style.clone(), &self.inner, context))
            }

            None => &self.style,
        };

        let pixel_polygon = self
            .inner
            .map_coords(|coord| context.epsg_4326_to_pixel(&coord));

        let mut path_builder = PathBuilder::new();

        for (index, point) in pixel_polygon.exterior().points().enumerate() {
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
                    shader: Shader::SolidColor(style.color_options.foreground),
                    anti_alias: style.color_options.anti_alias,
                    ..Paint::default()
                },
                FillRule::default(),
                Transform::default(),
                None,
            );

            if let Some(border) = style.line_style.color_options.border {
                pixmap.stroke_path(
                    &lines,
                    &Paint {
                        shader: Shader::SolidColor(style.line_style.color_options.background),
                        anti_alias: style.line_style.color_options.anti_alias,
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
                    shader: Shader::SolidColor(style.line_style.color_options.foreground),
                    anti_alias: style.line_style.color_options.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: style.line_style.width,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        self.inner
            .exterior()
            .points()
            .enumerate()
            .try_for_each(|(index, point)| {
                let context = &Context {
                    index,
                    ..context.clone()
                };

                point
                    .as_styled(style.point_style.clone())
                    .draw(pixmap, context)
            })?;

        Ok(())
    }
);

impl_styled_geo!(
    MultiPolygon,
    PolygonStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        self.inner
            .iter()
            .map(|polygon| polygon.as_styled(self.style.clone()))
            .try_for_each(|polygon| polygon.draw(pixmap, context))
    }
);

impl_styled_geo!(
    Rect,
    PolygonStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        self.inner
            .to_polygon()
            .as_styled(self.style.clone())
            .draw(pixmap, context)
    }
);

impl_styled_geo!(
    Triangle,
    PolygonStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        self.inner
            .to_polygon()
            .as_styled(self.style.clone())
            .draw(pixmap, context)
    }
);
