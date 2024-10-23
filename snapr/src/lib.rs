#![doc = include_str!("../README.md")]

use std::{f64::consts::PI, fmt};

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

mod builder;
pub mod drawing;
pub mod fetchers;

/// Error type used throughout the [`snapr`](crate) crate.
#[derive(Debug, Error)]
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

    /// Transparent errors returned from [`resvg::usvg`] functions.
    #[error(transparent)]
    #[cfg(feature = "svg")]
    Usvg(#[from] resvg::usvg::Error),

    /// Returned when the source of the error cannot be determined.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
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
    zoom: Option<u8>,

    /// Maximum zoom level of generated snapshots.
    max_zoom: u8,
}

impl<'a> Snapr<'a> {
    /// Returns a snapshot centered around the provided `geometry`.
    pub fn generate_snapshot_from_geometry<G>(&self, geometry: G) -> Result<image::RgbaImage, Error>
    where
        G: Into<geo::Geometry>,
    {
        let geometries = vec![geometry.into()];
        self.generate_snapshot_from_geometries(geometries)
    }

    /// Returns a snapshot centered around the provided `geometries`.
    pub fn generate_snapshot_from_geometries(
        &self,
        geometries: Vec<geo::Geometry>,
    ) -> Result<image::RgbaImage, Error> {
        let geometries = geometries
            .iter()
            .map(|geometry| geometry as &dyn Drawable)
            .collect();

        self.generate_snapshot(geometries)
    }

    /// Returns a snapshot rendering the provided `drawables`.
    pub fn generate_snapshot(
        &self,
        drawables: Vec<&'_ dyn Drawable>,
    ) -> Result<image::RgbaImage, Error> {
        let mut output_image = image::RgbaImage::new(self.width, self.height);

        let geometries = drawables
            .iter()
            .flat_map(|drawable| drawable.as_geometry())
            .collect::<Vec<_>>();

        let geometries = geo::GeometryCollection::from(geometries);

        let Some(mut pixmap) = Pixmap::new(self.width, self.height) else {
            todo!("Return an `Err` or find some way to safely go forward with the function")
        };

        let Some(center) = geometries.centroid() else {
            todo!("Return an `Err` or find a suitable default for `center`")
        };

        let zoom = match self.zoom {
            Some(zoom) => zoom.clamp(1, self.max_zoom),
            None => match geometries.bounding_rect() {
                Some(bounding_box) => self.zoom_from_geometries(bounding_box),
                None => todo!("Return an `Err` or find a suitable default for `bounding_box`"),
            },
        };

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

                drawable.draw(&mut pixmap, &context)
            })?;

        let pixmap_image = image::ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let pixel = pixmap.pixel(x, y)
                .expect("pixel coordinates should exactly match across `image::ImageBuffer` and `tiny_skia::Pixmap` instances");

            image::Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        });

        overlay(&mut output_image, &pixmap_image, 0, 0);
        Ok(output_image)
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
    fn zoom_from_geometries(&self, bounding_box: geo::Rect) -> u8 {
        let mut zoom = 1;

        for level in (0..=self.max_zoom).rev() {
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

        match self.tile_fetcher {
            TileFetcher::Individual(tile_fetcher) => {
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
                    let matrix_iter = (min_x..max_x)
                        .map(|x| (x, min_y..max_y))
                        .flat_map(|(x, y)| y.map(move |y| (x, y)));

                    let tiles = matrix_iter
                        .par_bridge()
                        .flat_map(x_y_to_tile)
                        .collect::<Vec<_>>();

                    tiles
                        .into_iter()
                        .for_each(|(tile, x, y)| overlay(image, &tile, x, y));
                }

                #[cfg(not(feature = "rayon"))]
                {
                    for x in min_x..max_x {
                        for y in min_y..max_y {
                            let (tile, x, y) = x_y_to_tile((x, y))?;
                            overlay(image, &tile, x, y);
                        }
                    }
                }
            }

            TileFetcher::Batch(tile_fetcher) => {
                let matrix = (min_x..max_x)
                    .map(|x| (x, min_y..max_y))
                    .flat_map(|(x, y)| y.map(move |y| (x, y)))
                    .collect::<Vec<_>>();

                let batches = tile_fetcher.fetch_tiles(&matrix, zoom)?;

                for (x, y, tile) in batches {
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
