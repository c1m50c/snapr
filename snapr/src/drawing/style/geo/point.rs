//! Stylable wrappers of [`geo::Point`], and [`geo::MultiPoint`].

use tiny_skia::{FillRule, Paint, Pixmap, Shader, Stroke, Transform};

use crate::{
    drawing::{
        epsg_4326_point_to_pixel_point,
        style::{ColorOptions, Style},
        svg::{LabelOptions, SvgOptions},
        Drawable,
    },
    Snapr,
};

use super::{macros::impl_styled, Shape};

/// Options for how a [`StyledPoint`] should be _visually_ represented.
#[derive(Clone, Debug, PartialEq)]
pub enum Representation {
    #[cfg(feature = "svg")]
    Svg(SvgOptions),

    Shape(Shape),
}

impl Default for Representation {
    fn default() -> Self {
        Self::Shape(Shape::default())
    }
}

/// Style options for [`StyledPoint`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledPointOptions {
    pub color_options: ColorOptions,
    pub representation: Representation,

    #[cfg(feature = "svg")]
    pub label_options: Option<LabelOptions>,
}

impl_styled!(Point, StyledPoint, StyledPointOptions);

impl<T> Drawable for StyledPoint<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapr: &Snapr,
        pixmap: &mut Pixmap,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), crate::Error> {
        let StyledPoint(geometry, style) = &self;
        let options = style.options(self);

        let point = epsg_4326_point_to_pixel_point(snapr, zoom, center, geometry)?;

        let shape = match &options.representation {
            Representation::Shape(shape) => shape,

            #[cfg(feature = "svg")]
            Representation::Svg(svg) => {
                let svg = svg.try_as_svg((point.x(), point.y()))?;
                svg.draw(snapr, pixmap, center, zoom)?;

                return Ok(());
            }
        };

        let shape = shape.to_path(point.x() as f32, point.y() as f32)?;

        pixmap.fill_path(
            &shape,
            &Paint {
                shader: Shader::SolidColor(options.color_options.foreground),
                anti_alias: options.color_options.anti_alias,
                ..Paint::default()
            },
            FillRule::default(),
            Transform::default(),
            None,
        );

        if let Some(border) = options.color_options.border {
            pixmap.stroke_path(
                &shape,
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

        #[cfg(feature = "svg")]
        if let Some(label_options) = &options.label_options {
            let svg = label_options.try_as_svg((point.x(), point.y()))?;
            svg.draw(snapr, pixmap, center, zoom)?;
        }

        Ok(())
    }
}

/// Style options for [`StyledMultiPoint`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledMultiPointOptions {
    pub point_options: StyledPointOptions,
}

impl_styled!(MultiPoint, StyledMultiPoint, StyledMultiPointOptions);

impl<T> Drawable for StyledMultiPoint<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapr: &Snapr,
        pixmap: &mut Pixmap,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), crate::Error> {
        let StyledMultiPoint(geometry, style) = &self;
        let options = style.options(self);

        for point in geometry.into_iter() {
            let styled = StyledPoint(*point, Style::Static(options.point_options.clone()));
            styled.draw(snapr, pixmap, center, zoom)?;
        }

        Ok(())
    }
}
