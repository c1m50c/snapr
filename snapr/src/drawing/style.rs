//! Contains utilities for styling [`Drawables`](super::Drawable).

use tiny_skia::Color;

use super::{
    geometry::{line::LineStyle, point::PointStyle, polygon::PolygonStyle},
    DrawingState,
};

/// Represents _styles_ that can be applied to [`Drawable`](super::Drawable) objects.
pub enum Style<'a> {
    Point(PointStyle),
    Line(LineStyle),
    Polygon(PolygonStyle),
    Dynamic(&'a dyn DynamicStyle),
}

impl<'a> Style<'a> {
    /// Attempts to convert the given [`Iterator`] of [`Styles`](Style) to a singular [`PointStyle`].
    pub fn for_point<I>(
        styles: I,
        state: &DrawingState,
        point: &geo::Point<i32>,
    ) -> Option<PointStyle>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let styles = styles.into_iter().flat_map(|style| match style {
            Self::Dynamic(style) => style.for_point(state, point),
            Self::Point(style) => Some(style).cloned(),
            _ => None,
        });

        styles.reduce(Self::merge_point_styles)
    }

    /// Attempts to convert the given [`Iterator`] of [`Styles`](Style) to a singular [`LineStyle`].
    pub fn for_line<I>(
        styles: I,
        state: &DrawingState,
        line_string: &geo::LineString<i32>,
    ) -> Option<LineStyle>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let styles = styles.into_iter().flat_map(|style| match style {
            Self::Dynamic(style) => style.for_line(state, line_string),
            Self::Line(style) => Some(style).cloned(),
            _ => None,
        });

        styles.reduce(Self::merge_line_styles)
    }

    /// Attempts to convert the given [`Iterator`] of [`Styles`](Style) to a singular [`PolygonStyle`].
    pub fn for_polygon<I>(
        styles: I,
        state: &DrawingState,
        polygon: &geo::Polygon<i32>,
    ) -> Option<PolygonStyle>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        let styles = styles.into_iter().flat_map(|style| match style {
            Self::Dynamic(style) => style.for_polygon(state, polygon),
            Self::Polygon(style) => Some(style).cloned(),
            _ => None,
        });

        styles.reduce(Self::merge_polygon_styles)
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

impl<'a> From<PointStyle> for Style<'a> {
    fn from(value: PointStyle) -> Self {
        Self::Point(value)
    }
}

impl<'a> From<LineStyle> for Style<'a> {
    fn from(value: LineStyle) -> Self {
        Self::Line(value)
    }
}

impl<'a> From<PolygonStyle> for Style<'a> {
    fn from(value: PolygonStyle) -> Self {
        Self::Polygon(value)
    }
}

impl<'a, T> From<&'a T> for Style<'a>
where
    T: DynamicStyle,
{
    fn from(value: &'a T) -> Self {
        Self::Dynamic(value)
    }
}

#[allow(unused_variables)]
pub trait DynamicStyle {
    /// Creates a [`PointStyle`] from a given `point`.
    fn for_point(&self, state: &DrawingState, point: &geo::Point<i32>) -> Option<PointStyle> {
        None
    }

    /// Creates a [`LineStyle`] from a given `line`.
    fn for_line(
        &self,
        state: &DrawingState,
        line_string: &geo::LineString<i32>,
    ) -> Option<LineStyle> {
        None
    }

    /// Creates a [`PolygonStyle`] from a given `polygon`.
    fn for_polygon(
        &self,
        state: &DrawingState,
        polygon: &geo::Polygon<i32>,
    ) -> Option<PolygonStyle> {
        None
    }
}

impl DynamicStyle for fn(&DrawingState, &geo::Point<i32>) -> Option<PointStyle> {
    fn for_point(&self, state: &DrawingState, point: &geo::Point<i32>) -> Option<PointStyle> {
        (self)(state, point)
    }
}

impl DynamicStyle for fn(&DrawingState, &geo::LineString<i32>) -> Option<LineStyle> {
    fn for_line(
        &self,
        state: &DrawingState,
        line_string: &geo::LineString<i32>,
    ) -> Option<LineStyle> {
        (self)(state, line_string)
    }
}

impl DynamicStyle for fn(&DrawingState, &geo::Polygon<i32>) -> Option<PolygonStyle> {
    fn for_polygon(
        &self,
        state: &DrawingState,
        polygon: &geo::Polygon<i32>,
    ) -> Option<PolygonStyle> {
        (self)(state, polygon)
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
