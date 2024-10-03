# snapr

Flexible and frictionless way to render snapshots of maps with stylized geometries.

## Features

### Flexibility

The [`snapr`](.) crate is designed with extensibility in mind.

#### Examples

##### Drawing

If you think our drawing solution sucks, don't use it! It can simply be disabled by removing the `drawing` feature flag. You can choose how you draw geometries via the `Snapr::generate_snapshot_from_geometries_with_drawer` method.

##### Tiles

We don't provide a default `TileFetcher` because we don't want to make the decision on how you choose to do so. We'll provide examples for common approaches to fetching tiles, but there are many crates and ways to do so, and we don't feel like restricting you to what we think is best. It's up to you how map tiles are fetched for the snapshots.

### Rendering

#### Geometry

Supports rendering all [`Geometry`](https://docs.rs/geo/latest/geo/geometry/enum.Geometry.html) primitives from the [`geo`](https://crates.io/crates/geo) crate out of the box through the `drawing` feature flag.

#### Map

Supports rendering map tiles from just about any tile provider. All tile-fetching is done through the a `TileFetcher` function, which is just a type alias for the following:

```rust
fn tile_fetcher(x: i32, y: i32, zoom: u8) -> Result<image::DynamicImage, snapr::Error> {
    todo!()
}
```

See [`examples/open-street-maps/lib.rs`](./examples/open-street-maps/src/lib.rs) for an example implementation of fetching tiles from <https://a.tile.osm.org> using [`reqwest`](https://crates.io/crates/reqwest).

### Styling

Easy to use styling system to control how geometry is drawn.
Each [`Geometry`](https://docs.rs/geo/latest/geo/geometry/enum.Geometry.html) primitive has a `Styled` counterpart that can be configured for additional aesthetics.