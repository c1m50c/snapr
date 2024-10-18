# Snapr

[![](https://img.shields.io/crates/v/snapr?style=flat-square)](https://crates.io/crates/snapr)
[![](https://img.shields.io/github/license/c1m50c/snapr?style=flat-square)](https://github.com/c1m50c/snapr/blob/main/LICENSE)
[![](https://img.shields.io/github/actions/workflow/status/c1m50c/snapr/publish.yml?style=flat-square)](https://github.com/c1m50c/snapr/actions/workflows/publish.yml)

Snapr ([/ˈsnæp ər/](http://ipa-reader.xyz/?text=%CB%88sn%C3%A6p:%C9%99r)) is a library that enables a flexible and frictionless way to render snapshots of maps with overlayed geometries.

## Examples

- [Open Street Maps](https://github.com/c1m50c/snapr/blob/main/examples/open-street-maps/) - Collection of binaries using an OSM tile fetcher.
  - [Point](https://github.com/c1m50c/snapr/blob/main/examples/open-street-maps/src/point/) - Example showing how to draw a point geometry.
  - [Line](https://github.com/c1m50c/snapr/blob/main/examples/open-street-maps/src/line/) - Example showing how to draw a line geometry.
  - [Line String](https://github.com/c1m50c/snapr/blob/main/examples/open-street-maps/src/line_string/) - Example showing how to draw a line string geometry.
  - [Polygon](https://github.com/c1m50c/snapr/blob/main/examples/open-street-maps/src/polygon/) - Example showing how to draw a polygon geometry.
- [Labels](https://github.com/c1m50c/snapr/blob/main/examples/label/) - Example showing how to label a point geometry.
- [SVG](https://github.com/c1m50c/snapr/blob/main/examples/svg/) - Example showing how to draw an SVG on top of a point geometry.
- [Batch](https://github.com/c1m50c/snapr/blob/main/examples/batch/) - Example showing how to use a [`TileFetcher::Batch`](https://docs.rs/snapr/latest/snapr/enum.TileFetcher.html#variant.Batch), as opposed to the usual [`TileFetcher::Individual`](https://docs.rs/snapr/latest/snapr/enum.TileFetcher.html#variant.Individual) variant.

## License

Licensed under the [MIT License](https://github.com/c1m50c/snapr/blob/main/LICENSE) found at the root of the repository.
