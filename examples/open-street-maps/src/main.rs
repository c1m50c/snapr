use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapper::SnapperBuilder;

fn main() -> Result<(), snapper::Error> {
    let snapper = SnapperBuilder::new()
        .with_tile_fetcher(tile_fetcher)
        .with_tile_size(256)
        .with_zoom(15)
        .build()?;

    // Chimney Rock, Nebraska
    // https://www.openstreetmap.org/search?lat=41.703811459356196&lon=-103.34835922605679
    let chimney_rock = geo::point!(x: 41.703811459356196, y: -103.34835922605679);

    // Chimney Rock Cemetery, Nebraska
    // https://www.openstreetmap.org/search?lat=41.702909695820175&lon=-103.33250120288363
    let chimney_rock_cemetery = geo::point!(x: 41.69996628239992, y: -103.34170814251178);

    // Chimney Rock Museum, Nebraska
    // https://www.openstreetmap.org/search?lat=41.702909695820175&lon=-103.33250120288363
    let chimney_rock_museum = geo::point!(x: 41.702909695820175, y: -103.33250120288363);

    let geometries = vec![
        geo::Geometry::from(chimney_rock),
        geo::Geometry::from(chimney_rock_cemetery),
        geo::Geometry::from(chimney_rock_museum),
    ];

    let snapshot = snapper
        .generate_snapshot_from_geometries(geo::GeometryCollection::from(geometries), None)?;

    if let Err(err) = snapshot.save("example.png") {
        return Err(snapper::Error::Unknown { source: err.into() });
    }

    Ok(())
}

fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
    let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");

    let client = ClientBuilder::new()
        .user_agent("snapper / 0.1.0")
        .build()
        .map_err(|error| snapper::Error::Unknown {
            source: error.into(),
        })?;

    let response = client
        .get(&address)
        .send()
        .and_then(|response| response.error_for_status());

    let cursor = match response {
        Ok(response) => match response.bytes() {
            Ok(response) => Cursor::new(response),

            Err(error) => {
                return Err(snapper::Error::Unknown {
                    source: error.into(),
                });
            }
        },

        Err(error) => {
            return Err(snapper::Error::Unknown {
                source: error.into(),
            });
        }
    };

    let mut image_reader = ImageReader::new(cursor);
    image_reader.set_format(ImageFormat::Png);

    image_reader
        .decode()
        .map_err(|error| snapper::Error::Unknown {
            source: error.into(),
        })
}
