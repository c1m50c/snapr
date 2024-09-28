use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::Client;
use snapper::SnapperBuilder;

fn main() -> Result<(), snapper::Error> {
    let snapper = SnapperBuilder::new()
        .with_tile_fetcher(tile_fetcher)
        .with_tile_size(256)
        .with_zoom(2)
        .build()?;

    let geometry = geo::Geometry::Point(
        geo::point!(x: 0.0, y: 0.0)
    );

    let snapshot = snapper.generate_snapshot_from_geometry(geometry)?;

    if let Err(err) = snapshot.save("example.png") {
        return Err(snapper::Error::Unknown { source: err.into() });
    }

    Ok(())
}

fn tile_fetcher(x: u32, y: u32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
    let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");
    let client = Client::new();

    let cursor = match client.get(&address).send() {
        Ok(response) => {
            match response.bytes() {
                Ok(response) => {
                    Cursor::new(response)
                },

                Err(error) => {
                    return Err(snapper::Error::Unknown { source: error.into() });
                }
            }
        },

        Err(error) => {
            return Err(snapper::Error::Unknown { source: error.into() });
        }
    };

    let mut image_reader = ImageReader::new(cursor);
    image_reader.set_format(ImageFormat::Png);

    image_reader.decode()
        .map_err(|error| snapper::Error::Unknown { source: error.into() })
}