use std::io::Cursor;

use geo::line_string;
use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapr::{
    drawing::{
        geometry::{line::LineStringStyle, point::PointStyle},
        style::{ColorOptions, Effect, Styleable},
        svg::Label,
    },
    SnaprBuilder, TileFetcher,
};

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(TileFetcher::Individual(&tile_fetcher))
        .with_tile_size(256)
        .with_zoom(16)
        .build()?;

    let line_string = geo::line_string![
        (x: 41.83993, y: -103.69907),
        (x: 41.83799, y: -103.69841),
        (x: 41.83485, y: -103.69969),
    ];

    let geometry = line_string.as_styled(LineStringStyle {
        point_style: PointStyle {
            effect: Some(Effect::new(|style, _, context| PointStyle {
                label: Some(Label {
                    color_options: ColorOptions {
                        border: Some(1.25),
                        ..Default::default()
                    },
                    text: (context.index + 1).to_string(),
                    ..Default::default()
                }),
                ..style
            })),
            ..Default::default()
        },
        ..Default::default()
    });

    snapr
        .snapshot_from_drawables(vec![&geometry])?
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
