use open_street_maps::tile_fetcher;
use snapr::SnaprBuilder;

fn main() -> Result<(), anyhow::Error> {
    let snapr = SnaprBuilder::new()
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
        chimney_rock.into(),
        chimney_rock_cemetery.into(),
        chimney_rock_museum.into(),
    ];

    snapr
        .generate_snapshot_from_geometries(geometries)?
        .save("example.png")?;

    Ok(())
}
