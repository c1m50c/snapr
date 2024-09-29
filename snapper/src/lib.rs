use std::f64::consts::PI;

use drawing::DrawableGeometry;
use geo::{Centroid, MapCoords};
use image::imageops::overlay;
use thiserror::Error;

pub use builder::SnapperBuilder;

mod builder;
mod drawing;

/// Error type used throughout the [`snapper`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Returned by [`SnapperBuilder`] when attempting to call [`build`](`SnapperBuilder::build()`) on an incomplete builder.
    /// Contains an inner [`reason`](Error::Builder::reason) explaining the specifics of the error.
    #[error("failed to build structure")]
    Builder {
        reason: String,
    },

    /// Returned by [`Snapper`] when a fetched tile does not match the expected [`tile_size`](Snapper::tile_size).
    #[error("incorrect tile size")]
    IncorrectTileSize {
        expected: u32,
        received: u32,
    },

    #[error("failed to convert between primitive numbers")]
    PrimitiveNumberConversion,

    /// Returned when the source of the error cannot be determined.
    #[error(transparent)]
    Unknown {
        #[from]
        source: anyhow::Error,
    },
}

/// Function that takes coordinates and a zoom level as arguments and returns an [`Image`](image::DynamicImage) of the map tile at the given position.
/// 
/// ## Example
/// 
/// ```rust
/// use image::DynamicImage;
/// 
/// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
///     todo!()
/// }
/// ```
pub type TileFetcher = fn(i32, i32, u8) -> Result<image::DynamicImage, Error>;

/// Utility structure to generate snapshots.
/// Should be normally constructed through building with [`SnapperBuilder`].
#[derive(Debug)]
pub struct Snapper {
    /// Function that returns an image of a map tile at specified coordinates.
    tile_fetcher: TileFetcher,

    /// Size of the image returned by the [`tile_fetcher`](Self::tile_fetcher).
    tile_size: u32,

    /// Height of generated snapshots.
    height: u32,

    /// Width of generated snapshots.
    width: u32,

    /// Zoom level of generated snapshots.
    zoom: u8,
}

impl Snapper {
    /// Returns a snapshot centered around the provided `geometry`.
    pub fn generate_snapshot_from_geometry(&self, geometry: geo::Geometry) -> Result<image::RgbaImage, Error> {
        let geometries = geo::GeometryCollection::from(geometry);
        self.generate_snapshot_from_geometries(geometries)
    }

    /// Returns a snapshot centered around the provided `geometries`.
    pub fn generate_snapshot_from_geometries(&self, geometries: geo::GeometryCollection) -> Result<image::RgbaImage, Error> {
        let mut output_image = image::RgbaImage::new(self.width, self.height);

        let Some(geometry_center_point) = geometries.centroid() else {
            todo!()
        };

        let reprojected_center = self.point_to_epsg_3857(geometry_center_point);
        self.overlay_backing_tiles(&mut output_image, reprojected_center)?;

        geometries.into_iter()
            .try_for_each(|geometry| geometry.draw(self, &mut output_image, geometry_center_point))?;

        Ok(output_image)
    }
}

impl Snapper {
    pub(crate) fn point_to_epsg_3857(&self, point: geo::Point) -> geo::Point<i32> {
        let point_as_rad = point.to_radians();
        let n = (1 << self.zoom as i32) as f64;

        geo::point!(
            x: (n * (point.y() + 180.0) / 360.0) as i32,
            y: (n * (1.0 - (point_as_rad.x().tan() + (1.0 / point_as_rad.x().cos())).ln() / PI) / 2.0) as i32
        )
    }
    
    pub(crate) fn latitude_to_pixel(&self, center: geo::Point, latitude: f64) -> f64 {
        (latitude - center.x()) * self.tile_size as f64 + self.width as f64 / 2.0
    }

    pub(crate) fn longitude_to_pixel(&self, center: geo::Point, longitude: f64) -> f64 {
        (longitude - center.y()) * self.tile_size as f64 + self.height as f64 / 2.0
    }

    /// Calls the [`tile_fetcher`](Self::tile_fetcher) function with the given coordinates and converts the returned [`image::DynamicImage`] into an [`image::RgbaImage`].
    #[inline(always)]
    fn get_tile(&self, x: i32, y: i32) -> Result<image::RgbaImage, Error> {
        let tile = (self.tile_fetcher)(x, y, self.zoom)?.to_rgba8();

        if tile.height() != self.tile_size {
            return Err(Error::IncorrectTileSize {
                expected: self.tile_size,
                received: tile.height()
            });
        }

        if tile.width() != self.tile_size {
            return Err(Error::IncorrectTileSize {
                expected: self.tile_size,
                received: tile.height()
            });
        }

        Ok(tile)
    }

    /// Fills the given `image` with tiles centered around the given `epsg_3857_center` point.
    fn overlay_backing_tiles(&self, image: &mut image::RgbaImage, epsg_3857_center: geo::Point<i32>) -> Result<(), Error> {
        let required_rows = 0.5 * (self.height as f64) / (self.tile_size as f64);
        let required_columns = 0.5 * (self.width as f64) / (self.tile_size as f64);

        // FIXME: The overlay is not properly centered after being generated.
        // This is due to us only caring about the center *tile* and not the exact center position when orienting things.
        // We should account for the exact (floating point) position when centering the snapshot.

        let min_x = (epsg_3857_center.x() as f64 - required_columns).floor() as i32;
        let min_y = (epsg_3857_center.y() as f64 - required_rows).floor() as i32;
        let max_x = (epsg_3857_center.x() as f64 + required_columns).ceil() as i32;
        let max_y = (epsg_3857_center.y() as f64 + required_rows).ceil() as i32;
        let n = 1 << self.zoom as i32;

        let center_as_f64 = epsg_3857_center.map_coords(|coords| {
            geo::Coord { x: coords.x as f64, y: coords.y as f64 }
        });

        for x in min_x..max_x {
            for y in min_y..max_y {
                let tile = self.get_tile(
                    (x as i32 + n) % n,
                    (y as i32 + n) % n,
                )?;

                overlay(
                    image,
                    &tile,
                    self.latitude_to_pixel(center_as_f64, x as f64) as i64,
                    self.longitude_to_pixel(center_as_f64, y as f64) as i64,
                );
            }
        }

        Ok(())
    }
}