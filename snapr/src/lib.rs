#![doc = include_str!("../README.md")]

use std::{
    f64::consts::PI,
    fmt,
    ops::{Deref, DerefMut},
};

use drawing::{Context, Drawable};
use geo::{BoundingRect, Centroid, Coord, MapCoords};
use image::imageops::overlay;
use thiserror::Error;
use tiny_skia::Pixmap;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use builder::SnaprBuilder;
pub use fetchers::TileFetcher;
pub use {geo, image, tiny_skia};

#[cfg(feature = "tokio")]
pub use fetchers::AsyncTileFetcher;

mod builder;
pub mod drawing;
pub mod fetchers;

#[cfg(feature = "tokio")]
pub mod tokio;

/// Error type used throughout the [`snapr`](crate) crate.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Returned by [`SnaprBuilder`] when attempting to call [`build`](`SnaprBuilder::build()`) on an incomplete builder.
    /// Contains an inner [`reason`](Error::Builder::reason) explaining the specifics of the error.
    #[error("failed to build structure")]
    Builder { reason: String },

    /// Returned by [`Snapr`] when a fetched tile does not match the expected [`tile_size`](Snapr::tile_size).
    #[error("incorrect tile size")]
    IncorrectTileSize { expected: u32, received: u32 },

    #[error("failed to construct path")]
    PathConstruction,

    #[error("failed to construct pixmap")]
    PixmapConstruction,

    #[error("failed to calculate a bounding box for the geometry collection")]
    BoundingBoxCalculation,

    #[error("failed to calculate a centroid for the geometry collection")]
    CentroidCalculation,

    #[cfg(feature = "tokio")]
    #[error("inner panic of spawned asynchronous task")]
    AsynchronousTaskPanic,

    /// Transparent errors returned from [`resvg::usvg`] functions.
    #[cfg(feature = "svg")]
    #[error(transparent)]
    Usvg(#[from] resvg::usvg::Error),

    /// Returned when the source of the error cannot be determined.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Used by [`Snapr`] to determine how the zoom level is calculated when generating snapshots.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Zoom {
    /// Specifies that the zoom level should be automatically derived from the geometry extents.
    /// Contains an inner [`u8`] that controls the max zoom level.
    Automatic(u8),

    /// Specifies that the zoom level should be constant across all snapshots.
    Constant(u8),
}

impl Deref for Zoom {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Automatic(inner) => inner,
            Self::Constant(inner) => inner,
        }
    }
}

impl DerefMut for Zoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Automatic(inner) => inner,
            Self::Constant(inner) => inner,
        }
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom::Automatic(17)
    }
}

impl From<u8> for Zoom {
    fn from(value: u8) -> Self {
        Zoom::Constant(value)
    }
}

/// Utility structure to generate snapshots.
/// Should be normally constructed through building with [`SnaprBuilder`].
pub struct Snapr<'a> {
    /// Function that returns an image of a map tile at specified coordinates.
    /// See [`TileFetcher`] for more details.
    tile_fetcher: TileFetcher<'a>,

    /// Size of the image returned by the [`tile_fetcher`](Self::tile_fetcher).
    tile_size: u32,

    /// Height of generated snapshots.
    height: u32,

    /// Width of generated snapshots.
    width: u32,

    /// Zoom level of generated snapshots.
    zoom: Zoom,
}

