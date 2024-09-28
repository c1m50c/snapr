use crate::{Error, Snapper, TileFetcher};

/// Builder structure for [`Snapper`].
/// 
/// ## Example
/// 
/// ```rust
/// use image::DynamicImage;
/// use snapper::SnapperBuilder;
/// 
/// fn tile_fetcher(x: u32, y: u32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
///     todo!()
/// }
/// 
/// let snapper = SnapperBuilder::new()
///     .with_tile_fetcher(tile_fetcher)
///     .build();
/// 
/// assert!(snapper.is_ok());
/// ```
#[derive(Debug, Default)]
pub struct SnapperBuilder {
    tile_fetcher: Option<TileFetcher>,
    tile_size: Option<u32>,
    height: Option<u32>,
    width: Option<u32>,
    zoom: Option<u8>,
}

impl SnapperBuilder {
    /// Constructs a new [`SnapperBuilder`] to be used in constructing a [`Snapper`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures a [`TileFetcher`] to be used in the [`Snapper::tile_fetcher`] field.
    pub fn with_tile_fetcher(self, tile_fetcher: TileFetcher) -> Self {
        Self {
            tile_fetcher: Some(tile_fetcher),
            ..self
        }
    }

    /// Configures the `tile_size` to be used in the [`Snapper::tile_size`] field.
    pub fn with_tile_size(self, tile_size: u32) -> Self {
        Self {
            tile_size: Some(tile_size),
            ..self
        }
    }

    /// Configures the `height` to be used in the [`Snapper::height`] field.
    pub fn with_height(self, height: u32) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }

    /// Configures the `width` to be used in the [`Snapper::width`] field.
    pub fn with_width(self, width: u32) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    /// Configures the `zoom` to be used in the [`Snapper::zoom`] field.
    pub fn with_zoom(self, zoom: u8) -> Self {
        Self {
            zoom: Some(zoom),
            ..self
        }
    }

    /// Attempts to construct a new [`Snapper`] from the [`SnapperBuilder`].
    /// 
    /// ## Example
    /// 
    /// ```rust
    /// use image::DynamicImage;
    /// use snapper::SnapperBuilder;
    /// 
    /// fn tile_fetcher(x: u32, y: u32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
    ///     todo!()
    /// }
    /// 
    /// let snapper = SnapperBuilder::new()
    ///     .with_tile_fetcher(tile_fetcher)
    ///     .build();
    /// 
    /// assert!(snapper.is_ok());
    /// ```
    pub fn build(self) -> Result<Snapper, Error> {
        let Some(tile_fetcher) = self.tile_fetcher else {
            return Err(Error::Builder {
                reason: "field `tile_fetcher` needs to be set prior to a `Snapper` being built".to_string()
            });
        };

        let tile_size = self.tile_size.unwrap_or(256);
        let height = self.height.unwrap_or(600);
        let width = self.width.unwrap_or(800);
        let zoom = self.zoom.unwrap_or(15);

        let snapper = Snapper {
            tile_fetcher,
            tile_size,
            height,
            width,
            zoom,
        };

        Ok(snapper)
    }
}