use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_point_to_pixel_point,
        style::{ColorOptions, Style},
        Drawable,
    },
    Snapper,
};

use super::{
    macros::impl_styled,
    point::{StyledPoint, StyledPointOptions},
};

#[derive(Clone, Debug, PartialEq)]
pub struct StyledLineOptions {
    pub color_options: ColorOptions,
    pub start_point_options: StyledPointOptions,
    pub end_point_options: StyledPointOptions,
    pub width: f32,
}

impl Default for StyledLineOptions {
    fn default() -> Self {
        Self {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(196, 196, 196, 255),
                border: Some(4.0),
                ..ColorOptions::default()
            },
            start_point_options: StyledPointOptions::default(),
            end_point_options: StyledPointOptions::default(),
            width: 3.0,
        }
    }
}

impl_styled!(Line, StyledLine, StyledLineOptions);

impl<T> Drawable for StyledLine<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledLine(geometry, style) = &self;
        let options = style.options(self);

        let start_point = epsg_4326_point_to_pixel_point(snapper, center, &geometry.start_point())?;
        let end_point = epsg_4326_point_to_pixel_point(snapper, center, &geometry.end_point())?;

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(start_point.x() as f32, start_point.y() as f32);
        path_builder.line_to(end_point.x() as f32, end_point.y() as f32);

        let line = path_builder
            .finish()
            .ok_or(crate::Error::PathConstruction)?;

        if let Some(border) = options.color_options.border {
            pixmap.stroke_path(
                &line,
                &Paint {
                    shader: Shader::SolidColor(options.color_options.background),
                    anti_alias: options.color_options.anti_alias,
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
                shader: Shader::SolidColor(options.color_options.foreground),
                anti_alias: options.color_options.anti_alias,
                ..Paint::default()
            },
            &Stroke {
                width: options.width,
                ..Stroke::default()
            },
            Transform::default(),
            None,
        );

        StyledPoint(
            geometry.start_point(),
            Style::Static(options.start_point_options.clone()),
        )
        .draw(snapper, pixmap, center)?;

        StyledPoint(
            geometry.end_point(),
            Style::Static(options.end_point_options.clone()),
        )
        .draw(snapper, pixmap, center)?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyledLineStringOptions {
    pub color_options: ColorOptions,
    pub point_options: StyledPointOptions,
    pub width: f32,
}

impl Default for StyledLineStringOptions {
    fn default() -> Self {
        Self {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(196, 196, 196, 255),
                border: Some(4.0),
                ..ColorOptions::default()
            },
            point_options: StyledPointOptions::default(),
            width: 3.0,
        }
    }
}

impl_styled!(LineString, StyledLineString, StyledLineStringOptions);

impl<T> Drawable for StyledLineString<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledLineString(geometry, style) = &self;
        let options = style.options(self);

        let converted_points = geometry
            .points()
            .flat_map(|point| epsg_4326_point_to_pixel_point(snapper, center, &point))
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
            if let Some(border) = options.color_options.border {
                pixmap.stroke_path(
                    &lines,
                    &Paint {
                        shader: Shader::SolidColor(options.color_options.background),
                        anti_alias: options.color_options.anti_alias,
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
                    shader: Shader::SolidColor(options.color_options.foreground),
                    anti_alias: options.color_options.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: options.width,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        geometry.points().try_for_each(|point| {
            StyledPoint(point, Style::Static(options.clone().point_options))
                .draw(snapper, pixmap, center)
        })?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledMultiLineStringOptions {
    pub line_string_options: StyledLineStringOptions,
}

impl_styled!(
    MultiLineString,
    StyledMultiLineString,
    StyledMultiLineStringOptions
);

impl<T> Drawable for StyledMultiLineString<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledMultiLineString(geometry, style) = &self;
        let options = style.options(self);

        for line_string in geometry.into_iter() {
            let styled = StyledLineString(
                line_string.clone(),
                Style::Static(options.line_string_options.clone()),
            );
            styled.draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}
