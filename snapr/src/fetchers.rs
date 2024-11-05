//! Module containing definitions and implementations for tile fetching traits.
//! See [`TileFetcher`] for more details.

#[cfg(feature = "tokio")]
use std::future::Future;
use std::sync::Arc;

use image::DynamicImage;

use crate::Error;

/// Types that represent objects that can fetch map tiles one-by-one with the tile's [`EPSG:3857`](https://epsg.io/3857) position.
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::Error;
///
/// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
///     let image = todo!("fetch tile's image from a tile provider");
///     Ok(image)
/// }
/// ```
#[cfg(feature = "rayon")]
pub trait IndividualTileFetcher: Sync {
    /// Takes in a [`EPSG:3857`](https://epsg.io/3857) coordinate and a `zoom` level, and returns an [`Image`](DynamicImage) of the tile at the given position.
    fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error>;
}

/// Types that represent objects that can fetch map tiles one-by-one with the tile's [`EPSG:3857`](https://epsg.io/3857) position.
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::Error;
///
/// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
///     let image = todo!("fetch tile's image from a tile provider");
///     Ok(image)
/// }
/// ```
#[cfg(not(feature = "rayon"))]
pub trait IndividualTileFetcher {
    /// Takes in a [`EPSG:3857`](https://epsg.io/3857) coordinate and a `zoom` level, and returns an [`Image`](DynamicImage) of the tile at the given position.
    fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error>;
}

#[cfg(feature = "rayon")]
impl<F> IndividualTileFetcher for F
where
    F: Fn(i32, i32, u8) -> Result<DynamicImage, Error> + Sync,
{
    fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
        (self)(x, y, zoom)
    }
}

#[cfg(not(feature = "rayon"))]
impl<F> IndividualTileFetcher for F
where
    F: Fn(i32, i32, u8) -> Result<DynamicImage, Error>,
{
    fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
        (self)(x, y, zoom)
    }
}

/// Types that represent objects that can fetch map tiles all at once with each tile's [`EPSG:3857`](https://epsg.io/3857) position.
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::Error;
///
/// fn tile_fetcher(coordinate_matrix: &[(i32, i32)], zoom: u8) -> Result<Vec<(i32, i32, DynamicImage)>, Error> {
///     let mut tiles = Vec::new();
///
///     for &(x, y) in coordinate_matrix {
///         let image = todo!("fetch tile's image from a tile provider");
///         tiles.push((x, y, image));
///     }
///
///     Ok(tiles)
/// }
/// ```
pub trait BatchTileFetcher {
    /// Takes in a matrix of [`EPSG:3857`](https://epsg.io/3857) coordinates and a `zoom` level, and returns a [`Vec`] of each tile's position and [`Image`](DynamicImage).
    fn fetch_tiles(
        &self,
        coordinate_matrix: &[(i32, i32)],
        zoom: u8,
    ) -> Result<Vec<(i32, i32, DynamicImage)>, Error>;
}

impl<F> BatchTileFetcher for F
where
    F: Fn(&[(i32, i32)], u8) -> Result<Vec<(i32, i32, DynamicImage)>, Error>,
{
    fn fetch_tiles(
        &self,
        coordinate_matrix: &[(i32, i32)],
        zoom: u8,
    ) -> Result<Vec<(i32, i32, DynamicImage)>, Error> {
        (self)(coordinate_matrix, zoom)
    }
}

