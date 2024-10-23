use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapr::{
    drawing::{
        geometry::point::PointStyle,
        style::{ColorOptions, Styleable},
        svg::Label,
    },
    SnaprBuilder, TileFetcher,
};

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(TileFetcher::Individual(&tile_fetcher))
        .with_tile_size(256)
        .with_zoom(13)
        .build()?;

    let point = geo::point!(x: 41.2551, y: -101.8354);

    let geometry = point.as_styled(PointStyle {
        label: Some(Label {
            color_options: ColorOptions {
                border: Some(1.5),
                ..Default::default()
            },
            text: "Water".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    });

    snapr
        .generate_snapshot(vec![&geometry])?
        .save("example.png")?;

    Ok(())
}

fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<DynamicImage, snapr::Error> {
    let address = format!("https://a.tile.osm.org/{zoom}/{x}/{y}.png");

    let client = ClientBuilder::new()
        .user_agent("snapr / 0.1.0")
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
