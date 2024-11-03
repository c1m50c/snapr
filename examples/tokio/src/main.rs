use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::ClientBuilder;
use snapr::{fetchers::AsyncTileFetcher, tokio::SnaprBuilder};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(AsyncTileFetcher::individual(tile_fetcher))
        .with_tile_size(256)
        .with_zoom(16)
        .build()
        .await?;

    let geometry = geo::point!(x: 41.11839, y: -95.91013);

    snapr
        .snapshot_from_geometry(geometry)?
        .save("example.png")?;

    Ok(())
}

async fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapr::Error> {
    let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");

    let client = ClientBuilder::new()
        .user_agent("snapr / 0.1.0")
        .build()
        .map_err(anyhow::Error::from)?;

    let cursor = client
        .get(address)
        .send()
        .await
        .and_then(|response| response.error_for_status())
        .map_err(anyhow::Error::from)?
        .bytes()
        .await
        .map(Cursor::new)
        .map_err(anyhow::Error::from)?;

    let mut image_reader = ImageReader::new(cursor);
    image_reader.set_format(ImageFormat::Png);

    let image = image_reader.decode().map_err(anyhow::Error::from)?;

    Ok(image)
}