impl<'a> Snapr<'a> {
    /// Attempts to generate a snapshot from the [`Drawable`] object.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "DEBUG", skip(self, drawable), err)
    )]
    pub fn snapshot_from_drawable(
        &self,
        drawable: &dyn Drawable,
    ) -> Result<image::RgbaImage, Error> {
        let drawables = vec![drawable];
        self.snapshot_from_drawables(drawables)
    }

    /// Attempts to generate a snapshot from the [`Drawable`] objects.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "DEBUG", skip(self, drawables), err)
    )]
    pub fn snapshot_from_drawables(
        &self,
        drawables: Vec<&dyn Drawable>,
    ) -> Result<image::RgbaImage, Error> {
        let mut output_image = image::RgbaImage::new(self.width, self.height);

        let geometries = drawables
            .iter()
            .flat_map(|drawable| drawable.as_geometry())
            .collect::<Vec<_>>();

        let geometries = geo::GeometryCollection::from(geometries);

        let Some(mut pixmap) = Pixmap::new(self.width, self.height) else {
            return Err(Error::PixmapConstruction);
        };

        let Some(center) = geometries.centroid() else {
            return Err(Error::CentroidCalculation);
        };

        let zoom = match self.zoom {
            Zoom::Constant(level) => level,
            Zoom::Automatic(max_level) => match geometries.bounding_rect() {
                Some(bounding_box) => self.zoom_from_geometries(bounding_box, max_level),
                None => return Err(Error::BoundingBoxCalculation),
            },
        };

        #[cfg(feature = "tracing")]
        {
            tracing::trace!(
                zoom,
                ?center,
                geometries = geometries.len(),
                drawables = drawables.len(),
                "calculated variables required for overlaying and rendering. overlaying backing tiles..."
            );
        }

        self.overlay_backing_tiles(&mut output_image, center, zoom)?;

        drawables
            .iter()
            .enumerate()
            .try_for_each(|(index, drawable)| {
                let context = Context {
                    snapr: self,
                    center,
                    zoom,
                    index,
                };

                #[cfg(feature = "tracing")]
                {
                    tracing::trace!(
                        ?context,
                        "rendering `Drawable` with the `Drawable.draw` method"
                    );
                }

                drawable.draw(&mut pixmap, &context)
            })?;

        #[cfg(feature = "tracing")]
        {
            tracing::trace!("merging the tiles and `Drawables` render images together");
        }

        let pixmap_image = image::ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let pixel = pixmap.pixel(x, y)
                .expect("pixel coordinates should exactly match across `image::ImageBuffer` and `tiny_skia::Pixmap` instances");

            image::Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        });

        overlay(&mut output_image, &pixmap_image, 0, 0);
        Ok(output_image)
    }

    /// Attempts to generate a snapshot from the given [`Geometry`](geo::Geometry).
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "DEBUG", skip(self, geometry), err)
    )]
    pub fn snapshot_from_geometry<G>(&self, geometry: G) -> Result<image::RgbaImage, Error>
    where
        G: Into<geo::Geometry>,
    {
        let geometries = vec![geometry.into()];
        self.snapshot_from_geometries(geometries)
    }

    /// Attempts to generate a snapshot from the given [`Geometries`](geo::Geometry).
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "DEBUG", skip(self), err)
    )]
    pub fn snapshot_from_geometries(
        &self,
        geometries: Vec<geo::Geometry>,
    ) -> Result<image::RgbaImage, Error> {
        let geometries = geometries
            .iter()
            .map(|geometry| geometry as &dyn Drawable)
            .collect();

        self.snapshot_from_drawables(geometries)
    }

    /// Converts a [`EPSG:4326`](https://epsg.io/4326) coordinate to a [`EPSG:3857`](https://epsg.io/3857) reprojection of said coordinate.
    /// Do note, that if you're attempting to use this function to call an XYZ layer you'll need to truncate the given `point` to be [`i32s`](i32).
    pub fn epsg_4326_to_epsg_3857(zoom: u8, point: geo::Point) -> geo::Point {
        let point_as_rad = point.to_radians();
        let n = (1 << zoom as i32) as f64;

        geo::point!(
            x: (n * (point.y() + 180.0) / 360.0),
            y: (n * (1.0 - (point_as_rad.x().tan() + (1.0 / point_as_rad.x().cos())).ln() / PI) / 2.0)
        )
    }
}

