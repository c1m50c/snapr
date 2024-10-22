//! Contains utilities for styling [`Drawables`](super::Drawable).

use tiny_skia::Color;

use super::geometry::{line::LineStyle, point::PointStyle, polygon::PolygonStyle};

/// Represents _styles_ that can be applied to [`Drawable`](super::Drawable) objects.
#[derive(Clone, Debug, PartialEq)]
pub enum Style {
    Point(PointStyle),
    Line(LineStyle),
    Polygon(PolygonStyle),
}

impl Style {
    /// Attempts to convert the given [`Iterator`] of [`Styles`](Style) to a singular [`PointStyle`].
    pub fn for_point<'a, I>(styles: I) -> Option<PointStyle>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let styles = styles.into_iter().flat_map(|style| match style {
            Self::Point(style) => Some(style),
            _ => None,
        });

        styles.cloned().reduce(Self::merge_point_styles)
    }

    /// Attempts to convert the given [`Iterator`] of [`Styles`](Style) to a singular [`LineStyle`].
    pub fn for_line<'a, I>(styles: I) -> Option<LineStyle>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let styles = styles.into_iter().flat_map(|style| match style {
            Self::Line(style) => Some(style),
            _ => None,
        });

        styles.cloned().reduce(Self::merge_line_styles)
    }

    /// Attempts to convert the given [`Iterator`] of [`Styles`](Style) to a singular [`PolygonStyle`].
    pub fn for_polygon<'a, I>(styles: I) -> Option<PolygonStyle>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let styles = styles.into_iter().flat_map(|style| match style {
            Self::Polygon(style) => Some(style),
            _ => None,
        });

        styles.cloned().reduce(Self::merge_polygon_styles)
    }

    /// Merges two [`PointStyle`]s into one with the fields in `b` taking priority over the fields in `a`.
    #[inline(always)]
    pub fn merge_point_styles(a: PointStyle, b: PointStyle) -> PointStyle {
        PointStyle {
            color_options: ColorOptions {
                foreground: b.color_options.foreground,
                background: b.color_options.background,
                anti_alias: b.color_options.anti_alias,
                border: b.color_options.border.or(a.color_options.border),
            },
            representation: b.representation,
            label: b.label.or(a.label),
        }
    }

    /// Merges two [`LineStyle`]s into one with the fields in `b` taking priority over the fields in `a`.
    #[inline(always)]
    pub fn merge_line_styles(a: LineStyle, b: LineStyle) -> LineStyle {
        LineStyle {
            color_options: ColorOptions {
                foreground: b.color_options.foreground,
                background: b.color_options.background,
                anti_alias: b.color_options.anti_alias,
                border: b.color_options.border.or(a.color_options.border),
            },
            width: b.width,
        }
    }

    /// Merges two [`PolygonStyle`]s into one with the fields in `b` taking priority over the fields in `a`.
    #[inline(always)]
    pub fn merge_polygon_styles(a: PolygonStyle, b: PolygonStyle) -> PolygonStyle {
        PolygonStyle {
            color_options: ColorOptions {
                foreground: b.color_options.foreground,
                background: b.color_options.background,
                anti_alias: b.color_options.anti_alias,
                border: b.color_options.border.or(a.color_options.border),
            },
        }
    }
}

impl From<PointStyle> for Style {
    fn from(value: PointStyle) -> Self {
        Self::Point(value)
    }
}

impl From<LineStyle> for Style {
    fn from(value: LineStyle) -> Self {
        Self::Line(value)
    }
}

impl From<PolygonStyle> for Style {
    fn from(value: PolygonStyle) -> Self {
        Self::Polygon(value)
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
