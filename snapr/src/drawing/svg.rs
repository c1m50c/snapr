//! Contains utilities to make rendering SVGs breezy-er.

use resvg::{
    render,
    usvg::{Options, Tree},
};
use tiny_skia::{Pixmap, Transform};

use crate::Snapr;

use super::{
    style::{ColorOptions, Style},
    Drawable,
};

/// Configuration structure used to generate a [`Drawable`] SVG.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Svg {
    pub offset: (i32, i32),
    pub svg: String,
}

impl Svg {
    /// Attempts to convert the [`SvgOptions`] into a valid [`Svg`].
    pub(crate) fn try_as_svg(&self, pixel: (i32, i32)) -> Result<SpatialSvg, crate::Error> {
        let mut options = Options::default();
        options.fontdb_mut().load_system_fonts();

        let svg = SpatialSvg {
            pixel: (pixel.0 - self.offset.0, pixel.1 - self.offset.1),
            tree: Tree::from_str(&self.svg, &options)?,
        };

        Ok(svg)
    }
}

/// Configuration structure used to generate a [`Drawable`] label.
#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    pub color_options: ColorOptions,
    pub font_family: String,
    pub font_size: f32,
    pub offset: (i32, i32),
    pub text: String,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            color_options: ColorOptions::default(),
            font_family: "Arial".to_string(),
            font_size: 16.0,
            offset: (0, 12),
            text: String::default(),
        }
    }
}

impl Label {
    /// Attempts to convert the [`LabelStyle`] into a valid [`Svg`].
    pub(crate) fn try_as_svg(&self, pixel: (i32, i32)) -> Result<SpatialSvg, crate::Error> {
        let raw_svg = format!(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg">
                <text style="fill: {foreground}; font-family: '{font_family}'; font-size: {font_size}px; paint-order: stroke; stroke: {background}; stroke-width: {border}px;">{text}</text>
            </svg>
            "##,
            foreground = self.color_options.foreground_as_hex_code(),
            font_family = self.font_family,
            font_size = self.font_size,
            background = self.color_options.background_as_hex_code(),
            border = self.color_options.border.unwrap_or(0.0),
            text = self.text,
        );

        let mut options = Options::default();
        options.fontdb_mut().load_system_fonts();

        let svg = SpatialSvg {
            pixel: (pixel.0 - self.offset.0, pixel.1 - self.offset.1),
            tree: Tree::from_str(&raw_svg, &options)?,
        };

        Ok(svg)
    }
}

/// Represents an SVG that's drawn centered on a certain [`pixel`](Self::pixel).
#[derive(Clone, Debug)]
pub(crate) struct SpatialSvg {
    pub(crate) pixel: (i32, i32),
    pub(crate) tree: Tree,
}

impl Drawable for SpatialSvg {
    fn draw(
        &self,
        _: &Snapr,
        _: &[Style],
        pixmap: &mut Pixmap,
        _: geo::Point,
        _: u8,
    ) -> Result<(), crate::Error> {
        let SpatialSvg { pixel, tree } = self;

        let svg_size = tree.size();
        let (x, y) = *pixel;

        render(
            tree,
            Transform::from_translate(
                x as f32 - (svg_size.width() / 2.0),
                y as f32 - (svg_size.height() / 2.0),
            ),
            &mut pixmap.as_mut(),
        );

        Ok(())
    }
}
