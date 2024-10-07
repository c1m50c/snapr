//! Contains utilities and implementations for stylable [`Drawables`](super::Drawable).

use std::borrow::Cow;

use tiny_skia::Color;

pub mod geo;

/// Contains a [`Static`](Style::Static) or [`Dynamic`](Style::Dynamic) style option to be used when _drawing_ [`Drawables`](super::Drawable).
#[derive(Clone, Debug, PartialEq)]
pub enum Style<O, P> {
    /// Represents style options that are static and don't typically change.
    Static(O),

    /// Represents style options that are dynamic and are fetched via a function.
    Dynamic(fn(&P) -> O),
}

impl<O: Default, P> Default for Style<O, P> {
    fn default() -> Self {
        Self::Static(O::default())
    }
}

impl<O: Clone, P> Style<O, P> {
    /// Returns the inner option of the [`Style`].
    #[inline(always)]
    pub fn options<'a>(&'a self, parent: &P) -> Cow<'a, O> {
        match self {
            Self::Static(options) => Cow::Borrowed(options),
            Self::Dynamic(getter) => Cow::Owned(getter(parent)),
        }
    }
}

/// Standard options for coloring [`Drawables`](super::Drawable) found throughout most style options.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorOptions {
    pub foreground: Color,
    pub background: Color,
    pub anti_alias: bool,
    pub border: Option<f32>,
}

impl ColorOptions {
    /// Converts the [`foreground`](Self::foreground) to a color hex code.
    pub fn foreground_as_hex_code(&self) -> String {
        let u8_color = self.foreground.to_color_u8();

        let array = [
            u8_color.red(),
            u8_color.green(),
            u8_color.blue(),
            u8_color.alpha(),
        ];

        format!("#{hex}", hex = hex::encode(array))
    }

    /// Converts the [`background`](Self::background) to a color hex code.
    pub fn background_as_hex_code(&self) -> String {
        let u8_color = self.background.to_color_u8();

        let array = [
            u8_color.red(),
            u8_color.green(),
            u8_color.blue(),
            u8_color.alpha(),
        ];

        format!("#{hex}", hex = hex::encode(array))
    }
}

impl Default for ColorOptions {
    fn default() -> Self {
        Self {
            foreground: Color::from_rgba8(248, 248, 248, 255),
            background: Color::from_rgba8(26, 26, 26, 255),
            anti_alias: true,
            border: Some(1.0),
        }
    }
}