//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Line`], and [`geo::LineString`] primitives.

use geo::MapCoords;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::drawing::{
    style::{ColorOptions, Effect, Styleable, Styled},
    Context, Drawable,
};

use super::{macros::impl_styled_geo, point::PointStyle};

macro_rules! impl_line_style {
    ($style: ident, $line: ident) => {
        #[derive(Clone, Debug, PartialEq)]
        #[doc = concat!("A style that can be applied to the [`geo::", stringify!($line), "`] primitive.")]
        pub struct $style {
            pub color_options: ColorOptions,
            pub point_style: PointStyle,
            pub width: f32,
            pub effect: Option<Effect<geo::$line<f64>, Self>>,
        }

        impl Default for $style {
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
    LineStyle,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        let style = match self.style.effect {
            Some(effect) => &((effect)(self.style.clone(), &self.inner, context)),
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
    LineStringStyle,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        let style = match self.style.effect {
            Some(effect) => &((effect)(self.style.clone(), &self.inner, context)),
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
    LineStringStyle,
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        self.inner
            .iter()
            .map(|line_string| line_string.as_styled(self.style.clone()))
            .try_for_each(|line_string| line_string.draw(pixmap, context))
    }
);
