use std::f64::consts::PI;

use geo::{Centroid, MapCoords};
use image::imageops::overlay;
use thiserror::Error;

pub use builder::SnapperBuilder;

mod builder;

#[cfg(feature = "drawing")]
mod drawing;

/// Error type used throughout the [`snapper`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Returned by [`SnapperBuilder`] when attempting to call [`build`](`SnapperBuilder::build()`) on an incomplete builder.
    /// Contains an inner [`reason`](Error::Builder::reason) explaining the specifics of the error.
    #[error("failed to build structure")]
    Builder { reason: String },

    /// Returned by [`Snapper`] when a fetched tile does not match the expected [`tile_size`](Snapper::tile_size).
    #[error("incorrect tile size")]
    IncorrectTileSize { expected: u32, received: u32 },

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
    #[allow(dead_code)]
    tile_fetcher: TileFetcher,

    /// Size of the image returned by the [`tile_fetcher`](Self::tile_fetcher).
    #[allow(dead_code)]
    tile_size: u32,

    /// Height of generated snapshots.
    height: u32,

    /// Width of generated snapshots.
    width: u32,

    /// Zoom level of generated snapshots.
    #[allow(dead_code)]
    zoom: u8,
}

impl Snapper {
    /// Returns a snapshot centered around the provided `geometry`.
    #[cfg(feature = "drawing")]
    pub fn generate_snapshot_from_geometry(
        &self,
        geometry: geo::Geometry,
        style: Option<drawing::Style>,
    ) -> Result<image::RgbaImage, Error> {
        let geometries = geo::GeometryCollection::from(geometry);
        self.generate_snapshot_from_geometries(geometries, style)
    }

    /// Returns a snapshot centered around the provided `geometries`.
    #[cfg(feature = "drawing")]
    pub fn generate_snapshot_from_geometries(
        &self,
        geometries: geo::GeometryCollection,
        style: Option<drawing::Style>,
    ) -> Result<image::RgbaImage, Error> {
        use drawing::DrawableGeometry;

        let style = style.unwrap_or_default();

        self.generate_snapshot_from_geometries_with_drawer(
            geometries,
            |geometries, snapper, image, center| -> Result<(), Error> {
                geometries
                    .into_iter()
                    .try_for_each(|geometry| geometry.draw(snapper, image, &style, center))?;

                Ok(())
            },
        )
    }

    /// Returns a snapshot centered around the provided `geometries`.
    /// The drawing of each of the `geometries` is done with the given `drawer` function.
    pub fn generate_snapshot_from_geometries_with_drawer<D>(
        &self,
        geometries: geo::GeometryCollection,
        drawer: D,
    ) -> Result<image::RgbaImage, Error>
    where
        D: Fn(
            geo::GeometryCollection,
            &Self,
            &mut image::RgbaImage,
            geo::Point,
        ) -> Result<(), Error>,
    {
        let mut output_image = image::RgbaImage::new(self.width, self.height);

        let Some(geometry_center_point) = geometries.centroid() else {
            todo!("Return an `Err` or find a suitable default for `geometry_center_point`")
        };

        self.overlay_backing_tiles(&mut output_image, geometry_center_point)?;
        drawer(geometries, self, &mut output_image, geometry_center_point)?;

        Ok(output_image)
    }

    /// Converts a [`EPSG:4326`](https://epsg.io/4326) coordinate to a [`EPSG:3857`](https://epsg.io/3857) reprojection of said coordinate.
    /// Do note, that if you're attempting to use this function to call an XYZ layer you'll need to truncate the given `point` to be [`i32s`](i32).
    pub fn epsg_4326_to_epsg_3857(&self, point: geo::Point) -> geo::Point {
        let point_as_rad = point.to_radians();
        let n = (1 << self.zoom as i32) as f64;

        geo::point!(
            x: (n * (point.y() + 180.0) / 360.0),
            y: (n * (1.0 - (point_as_rad.x().tan() + (1.0 / point_as_rad.x().cos())).ln() / PI) / 2.0)
        )
    }

    /// Converts a [`EPSG:4326`](https://epsg.io/4326) coordinate to the corresponding pixel coordinate in a snapshot.
    pub fn epsg_4326_to_pixel(&self, center: geo::Point, point: geo::Point) -> geo::Point<i32> {
        let epsg_3857_point =
            self.epsg_4326_to_epsg_3857(point) - self.epsg_4326_to_epsg_3857(center);

        geo::point!(
            x: (epsg_3857_point.x().fract() * self.tile_size as f64 + self.width as f64 / 2.0).round() as i32,
            y: (epsg_3857_point.y().fract() * self.tile_size as f64 + self.height as f64 / 2.0).round() as i32,
        )
    }
}

impl Snapper {
    /// Calls the [`tile_fetcher`](Self::tile_fetcher) function with the given coordinates and converts the returned [`image::DynamicImage`] into an [`image::RgbaImage`].
    #[inline(always)]
    fn get_tile(&self, x: i32, y: i32) -> Result<image::RgbaImage, Error> {
        let tile = (self.tile_fetcher)(x, y, self.zoom)?.to_rgba8();

        if tile.height() != self.tile_size {
            return Err(Error::IncorrectTileSize {
                expected: self.tile_size,
                received: tile.height(),
            });
        }

        if tile.width() != self.tile_size {
            return Err(Error::IncorrectTileSize {
                expected: self.tile_size,
                received: tile.height(),
            });
        }

        Ok(tile)
    }

    /// Fills the given `image` with tiles centered around the given `epsg_3857_center` point.
    fn overlay_backing_tiles(
        &self,
        image: &mut image::RgbaImage,
        center: geo::Point,
    ) -> Result<(), Error> {
        let required_rows = 0.5 * (self.height as f64) / (self.tile_size as f64);
        let required_columns = 0.5 * (self.width as f64) / (self.tile_size as f64);

        let epsg_3857_center = self.epsg_4326_to_epsg_3857(center);
        let n = 1 << self.zoom as i32;

        let min_x = (epsg_3857_center.x() - required_columns).floor() as i32;
        let min_y = (epsg_3857_center.y() - required_rows).floor() as i32;
        let max_x = (epsg_3857_center.x() + required_columns).ceil() as i32;
        let max_y = (epsg_3857_center.y() + required_rows).ceil() as i32;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let tile = self.get_tile((x + n) % n, (y + n) % n)?;

                let tile_coords = (geo::Point::from((x as f64, y as f64)) - epsg_3857_center)
                    .map_coords(|coord| geo::Coord {
                        x: coord.x * self.tile_size as f64 + self.width as f64 / 2.0,
                        y: coord.y * self.tile_size as f64 + self.height as f64 / 2.0,
                    });

                overlay(image, &tile, tile_coords.x() as i64, tile_coords.y() as i64);
            }
        }

        Ok(())
    }
}