impl<'a> Snapr<'a> {
    /// Calculates the [`zoom`](Self::zoom) level to use when [`zoom`](Self::zoom) itself is [`None`].
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "TRACE", skip(self), ret)
    )]
    fn zoom_from_geometries(&self, bounding_box: geo::Rect, max_zoom: u8) -> u8 {
        let mut zoom = 1;

        for level in (0..=max_zoom).rev() {
            let bounding_box = bounding_box.map_coords(|coords| {
                let converted = Self::epsg_4326_to_epsg_3857(level, geo::Point::from(coords));

                Coord {
                    x: converted.x(),
                    y: converted.y(),
                }
            });

            let distance = geo::coord! { x: bounding_box.max().x - bounding_box.min().x, y: bounding_box.min().y - bounding_box.max().y }
                * self.tile_size as f64;

            let dimensions = geo::point!(x: self.width as f64, y: self.height as f64).0;

            if distance.x > dimensions.x || distance.y > dimensions.y {
                continue;
            }

            zoom = level;
            break;
        }

        zoom
    }

    /// Fills the given `image` with tiles centered around the given `epsg_3857_center` point.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "TRACE", skip(self, image), err)
    )]
    fn overlay_backing_tiles(
        &self,
        image: &mut image::RgbaImage,
        center: geo::Point,
        zoom: u8,
    ) -> Result<(), Error> {
        let required_rows = 0.5 * (self.height as f64) / (self.tile_size as f64);
        let required_columns = 0.5 * (self.width as f64) / (self.tile_size as f64);

        let epsg_3857_center = Self::epsg_4326_to_epsg_3857(zoom, center);
        let n = 1 << zoom as i32;

        let min_x = (epsg_3857_center.x() - required_columns).floor() as i32;
        let min_y = (epsg_3857_center.y() - required_rows).floor() as i32;
        let max_x = (epsg_3857_center.x() + required_columns).ceil() as i32;
        let max_y = (epsg_3857_center.y() + required_rows).ceil() as i32;

        #[cfg(feature = "tracing")]
        {
            tracing::trace!(
                required_rows,
                required_columns,
                ?epsg_3857_center,
                min = ?(min_x, min_y),
                max = ?(max_x, max_y),
                "calculated bounds and required variables"
            );
        }

        let coordinate_matrix = (min_x..max_x)
            .map(|x| (x, min_y..max_y))
            .flat_map(|(x, y)| y.map(move |y| (x, y)));

        match self.tile_fetcher {
            TileFetcher::Individual(ref tile_fetcher) => {
                // Capture various fields in `self` to enable `x_y_to_tile` to automatically implement `Sync`
                let (tile_fetcher, tile_size, height, width, zoom) =
                    (tile_fetcher, self.tile_size, self.height, self.width, zoom);

                let x_y_to_tile =
                    |(x, y): (i32, i32)| -> Result<(image::RgbaImage, i64, i64), Error> {
                        let tile = tile_fetcher
                            .fetch_tile((x + n) % n, (y + n) % n, zoom)?
                            .to_rgba8();

                        let tile_coords = (geo::Point::from((x as f64, y as f64))
                            - epsg_3857_center)
                            .map_coords(|coord| geo::Coord {
                                x: coord.x * tile_size as f64 + width as f64 / 2.0,
                                y: coord.y * tile_size as f64 + height as f64 / 2.0,
                            });

                        Ok((tile, tile_coords.x() as i64, tile_coords.y() as i64))
                    };

                #[cfg(feature = "rayon")]
                {
                    #[cfg(feature = "tracing")]
                    {
                        tracing::trace!(
                            "executing `TileFetcher::Individual` in parallel with `rayon` crate"
                        );
                    }

                    coordinate_matrix
                        .par_bridge()
                        .flat_map(x_y_to_tile)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .for_each(|(tile, x, y)| overlay(image, &tile, x, y));
                }

                #[cfg(not(feature = "rayon"))]
                {
                    #[cfg(feature = "tracing")]
                    {
                        tracing::trace!("executing `TileFetcher::Individual` sequentially");
                    }

                    for (x, y) in coordinate_matrix {
                        let (tile, x, y) = x_y_to_tile((x, y))?;
                        overlay(image, &tile, x, y);
                    }
                }
            }

            TileFetcher::Batch(ref tile_fetcher) => {
                let coordinate_matrix = coordinate_matrix.collect::<Vec<_>>();

                #[cfg(feature = "tracing")]
                {
                    tracing::trace!("executing `TileFetcher::Batch`");
                }

                for (x, y, tile) in tile_fetcher.fetch_tiles(&coordinate_matrix, zoom)? {
                    let tile_coords = (geo::Point::from((x as f64, y as f64)) - epsg_3857_center)
                        .map_coords(|coord| geo::Coord {
                            x: coord.x * self.tile_size as f64 + self.width as f64 / 2.0,
                            y: coord.y * self.tile_size as f64 + self.height as f64 / 2.0,
                        });

                    overlay(image, &tile, tile_coords.x() as i64, tile_coords.y() as i64);
                }
            }
        }

        Ok(())
    }
}

impl<'a> fmt::Debug for Snapr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Snapr")
            .field("tile_size", &self.tile_size)
            .field("height", &self.height)
            .field("width", &self.width)
            .field("zoom", &self.zoom)
            .finish()
    }
}
