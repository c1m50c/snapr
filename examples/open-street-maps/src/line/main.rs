use open_street_maps::tile_fetcher;
use snapper::{
    drawing::style::{
        geo::{
            line::{StyledLine, StyledLineOptions},
            point::StyledPointOptions,
            StyledGeometry,
        },
        ColorOptions,
    },
    SnapperBuilder,
};
use tiny_skia::Color;

fn main() -> Result<(), anyhow::Error> {
    let snapper = SnapperBuilder::new()
        .with_tile_fetcher(tile_fetcher)
        .with_tile_size(256)
        .with_zoom(15)
        .build()?;

    let line = geo::Line::new(
        // Chimney Rock, Nebraska
        // https://www.openstreetmap.org/search?lat=41.703811459356196&lon=-103.34835922605679
        geo::coord! { x: 41.703811459356196, y: -103.34835922605679 },
        // Chimney Rock Museum, Nebraska
        // https://www.openstreetmap.org/search?lat=41.702909695820175&lon=-103.33250120288363
        geo::coord! { x: 41.702909695820175, y: -103.33250120288363 },
    );

    let start_point_options = StyledPointOptions {
        color_options: ColorOptions {
            foreground: Color::from_rgba8(16, 248, 16, 255),
            ..ColorOptions::default()
        },
        ..StyledPointOptions::default()
    };

    let end_point_options = StyledPointOptions {
        color_options: ColorOptions {
            foreground: Color::from_rgba8(248, 16, 16, 255),
            ..ColorOptions::default()
        },
        ..StyledPointOptions::default()
    };

    let styled_line = StyledLine(
        line,
        StyledLineOptions {
            start_point_options,
            end_point_options,
            ..StyledLineOptions::default()
        },
    );

    snapper
        .generate_snapshot_from_geometry(StyledGeometry::Line(styled_line))?
        .save("example.png")?;

    Ok(())
}
