use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use reqwest::blocking::ClientBuilder;
use snapr::{
    drawing::{
        style::{
            geo::{Representation, StyledPoint, StyledPointOptions},
            Style,
        },
        svg::SvgOptions,
    },
    SnaprBuilder,
};

// https://openmoji.org/library/emoji-1F686/
const SVG: &str = r##"
<svg viewBox="0 0 72 72" xmlns="http://www.w3.org/2000/svg">
  <g>
    <rect x="21.6" y="11" rx="3.0558" ry="3.0558" width="28.8" height="33.6" fill="#d0cfce"/>
    <rect x="25" y="18" width="22" height="10" fill="#3f3f3f"/>
    <rect x="25" y="35" width="5" height="5" fill="#fcea2b"/>
    <rect x="42" y="35" width="5" height="5" fill="#fcea2b"/>
  </g>
  <g>
    <line x1="25" x2="12" y1="48" y2="61" fill="none" stroke="#000" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2"/>
    <line x1="60" x2="47" y1="61" y2="48" fill="none" stroke="#000" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2"/>
    <line x1="13" x2="59.3326" y1="57.9356" y2="58.2" fill="none" stroke="#000" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2"/>
    <line x1="17" x2="55.3326" y1="53.9356" y2="54.2" fill="none" stroke="#000" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2"/>
    <line x1="21" x2="51.3326" y1="49.9356" y2="50.2" fill="none" stroke="#000" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2"/>
    <path fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M47,19v8.8H26"/>
    <polyline fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" points="26 40 30 40 30 36"/>
    <polyline fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" points="43 40 47 40 47 36"/>
    <rect x="21.6" y="11" rx="3.0558" ry="3.0558" width="28.8" height="33.6" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"/>
  </g>
</svg>
"##;

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(&tile_fetcher)
        .with_tile_size(256)
        .with_zoom(15)
        .build()?;

    let style = StyledPointOptions {
        representation: Representation::Svg(SvgOptions {
            svg: SVG.to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let geometry = StyledPoint(
        geo::point!(x: 41.14974, y: -100.83754),
        Style::Static(style),
    );

    snapr
        .generate_snapshot_from_geometry(geometry)?
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
