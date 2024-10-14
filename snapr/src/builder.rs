use std::fmt;

use crate::{Error, Snapr, TileFetcher};

/// Builder structure for [`snapr`].
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::{SnaprBuilder, TileFetcher};
///
/// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapr::Error> {
///     todo!()
/// }
///
/// let snapr = SnaprBuilder::new()
///     .with_tile_fetcher(TileFetcher::Individual(&tile_fetcher))
///     .build();
///
/// assert!(snapr.is_ok());
/// ```
#[derive(Default)]
pub struct SnaprBuilder<'a> {
    tile_fetcher: Option<TileFetcher<'a>>,
    tile_size: Option<u32>,
    height: Option<u32>,
    width: Option<u32>,
    zoom: Option<u8>,
}

impl<'a> SnaprBuilder<'a> {
    /// Constructs a new [`SnaprBuilder`] to be used in constructing a [`Snapr`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures a [`TileFetcher`] to be used in the [`Snapr::tile_fetcher`] field.
    pub fn with_tile_fetcher(self, tile_fetcher: TileFetcher<'a>) -> Self {
        Self {
            tile_fetcher: Some(tile_fetcher),
            ..self
        }
    }

    /// Configures the `tile_size` to be used in the [`Snapr::tile_size`] field.
    pub fn with_tile_size(self, tile_size: u32) -> Self {
        Self {
            tile_size: Some(tile_size),
            ..self
        }
    }

    /// Configures the `height` to be used in the [`Snapr::height`] field.
    pub fn with_height(self, height: u32) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }

    /// Configures the `width` to be used in the [`Snapr::width`] field.
    pub fn with_width(self, width: u32) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    /// Configures the `zoom` to be used in the [`Snapr::zoom`] field.
    pub fn with_zoom(self, zoom: u8) -> Self {
        Self {
            zoom: Some(zoom),
            ..self
        }
    }

    /// Attempts to construct a new [`Snapr`] from the [`SnaprBuilder`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use image::DynamicImage;
    /// use snapr::{SnaprBuilder, TileFetcher};
    ///
    /// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapr::Error> {
    ///     todo!()
    /// }
    ///
    /// let snapr = SnaprBuilder::new()
    ///     .with_tile_fetcher(TileFetcher::Individual(&tile_fetcher))
    ///     .build();
    ///
    /// assert!(snapr.is_ok());
    /// ```
    pub fn build(self) -> Result<Snapr<'a>, Error> {
        let Some(tile_fetcher) = self.tile_fetcher else {
            return Err(Error::Builder {
                reason: "field `tile_fetcher` needs to be set prior to a `snapr` being built"
                    .to_string(),
            });
        };

        let tile_size = self.tile_size.unwrap_or(256);
        let height = self.height.unwrap_or(600);
        let width = self.width.unwrap_or(800);
        let zoom = self.zoom;

        let snapr = Snapr {
            tile_fetcher,
            tile_size,
            height,
            width,
            zoom,
        };

        Ok(snapr)
    }
}

impl<'a> fmt::Debug for SnaprBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SnaprBuilder")
            .field("tile_size", &self.tile_size)
            .field("height", &self.height)
            .field("width", &self.width)
            .field("zoom", &self.zoom)
            .finish()
    }
}
