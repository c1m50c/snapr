//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Line`], and [`geo::LineString`] primitives.

use std::fmt;

use geo::MapCoords;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::drawing::{
    style::{ColorOptions, Effect, Styleable, Styled},
    Context, Drawable,
};

use super::{macros::impl_styled_geo, point::PointStyle};

macro_rules! impl_line_style {
    ($style: ident, $line: ident) => {
        #[derive(Clone)]
        #[doc = concat!("A style that can be applied to the [`geo::", stringify!($line), "`] primitive.")]
        pub struct $style<'a> {
            pub color_options: ColorOptions,
            pub point_style: PointStyle<'a>,
            pub width: f32,
            pub effect: Option<Effect<'a, geo::$line<f64>, Self>>,
        }

        impl<'a> fmt::Debug for $style<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($style))
                    .field("color_options", &self.color_options)
                    .field("point_style", &self.point_style)
                    .field("width", &self.width)
                    .finish()
            }
        }

        impl<'a> Default for $style<'a> {
            fn default() -> Self {
                Self {
                    color_options: ColorOptions {
                        foreground: Color::from_rgba8(196, 196, 196, 255),
                        border: Some(4.0),
                        ..ColorOptions::default()
                    },
                    point_style: PointStyle::default(),
                    width: 3.0,
                    effect: None,
                }
            }
        }
    };
}

impl_line_style!(LineStyle, Line);
impl_line_style!(LineStringStyle, LineString);

impl_styled_geo!(
    Line,
    LineStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        let style = match &self.style.effect {
            Some(effect) => {
                &(effect
                    .clone()
                    .apply(self.style.clone(), self.inner, context))
            }

            None => &self.style,
        };

        let line = self
            .inner
            .map_coords(|coord| context.epsg_4326_to_pixel(&coord));

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

        self.inner
            .start_point()
            .as_styled(style.point_style.clone())
            .draw(
                pixmap,
                &Context {
                    index: 0,
                    ..context.clone()
                },
            )?;

        self.inner
            .end_point()
            .as_styled(style.point_style.clone())
            .draw(
                pixmap,
                &Context {
                    index: 1,
                    ..context.clone()
                },
            )?;

        Ok(())
    }
);

impl_styled_geo!(
    LineString,
    LineStringStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        let style = match &self.style.effect {
            Some(effect) => {
                &(effect
                    .clone()
                    .apply(self.style.clone(), self.inner, context))
            }

            None => &self.style,
        };

        let mut path_builder = PathBuilder::new();

        let line_string = self
            .inner
            .map_coords(|coord| context.epsg_4326_to_pixel(&coord));

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

        self.inner
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
    MultiLineString,
    LineStringStyle<'_>,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        self.inner
            .iter()
            .map(|line_string| line_string.as_styled(self.style.clone()))
            .try_for_each(|line_string| line_string.draw(pixmap, context))
    }
);
