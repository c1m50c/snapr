use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::{Client, ClientBuilder};
use snapr::{fetchers::IndividualTileFetcher, SnaprBuilder, TileFetcher};

fn main() -> Result<(), anyhow::Error> {
    let tile_fetcher = OSMTileFetcher::new()?;

    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(TileFetcher::individual(tile_fetcher))
        .with_tile_size(256)
        .with_zoom(15)
        .build()?;

    let geometry = geo::point!(x: 40.807997, y: -96.699724);

    snapr
        .snapshot_from_geometry(geometry)?
        .save("example.png")?;

    Ok(())
}

struct OSMTileFetcher(Client);

impl OSMTileFetcher {
    fn new() -> Result<Self, anyhow::Error> {
        let user_agent = format!("snapr/{version}", version = env!("CARGO_PKG_VERSION"));

        let client = ClientBuilder::new()
            .user_agent(user_agent)
            .build()
            .map_err(anyhow::Error::from)?;

        Ok(Self(client))
    }
}

impl IndividualTileFetcher for OSMTileFetcher {
    fn fetch_tile(&self, x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapr::Error> {
        let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");

        let cursor = self
            .0
            .get(address)
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.bytes())
            .map(Cursor::new)
            .map_err(anyhow::Error::from)?;

        let mut image_reader = ImageReader::new(cursor);
        image_reader.set_format(ImageFormat::Png);

        let image = image_reader.decode().map_err(anyhow::Error::from)?;

        Ok(image)
    }
}
