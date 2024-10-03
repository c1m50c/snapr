//! Contains utilities to make rendering SVGs breezy-er.

use resvg::{
    render,
    usvg::{Options, Tree},
};
use tiny_skia::Transform;

use super::{style::ColorOptions, Drawable};

/// Options used when constructing a [`Drawable`] SVG.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SvgOptions {
    pub svg: String,
}

impl SvgOptions {
    /// Attempts to convert the [`SvgOptions`] into a valid [`Svg`].
    pub(crate) fn try_as_svg(&self, pixel: (i32, i32)) -> Result<Svg, crate::Error> {
        let svg = Svg {
            pixel,
            tree: Tree::from_str(&self.svg, &Options::default())?,
        };

        Ok(svg)
    }
}

/// Represents an SVG that's drawn centered on a certain [`pixel`](Self::pixel).
#[derive(Clone, Debug)]
pub(crate) struct Svg {
    pub(crate) pixel: (i32, i32),
    pub(crate) tree: Tree,
}

impl Drawable for Svg {
    fn draw(
        &self,
        _: &crate::Snapper,
        pixmap: &mut tiny_skia::Pixmap,
        _: geo::Point,
    ) -> Result<(), crate::Error> {
        let Svg { pixel, tree } = self;

        let svg_size = tree.size();
        let (x, y) = *pixel;

        render(
            dbg!(tree),
            Transform::from_translate(
                x as f32 - (svg_size.width() / 2.0),
                y as f32 - (svg_size.height() / 2.0),
            ),
            &mut pixmap.as_mut(),
        );

        Ok(())
    }
}

/// Options used when drawing a label.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelOptions {
    pub color_options: ColorOptions,
    pub text: String,
}

impl LabelOptions {
    /// Attempts to convert the [`LabelOptions`] into a valid [`Svg`].
    pub(crate) fn try_as_svg(&self, pixel: (i32, i32)) -> Result<Svg, crate::Error> {
        let raw_svg = format!(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg">
                <text x="0" y="0">{text}</text>
            </svg>
            "##,
            text = self.text
        );

        let svg = Svg {
            pixel,
            tree: Tree::from_str(&raw_svg, &Options::default())?,
        };

        Ok(svg)
    }
}
