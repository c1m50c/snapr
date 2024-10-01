//! Stylable wrappers of [`geo::Polygon`], [`geo::MultiPolygon`], [`geo::Rect`] and [`geo::Triangle`].

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_point_to_pixel_point,
        style::{ColorOptions, Style},
        Drawable,
    },
    Snapper,
};

use super::{
    line::StyledLineStringOptions,
    macros::impl_styled,
    point::{StyledPoint, StyledPointOptions},
};

/// Style options for [`StyledPolygon`].
#[derive(Clone, Debug, PartialEq)]
pub struct StyledPolygonOptions {
    pub color_options: ColorOptions,
    pub point_options: StyledPointOptions,
    pub line_string_options: StyledLineStringOptions,
}

impl Default for StyledPolygonOptions {
    fn default() -> Self {
        Self {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(248, 248, 248, 64),
                border: None,
                ..ColorOptions::default()
            },
            point_options: StyledPointOptions::default(),
            line_string_options: StyledLineStringOptions::default(),
        }
    }
}

impl_styled!(Polygon, StyledPolygon, StyledPolygonOptions);

impl<T> Drawable for StyledPolygon<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledPolygon(geometry, style) = &self;
        let options = style.options(self);

        let converted_points = geometry
            .exterior()
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

        path_builder.close();

        if let Some(lines) = path_builder.finish() {
            pixmap.fill_path(
                &lines,
                &Paint {
                    shader: Shader::SolidColor(options.color_options.foreground),
                    anti_alias: options.color_options.anti_alias,
                    ..Paint::default()
                },
                FillRule::default(),
                Transform::default(),
                None,
            );

            if let Some(border) = options.line_string_options.color_options.border {
                pixmap.stroke_path(
                    &lines,
                    &Paint {
                        shader: Shader::SolidColor(
                            options.line_string_options.color_options.background,
                        ),
                        anti_alias: options.line_string_options.color_options.anti_alias,
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
                    shader: Shader::SolidColor(
                        options.line_string_options.color_options.foreground,
                    ),
                    anti_alias: options.line_string_options.color_options.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: options.line_string_options.width,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        geometry.exterior().points().try_for_each(|point| {
            StyledPoint(point, Style::Static(options.point_options.clone()))
                .draw(snapper, pixmap, center)
        })?;

        Ok(())
    }
}

/// Style options for [`StyledMultiPolygon`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledMultiPolygonOptions {
    pub polygon_options: StyledPolygonOptions,
}

impl_styled!(MultiPolygon, StyledMultiPolygon, StyledMultiPolygonOptions);

impl<T> Drawable for StyledMultiPolygon<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledMultiPolygon(geometry, style) = &self;
        let options = style.options(self);

        for polygon in geometry.into_iter() {
            let styled = StyledPolygon(
                polygon.clone(),
                Style::Static(options.polygon_options.clone()),
            );
            styled.draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}

/// Style options for [`StyledRect`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledRectOptions {
    pub polygon_options: StyledPolygonOptions,
}

impl_styled!(Rect, StyledRect, StyledRectOptions);

impl<T> Drawable for StyledRect<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledRect(geometry, style) = &self;
        let options = style.options(self);

        let polygon = StyledPolygon(
            geometry.to_polygon(),
            Style::Static(options.polygon_options.clone()),
        );
        polygon.draw(snapper, pixmap, center)?;

        Ok(())
    }
}

/// Style options for [`StyledTriangle`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledTriangleOptions {
    pub polygon_options: StyledPolygonOptions,
}

impl_styled!(Triangle, StyledTriangle, StyledTriangleOptions);

impl<T> Drawable for StyledTriangle<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledTriangle(geometry, style) = &self;
        let options = style.options(self);

        let polygon = StyledPolygon(
            geometry.to_polygon(),
            Style::Static(options.polygon_options.clone()),
        );
        polygon.draw(snapper, pixmap, center)?;

        Ok(())
    }
}
