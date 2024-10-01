use std::ops::{Deref, DerefMut};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Shader, Stroke, Transform};

use super::{epsg_4326_point_to_pixel_point, Drawable};

#[derive(Clone, Debug, PartialEq)]
pub struct ColorOptions {
    pub foreground: Color,
    pub background: Color,
    pub anti_alias: bool,
    pub bordered: bool,
}

impl Default for ColorOptions {
    fn default() -> Self {
        Self {
            foreground: Color::from_rgba8(248, 248, 248, 255),
            background: Color::from_rgba8(26, 26, 26, 255),
            anti_alias: true,
            bordered: true,
        }
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

// FIXME: The below `Into` implementation should probably be a `From` implementation.
// We don't currently represent a styled variant of `GeometryCollection`, but we probably should.

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

macro_rules! impl_styled {
    ($base: ident, $styled: ident) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $styled<T: geo::CoordNum = f64>(pub geo::$base<T>, pub ColorOptions);

        impl<T: geo::CoordNum> From<geo::$base<T>> for $styled<T> {
            fn from(value: geo::$base<T>) -> Self {
                Self(value, ColorOptions::default())
            }
        }

        impl<T: geo::CoordNum> From<geo::$base<T>> for StyledGeometry<T> {
            fn from(value: geo::$base<T>) -> Self {
                Self::$base($styled(value, ColorOptions::default()))
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

impl_styled!(Point, StyledPoint);
impl_styled!(Line, StyledLine);
impl_styled!(LineString, StyledLineString);
impl_styled!(Polygon, StyledPolygon);
impl_styled!(MultiPoint, StyledMultiPoint);
impl_styled!(MultiLineString, StyledMultiLineString);
impl_styled!(MultiPolygon, StyledMultiPolygon);
impl_styled!(Rect, StyledRect);
impl_styled!(Triangle, StyledTriangle);

impl<T> Drawable for StyledPoint<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &crate::Snapper,
        pixmap: &mut tiny_skia::Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let point = epsg_4326_point_to_pixel_point(snapper, center, self)?;

        let mut path_builder = PathBuilder::new();
        path_builder.push_circle(point.x() as f32, point.y() as f32, 4.0);

        if let Some(circle) = path_builder.finish() {
            pixmap.fill_path(
                &circle,
                &Paint {
                    shader: Shader::SolidColor(self.1.foreground),
                    anti_alias: self.1.anti_alias,
                    ..Paint::default()
                },
                FillRule::default(),
                Transform::default(),
                None,
            );

            if self.1.bordered {
                pixmap.stroke_path(
                    &circle,
                    &Paint {
                        shader: Shader::SolidColor(self.1.background),
                        anti_alias: self.1.anti_alias,
                        ..Paint::default()
                    },
                    &Stroke {
                        width: 1.0,
                        ..Stroke::default()
                    },
                    Transform::default(),
                    None,
                );
            }
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
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl<T> Drawable for StyledLineString<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        snapper: &crate::Snapper,
        pixmap: &mut tiny_skia::Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error> {
        let converted_points = self
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
            if self.1.bordered {
                pixmap.stroke_path(
                    &lines,
                    &Paint {
                        shader: Shader::SolidColor(self.1.background),
                        anti_alias: self.1.anti_alias,
                        ..Paint::default()
                    },
                    &Stroke {
                        width: 3.0,
                        ..Stroke::default()
                    },
                    Transform::default(),
                    None,
                );
            }

            pixmap.stroke_path(
                &lines,
                &Paint {
                    shader: Shader::SolidColor(self.1.foreground),
                    anti_alias: self.1.anti_alias,
                    ..Paint::default()
                },
                &Stroke {
                    width: 2.0,
                    ..Stroke::default()
                },
                Transform::default(),
                None,
            );
        }

        for point in self.points() {
            StyledPoint::from(point).draw(snapper, pixmap, center)?;
        }

        Ok(())
    }
}

impl<T> Drawable for StyledPolygon<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl<T> Drawable for StyledMultiPoint<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl<T> Drawable for StyledMultiLineString<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl<T> Drawable for StyledMultiPolygon<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl<T> Drawable for StyledRect<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}

impl<T> Drawable for StyledTriangle<T>
where
    T: geo::CoordNum,
{
    fn draw(
        &self,
        _snapper: &crate::Snapper,
        _pixmap: &mut tiny_skia::Pixmap,
        _center: geo::Point,
    ) -> Result<(), crate::Error> {
        unimplemented!()
    }
}
