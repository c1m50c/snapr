//! Contains [`Drawable`](super::Drawable) implementations and [`Styles`](super::style::Style) for [`geo`] primitives.

use tiny_skia::Pixmap;

use super::{Context, Drawable};

pub mod line;
pub mod point;
pub mod polygon;

impl Drawable for geo::Geometry<f64> {
    fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
        match self {
            Self::Point(geometry) => geometry.draw(pixmap, context),
            Self::Line(geometry) => geometry.draw(pixmap, context),
            Self::LineString(geometry) => geometry.draw(pixmap, context),
            Self::Polygon(geometry) => geometry.draw(pixmap, context),
            Self::MultiPoint(geometry) => geometry.draw(pixmap, context),
            Self::MultiLineString(geometry) => geometry.draw(pixmap, context),
            Self::MultiPolygon(geometry) => geometry.draw(pixmap, context),
            Self::Rect(geometry) => geometry.draw(pixmap, context),
            Self::Triangle(geometry) => geometry.draw(pixmap, context),

            Self::GeometryCollection(geometry) => geometry
                .into_iter()
                .try_for_each(|geometry| geometry.draw(pixmap, context)),
        }
    }

    fn as_geometry(&self) -> Option<geo::Geometry<f64>> {
        Some(self.clone())
    }
}

pub(crate) mod macros {
    macro_rules! impl_styled_geo {
        ($type: ident, $style: ident, $draw: item) => {
            impl<F> Styleable<F> for geo::$type<f64>
            where F: Fn(&geo::$type<f64>, &Context) -> $style {  }

            impl Styleable<$style> for geo::$type<f64> {}

            impl<F> Drawable for Styled<'_, geo::$type<f64>, F>
            where
                F: Fn(&geo::$type<f64>, &Context) -> $style
            {
                fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
                    let style = (self.style)(self.inner, context);
                    self.inner.as_styled(style).draw(pixmap, context)
                }

                fn as_geometry(&self) -> Option<geo::Geometry<f64>> {
                    Some(self.inner.clone().into())
                }
            }

            impl Drawable for Styled<'_, geo::$type<f64>, $style> {
                $draw

                fn as_geometry(&self) -> Option<geo::Geometry<f64>> {
                    Some(self.inner.clone().into())
                }
            }

            impl Drawable for geo::$type<f64> {
                fn draw(&self, pixmap: &mut Pixmap, context: &Context) -> Result<(), crate::Error> {
                    self.as_styled($style::default())
                        .draw(pixmap, context)
                }

                fn as_geometry(&self) -> Option<geo::Geometry<f64>> {
                    Some(self.clone().into())
                }
            }
        };
    }

    pub(crate) use impl_styled_geo;
}
