use std::ops::{Deref, DerefMut};
use tiny_skia::{Color, FillRule, Paint, Path, PathBuilder, Pixmap, Shader, Stroke, Transform};

use crate::Snapper;

use super::{epsg_4326_point_to_pixel_point, Drawable};

#[derive(Clone, Debug, PartialEq)]
pub struct ColorOptions {
    pub foreground: Color,
    pub background: Color,
    pub anti_alias: bool,
    pub border: Option<f32>,
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

#[derive(Clone, Debug, PartialEq)]
pub enum StyledGeometry<T: geo::CoordNum = f64> {
    Point(StyledPoint<T>),
    Line(StyledLine<T>),
    LineString(StyledLineString<T>),
    Polygon(StyledPolygon<T>),
    MultiPoint(StyledMultiPoint<T>),
    MultiLineString(StyledMultiLineString<T>),
    MultiPolygon(StyledMultiPolygon<T>),
    Rect(StyledRect<T>),
    Triangle(StyledTriangle<T>),
}

impl<T> Drawable for StyledGeometry<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Line(geometry) => geometry.draw(snapper, pixmap, center),
            Self::LineString(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Polygon(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiPoint(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiLineString(geometry) => geometry.draw(snapper, pixmap, center),
            Self::MultiPolygon(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Rect(geometry) => geometry.draw(snapper, pixmap, center),
            Self::Triangle(geometry) => geometry.draw(snapper, pixmap, center),
        }
    }
}

// FIXME: The below `Into` implementation should probably be a `From` implementation.
// We don't currently represent a styled variant of `GeometryCollection`, but we probably should.

#[allow(clippy::from_over_into)]
impl<T: geo::CoordNum> Into<geo::Geometry<T>> for StyledGeometry<T> {
    fn into(self) -> geo::Geometry<T> {
        match self {
            Self::Point(geometry) => geo::Geometry::Point(geometry.0),
            Self::Line(geometry) => geo::Geometry::Line(geometry.0),
            Self::LineString(geometry) => geo::Geometry::LineString(geometry.0),
            Self::Polygon(geometry) => geo::Geometry::Polygon(geometry.0),
            Self::MultiPoint(geometry) => geo::Geometry::MultiPoint(geometry.0),
            Self::MultiLineString(geometry) => geo::Geometry::MultiLineString(geometry.0),
            Self::MultiPolygon(geometry) => geo::Geometry::MultiPolygon(geometry.0),
            Self::Rect(geometry) => geo::Geometry::Rect(geometry.0),
            Self::Triangle(geometry) => geo::Geometry::Triangle(geometry.0),
        }
    }
}

/// Macro for implementing requirements for a styled geometry type.
macro_rules! impl_styled {
    ($base: ident, $styled: ident, $options: ident) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $styled<T: geo::CoordNum = f64>(pub geo::$base<T>, pub $options);

        impl<T: geo::CoordNum> From<geo::$base<T>> for $styled<T> {
            fn from(value: geo::$base<T>) -> Self {
                Self(value, $options::default())
            }
        }

        impl<T: geo::CoordNum> From<geo::$base<T>> for StyledGeometry<T> {
            fn from(value: geo::$base<T>) -> Self {
                Self::$base($styled(value, $options::default()))
            }
        }

        #[allow(clippy::from_over_into)]
        impl<T: geo::CoordNum> Into<StyledGeometry<T>> for $styled<T> {
            fn into(self) -> StyledGeometry<T> {
                StyledGeometry::$base(self)
            }
        }

        impl<T: geo::CoordNum> Deref for $styled<T> {
            type Target = geo::$base<T>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T: geo::CoordNum> DerefMut for $styled<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledPointOptions {
    pub color_options: ColorOptions,
    pub shape: Shape,
}

impl_styled!(Point, StyledPoint, StyledPointOptions);

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledMultiPointOptions {
    pub point_options: StyledPointOptions,
}

impl_styled!(MultiPoint, StyledMultiPoint, StyledMultiPointOptions);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledMultiLineStringOptions {
    pub line_string_options: StyledLineStringOptions,
}

impl_styled!(
    MultiLineString,
    StyledMultiLineString,
    StyledMultiLineStringOptions
);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledMultiPolygonOptions {
    pub polygon_options: StyledPolygonOptions,
}

impl_styled!(MultiPolygon, StyledMultiPolygon, StyledMultiPolygonOptions);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledRectOptions {
    pub polygon_options: StyledPolygonOptions,
}

impl_styled!(Rect, StyledRect, StyledRectOptions);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyledTriangleOptions {
    pub polygon_options: StyledPolygonOptions,
}

impl_styled!(Triangle, StyledTriangle, StyledTriangleOptions);

impl<T> Drawable for StyledPoint<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledPoint(geometry, options) = &self;

        let point = epsg_4326_point_to_pixel_point(snapper, center, geometry)?;
        let shape = options.shape.to_path(point.x() as f32, point.y() as f32)?;

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

        Ok(())
    }
}

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
        let StyledLine(geometry, options) = &self;

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

        StyledPoint(geometry.start_point(), options.start_point_options.clone())
            .draw(snapper, pixmap, center)?;

        StyledPoint(geometry.end_point(), options.end_point_options.clone())
            .draw(snapper, pixmap, center)?;

        Ok(())
    }
}

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
        let StyledLineString(geometry, options) = &self;

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
            StyledPoint(point, options.clone().point_options).draw(snapper, pixmap, center)
        })?;

        Ok(())
    }
}

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
        let StyledPolygon(geometry, options) = &self;

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
            StyledPoint(point, options.point_options.clone()).draw(snapper, pixmap, center)
        })?;

        Ok(())
    }
}

impl<T> Drawable for StyledMultiPoint<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let StyledMultiPoint(geometry, options) = &self;

        for point in geometry.into_iter() {
            let styled = StyledPoint(*point, options.point_options.clone());
            styled.draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}

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
        let StyledMultiLineString(geometry, options) = &self;

        for line_string in geometry.into_iter() {
            let styled = StyledLineString(line_string.clone(), options.line_string_options.clone());
            styled.draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}

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
        let StyledMultiPolygon(geometry, options) = &self;

        for polygon in geometry.into_iter() {
            let styled = StyledPolygon(polygon.clone(), options.polygon_options.clone());
            styled.draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}

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
        let StyledRect(geometry, options) = &self;

        let polygon = StyledPolygon(geometry.to_polygon(), options.polygon_options.clone());
        polygon.draw(snapper, pixmap, center)?;

        Ok(())
    }
}

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
        let StyledTriangle(geometry, options) = &self;

        let polygon = StyledPolygon(geometry.to_polygon(), options.polygon_options.clone());
        polygon.draw(snapper, pixmap, center)?;

        Ok(())
    }
}
