use open_street_maps::tile_fetcher;
use snapr::{SnaprBuilder, TileFetcher};

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
        .with_tile_fetcher(TileFetcher::Individual(&tile_fetcher))
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

    snapr
        .snapshot_from_geometry(line)?
        .save("example.png")?;

    Ok(())
}
