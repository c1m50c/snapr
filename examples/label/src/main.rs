use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapper::{
    drawing::{
        style::{
            geo::{StyledPoint, StyledPointOptions},
            Style,
        },
        svg::LabelOptions,
    },
    SnapperBuilder,
};

fn main() -> Result<(), anyhow::Error> {
    let snapper = SnapperBuilder::new()
        .with_tile_fetcher(tile_fetcher)
        .with_tile_size(256)
        .with_zoom(13)
        .build()?;

    let style = StyledPointOptions {
        label_options: Some(LabelOptions {
            text: "Water".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let geometry = StyledPoint(geo::point!(x: 41.2551, y: -101.8354), Style::Static(style));

    snapper
        .generate_snapshot_from_geometry(geometry)?
        .save("example.png")?;

    Ok(())
}

fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapper::Error> {
    let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");

    let client = ClientBuilder::new()
        .user_agent("snapper / 0.1.0")
        .build()
        .map_err(anyhow::Error::from)?;

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

    Ok(image)
}