/// Represents types implementing either [`IndividualTileFetcher`] or [`BatchTileFetcher`].
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::{Error, TileFetcher};
///
/// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
///     todo!()
/// }
///
/// let individual_tile_fetcher = TileFetcher::individual(tile_fetcher);
/// ```
pub enum TileFetcher<'a> {
    /// See [`IndividualTileFetcher`].
    Individual(Box<dyn IndividualTileFetcher + 'a>),

    /// See [`BatchTileFetcher`].
    Batch(Box<dyn BatchTileFetcher + 'a>),
}

impl<'a> TileFetcher<'a> {
    /// Constructs a new [`TileFetcher::Individual`] from a [`IndividualTileFetcher`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use image::DynamicImage;
    /// use snapr::{Error, TileFetcher};
    ///
    /// fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
    ///     todo!()
    /// }
    ///
    /// let fetcher = TileFetcher::individual(tile_fetcher);
    /// ```
    #[inline(always)]
    pub fn individual<F>(tile_fetcher: F) -> Self
    where
        F: IndividualTileFetcher + 'a,
    {
        Self::Individual(Box::new(tile_fetcher))
    }

    /// Constructs a new [`TileFetcher::Batch`] from a [`BatchTileFetcher`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use image::DynamicImage;
    /// use snapr::{Error, TileFetcher};
    ///
    /// fn tile_fetcher(coordinate_matrix: &[(i32, i32)], zoom: u8) -> Result<Vec<(i32, i32, DynamicImage)>, Error>{
    ///     todo!()
    /// }
    ///
    /// let fetcher = TileFetcher::batch(tile_fetcher);
    /// ```
    #[inline(always)]
    pub fn batch<F>(tile_fetcher: F) -> Self
    where
        F: BatchTileFetcher + 'a,
    {
        Self::Batch(Box::new(tile_fetcher))
    }
}
/// Types that represent objects that can fetch map tiles one-by-one with the tile's [`EPSG:3857`](https://epsg.io/3857) position.
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::Error;
///
/// async fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
///     let image = todo!("fetch tile's image from a tile provider");
///     Ok(image)
/// }
/// ```
#[cfg(feature = "tokio")]
#[async_trait::async_trait]
pub trait AsyncIndividualTileFetcher: Send + Sync {
    /// Takes in a [`EPSG:3857`](https://epsg.io/3857) coordinate and a `zoom` level, and returns an [`Image`](DynamicImage) of the tile at the given position.
    async fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error>;
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl<A, F> AsyncIndividualTileFetcher for F
where
    A: Future<Output = Result<DynamicImage, Error>> + Send,
    F: (Fn(i32, i32, u8) -> A) + Send + Sync,
{
    async fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
        (self)(x, y, zoom).await
    }
}

/// Types that represent objects that can fetch map tiles all at once with each tile's [`EPSG:3857`](https://epsg.io/3857) position.
///
/// ## Example
///
/// ```rust
/// use image::DynamicImage;
/// use snapr::Error;
///
/// async fn tile_fetcher(coordinate_matrix: &[(i32, i32)], zoom: u8) -> Result<Vec<(i32, i32, DynamicImage)>, Error> {
///     let mut tiles = Vec::new();
///
///     for &(x, y) in coordinate_matrix {
///         let image = todo!("fetch tile's image from a tile provider");
///         tiles.push((x, y, image));
///     }
///
///     Ok(tiles)
/// }
/// ```
#[cfg(feature = "tokio")]
#[async_trait::async_trait]
pub trait AsyncBatchTileFetcher: Sync {
    /// Takes in a matrix of [`EPSG:3857`](https://epsg.io/3857) coordinates and a `zoom` level, and returns a [`Vec`] of each tile's position and [`Image`](DynamicImage).
    async fn fetch_tiles(
        &self,
        coordinate_matrix: &[(i32, i32)],
        zoom: u8,
    ) -> Result<Vec<(i32, i32, DynamicImage)>, Error>;
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl<A, F> AsyncBatchTileFetcher for F
where
    A: Future<Output = Result<Vec<(i32, i32, DynamicImage)>, Error>> + Send,
    F: (Fn(&[(i32, i32)], u8) -> A) + Sync,
{
    async fn fetch_tiles(
        &self,
        coordinate_matrix: &[(i32, i32)],
        zoom: u8,
    ) -> Result<Vec<(i32, i32, DynamicImage)>, Error> {
        (self)(coordinate_matrix, zoom).await
    }
}

/// Represents types implementing either [`AsyncIndividualTileFetcher`] or [`AsyncBatchTileFetcher`].
#[cfg(feature = "tokio")]
pub enum AsyncTileFetcher {
    /// See [`AsyncIndividualTileFetcher`].
    Individual(Arc<dyn AsyncIndividualTileFetcher>),

    /// See [`AsyncBatchTileFetcher`].
    Batch(Box<dyn AsyncBatchTileFetcher>),
}

#[cfg(feature = "tokio")]
impl AsyncTileFetcher {
    /// Constructs a new [`AsyncTileFetcher::Individual`] from a [`AsyncIndividualTileFetcher`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use image::DynamicImage;
    /// use snapr::{Error, AsyncTileFetcher};
    ///
    /// async fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, Error> {
    ///     todo!()
    /// }
    ///
    /// let fetcher = AsyncTileFetcher::individual(tile_fetcher);
    /// ```
    #[inline(always)]
    pub fn individual<F>(tile_fetcher: F) -> Self
    where
        F: AsyncIndividualTileFetcher + 'static,
    {
        Self::Individual(Arc::new(tile_fetcher))
    }

    /// Constructs a new [`AsyncTileFetcher::Batch`] from a [`AsyncBatchTileFetcher`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use image::DynamicImage;
    /// use snapr::{Error, AsyncTileFetcher};
    ///
    /// async fn tile_fetcher(coordinate_matrix: &[(i32, i32)], zoom: u8) -> Result<Vec<(i32, i32, DynamicImage)>, Error>{
    ///     todo!()
    /// }
    ///
    /// let fetcher = AsyncTileFetcher::batch(tile_fetcher);
    /// ```
    #[inline(always)]
    pub fn batch<F>(tile_fetcher: F) -> Self
    where
        F: AsyncBatchTileFetcher + 'static,
    {
        Self::Batch(Box::new(tile_fetcher))
    }
}

#[cfg(feature = "tokio")]
impl AsyncTileFetcher {
    /// Retrieves tiles from the [`AsyncTileFetcher`] with an [`AsyncBatchTileFetcher`] executor.
    pub(crate) async fn fetch_tiles_in_batch(
        &self,
        coordinate_matrix: &[(i32, i32)],
        zoom: u8,
    ) -> Result<Vec<(i32, i32, DynamicImage)>, Error> {
        use tokio::task::JoinSet;

        let expected_tile_count = coordinate_matrix.len();

        match self {
            AsyncTileFetcher::Individual(tile_fetcher) => {
                let mut tiles = Vec::with_capacity(expected_tile_count);
                let mut tasks = JoinSet::new();

                for &(x, y) in coordinate_matrix {
                    let tile_fetcher = tile_fetcher.clone();

                    tasks.spawn(async move {
                        let tile = tile_fetcher.fetch_tile(x, y, zoom).await;
                        tile.map(|tile| (x, y, tile))
                    });
                }

                while let Some(task) = tasks.join_next().await {
                    let tile = task.map_err(|_| Error::AsynchronousTaskPanic)??;
                    tiles.push(tile);
                }

                Ok(tiles)
            }

            AsyncTileFetcher::Batch(tile_fetcher) => {
                tile_fetcher.fetch_tiles(coordinate_matrix, zoom).await
            }
        }
    }
}
