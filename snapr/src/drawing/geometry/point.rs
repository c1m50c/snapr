//! Contains [`Drawable`] implementations and [`Styles`](Style) for [`geo::Point`]` primitives.

use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_point_to_pixel_point,
        style::{ColorOptions, Style},
        Drawable,
    },
    Snapr,
};

/// Represents a _shape_ that can be transformed into a [`Path`] via the [`Shape::to_path`] method.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f32 },
}

impl Shape {
    /// Converts the [`Shape`] to a [`Path`] modeling the selected variant.
    pub fn to_path(&self, x: f32, y: f32) -> Result<Path, crate::Error> {
        let mut path_builder = PathBuilder::new();

        match self {
            Self::Circle { radius } => {
                path_builder.push_circle(x, y, *radius);
            }
        }

        path_builder.finish().ok_or(crate::Error::PathConstruction)
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Circle { radius: 4.0 }
    }
}

/// Controls how a [`geo::Point`] will be visualized when drawn.
#[derive(Clone, Debug, PartialEq)]
pub enum Representation {
    Shape(Shape),

    #[cfg(feature = "svg")]
    Svg(crate::drawing::svg::Svg),
}

impl Default for Representation {
    fn default() -> Self {
        Self::Shape(Shape::default())
    }
}

/// A [`Style`] that can be applied to [`geo::Point`] primitives.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PointStyle {
    pub color_options: ColorOptions,
    pub representation: Representation,

    #[cfg(feature = "svg")]
    pub label: Option<crate::drawing::svg::Label>,
}

impl<T> Drawable for geo::Point<T>
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
        let point = epsg_4326_point_to_pixel_point(snapr, zoom, center, self)?;
        let style = Style::for_point(styles).unwrap_or_default();

        let shape = match &style.representation {
            Representation::Shape(shape) => shape,

            #[cfg(feature = "svg")]
            Representation::Svg(svg) => {
                let svg = svg.try_as_svg((point.x(), point.y()))?;
                svg.draw(snapr, styles, pixmap, center, zoom)?;

                return Ok(());
            }
        };

        let shape = shape.to_path(point.x() as f32, point.y() as f32)?;

        pixmap.fill_path(
            &shape,
            &Paint {
                shader: Shader::SolidColor(style.color_options.foreground),
                anti_alias: style.color_options.anti_alias,
                ..Paint::default()
            },
            FillRule::default(),
            Transform::default(),
            None,
        );

        if let Some(border) = style.color_options.border {
            pixmap.stroke_path(
                &shape,
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

        #[cfg(feature = "svg")]
        if let Some(label) = &style.label {
            let svg = label.try_as_svg((point.x(), point.y()))?;
            svg.draw(snapr, styles, pixmap, center, zoom)?;
        }

        Ok(())
    }
}

impl<T> Drawable for geo::MultiPoint<T>
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
            .try_for_each(|point| point.draw(snapr, styles, pixmap, center, zoom))
    }
}
