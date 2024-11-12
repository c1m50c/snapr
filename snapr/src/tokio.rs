//! Contains a [`SnaprBuilder`] implementation that constructs an [`AsyncTileFetcher`] with a [`tokio`] executor.

use std::{fmt, thread};

use tokio::runtime::Handle;

use crate::{
    builder::macros::impl_snapr_builder,
    fetchers::{AsyncTileFetcher, BatchTileFetcher},
    Error, Snapr, TileFetcher, Zoom,
};

/// Builder structure for [`Snapr`].
#[derive(Default)]
pub struct SnaprBuilder<'a> {
    tile_fetcher: Option<AsyncTileFetcher<'a>>,
    tile_size: Option<u32>,
    height: Option<u32>,
    width: Option<u32>,
    zoom: Option<Zoom>,
}

impl<'a> SnaprBuilder<'a> {
    /// Attempts to construct a new [`Snapr`] from the [`SnaprBuilder`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use image::DynamicImage;
    /// use snapr::{AsyncTileFetcher, tokio::SnaprBuilder};
    ///
    /// async fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapr::Error> {
    ///     todo!()
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let snapr = SnaprBuilder::new()
    ///         .with_tile_fetcher(AsyncTileFetcher::individual(tile_fetcher))
    ///         .build()
    ///         .await;
    ///
    ///     assert!(snapr.is_ok());
    /// }
    /// ```
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "TRACE", skip(self), err)
    )]
    pub async fn build(self) -> Result<Snapr<'a>, Error> {
        let Some(tile_fetcher) = self.tile_fetcher else {
            return Err(Error::Builder {
                reason: "field `tile_fetcher` needs to be set prior to a `snapr` being built"
                    .to_string(),
            });
        };

        let tile_size = self.tile_size.unwrap_or(256);
        let height = self.height.unwrap_or(600);
        let width = self.width.unwrap_or(800);
        let zoom = self.zoom.unwrap_or_default();

        let tile_fetcher = {
            let tokio_tile_fetcher = TokioTileFetcher {
                handle: Handle::current(),
                inner: tile_fetcher,
            };

            #[cfg(feature = "tracing")]
            {
                tracing::trace!(
                    handle = ?tokio_tile_fetcher.handle,
                    "built internal `TokioTileFetcher`"
                );
            }

            TileFetcher::batch(tokio_tile_fetcher)
        };

        let snapr = crate::Snapr {
            tile_fetcher,
            tile_size,
            height,
            width,
            zoom,
        };

        Ok(snapr)
    }
}

impl_snapr_builder!(
    (SnaprBuilder<'a>, SnaprBuilder),
    (Snapr<'a>, Snapr),
    (AsyncTileFetcher<'a>, AsyncTileFetcher)
);

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

struct TokioTileFetcher<'a> {
    handle: Handle,
    inner: AsyncTileFetcher<'a>,
}

impl<'a> BatchTileFetcher for TokioTileFetcher<'a> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "TRACE", skip(self), err)
    )]
    fn fetch_tiles(
        &self,
        coordinate_matrix: &[(i32, i32)],
        zoom: u8,
    ) -> Result<Vec<(i32, i32, image::DynamicImage)>, Error> {
        thread::scope(move |scope| {
            let spawned = scope.spawn(move || {
                #[cfg(feature = "tracing")]
                {
                    tracing::trace!("spawned `std::thread` to execute future on");
                }

                self.handle.block_on(async move {
                    #[cfg(feature = "tracing")]
                    {
                        tracing::trace!("running `Handle::block_on` on `AsyncTileFetcher.fetch_tiles_in_batch` future");
                    }

                    self.inner
                        .fetch_tiles_in_batch(coordinate_matrix, zoom)
                        .await
                })
            });

            spawned.join().map_err(|_| Error::AsynchronousTaskPanic)?
        })
    }
}
