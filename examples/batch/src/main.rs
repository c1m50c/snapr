use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapr::{SnaprBuilder, TileFetcher};

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(TileFetcher::Batch(&tile_fetcher))
        .with_tile_size(256)
        .with_zoom(15)
        .build()?;

    let geometry = geo::point!(x: 41.225683, y: -95.927762);

    snapr
        .snapshot_from_geometry(geometry)?
        .save("example.png")?;

    Ok(())
}

fn tile_fetcher(
    coords: &[(i32, i32)],
    zoom: u8,
) -> Result<Vec<(i32, i32, DynamicImage)>, snapr::Error> {
    let client = ClientBuilder::new()
        .user_agent("snap")
        .build()
        .map_err(anyhow::Error::from)?;

    let mut tiles = Vec::with_capacity(1_440_000);

    for (x, y) in coords {
        let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");

        let cursor = client
            .get(&address)
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.bytes())
            .map(Cursor::new)
            .map_err(anyhow::Error::from)?;

        let mut image_reader = ImageReader::new(cursor);
        image_reader.set_format(ImageFormat::Png);

        let image = image_reader.decode().map_err(anyhow::Error::from)?;
        tiles.push((*x, *y, image));
    }

    Ok(tiles)
}
