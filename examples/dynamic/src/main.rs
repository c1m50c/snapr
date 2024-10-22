use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapr::{
    drawing::{
        geometry::point::PointStyle,
        style::{ColorOptions, DynamicStyle, Style},
        DrawingState,
    },
    tiny_skia::Color,
    SnaprBuilder, TileFetcher,
};

struct Random;

impl DynamicStyle for Random {
    fn for_point(&self, _: &DrawingState, _: &geo::Point<i32>) -> Option<PointStyle> {
        let style = PointStyle {
            color_options: ColorOptions {
                foreground: Color::from_rgba8(rand::random(), rand::random(), rand::random(), 255),
                ..Default::default()
            },
            ..Default::default()
        };

        Some(style)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(TileFetcher::Individual(&tile_fetcher))
        .with_tile_size(256)
        .with_zoom(13)
        .build()?;

    let geometry = geo::point!(x: 41.04625, y: -96.31426);
    let style = Style::Dynamic(&Random);

    snapr
        .generate_snapshot_from_geometry(geometry, &[style])?
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
