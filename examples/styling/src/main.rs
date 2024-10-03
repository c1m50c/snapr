use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapper::{
    drawing::style::{
        geo::{Representation, Shape, StyledPoint, StyledPointOptions},
        ColorOptions, Style,
    },
    SnapperBuilder,
};
use tiny_skia::Color;

fn main() -> Result<(), anyhow::Error> {
    let snapper = SnapperBuilder::new()
        .with_tile_fetcher(tile_fetcher)
        .with_tile_size(256)
        .with_zoom(12)
        .build()?;

    let styler = |point: &geo::Point| {
        if geo::point!(x: point.x(), y: point.y()) == geo::point!(x: 42.85643, y: -103.58290) {
            return StyledPointOptions {
                color_options: ColorOptions {
                    foreground: Color::from_rgba8(248, 128, 16, 255),
                    ..Default::default()
                },
                representation: Representation::Shape(Shape::Circle { radius: 8.0 }),
                ..Default::default()
            };
        }

        StyledPointOptions::default()
    };

    let geometries = vec![
        StyledPoint(
            geo::point!(x: 42.85643, y: -103.58290),
            Style::Dynamic(styler),
        )
        .into(),
        StyledPoint(
            geo::point!(x: 42.82386, y: -103.59335),
            Style::Dynamic(styler),
        )
        .into(),
        StyledPoint(
            geo::point!(x: 42.82336, y: -103.58356),
            Style::Dynamic(styler),
        )
        .into(),
        StyledPoint(
            geo::point!(x: 42.86754, y: -103.49842),
            Style::Dynamic(styler),
        )
        .into(),
        StyledPoint(
            geo::point!(x: 42.85873, y: -103.49636),
            Style::Dynamic(styler),
        )
        .into(),
    ];

    snapper
        .generate_snapshot_from_geometries(geometries)?
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
