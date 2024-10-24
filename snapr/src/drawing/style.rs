//! Contains utilities for styling [`Drawables`](super::Drawable).

use tiny_skia::Color;

use super::{Context, Drawable};

/// Represents a [`Drawable`] that has been _styled_.
pub struct Styled<'a, T: Styleable<S>, S> {
    pub inner: &'a T,
    pub style: S,
}

/// Types that can be converted into a [`Styled`] variant.
pub trait Styleable<S>: Drawable + Sized {
    /// Constructs a [`Styled`] variant of the type using the given `style`.
    fn as_styled(&self, style: S) -> Styled<Self, S> {
        Styled { inner: self, style }
    }
}

/// Function that consumes the current style and returns a new style based on the given parameters.
/// Used by styles to enable more dynamic _effects_ on said styles.
pub type Effect<T, S> = fn(S, &T, &Context) -> S;

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
