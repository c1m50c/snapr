use thiserror::Error;

pub use builder::SnapperBuilder;

mod builder;

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
/// fn tile_fetcher(x: u32, y: u32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
///     todo!()
/// }
/// ```
pub type TileFetcher = fn(u32, u32, u8) -> Result<image::DynamicImage, Error>;

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
    #[allow(unused_variables)]
    pub fn generate_snapshot_from_geometry(
        &self,
        geometry: geo::Geometry,
    ) -> Result<image::RgbaImage, Error> {
        let output_image = image::RgbaImage::new(self.width, self.height);
        Ok(output_image)
    }

    /// Returns a snapshot centered around the provided `geometries`.
    #[allow(unused_variables)]
    pub fn generate_snapshot_from_geometries(
        &self,
        geometries: Vec<geo::Geometry>,
    ) -> Result<image::RgbaImage, Error> {
        let output_image = image::RgbaImage::new(self.width, self.height);
        Ok(output_image)
    }
}

impl Snapper {
    /// Calls the [`tile_fetcher`](Self::tile_fetcher) function with the given coordinates and converts the returned [`image::DynamicImage`] into an [`image::RgbaImage`].
    #[inline(always)]
    #[allow(dead_code)]
    fn get_tile(&self, x: u32, y: u32) -> Result<image::RgbaImage, Error> {
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
}
