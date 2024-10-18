# Snapr

[![](https://img.shields.io/crates/v/snapr?style=flat-square)](https://crates.io/crates/snapr)
[![](https://img.shields.io/pypi/v/snapr?style=flat-square&color=blue)](https://pypi.org/project/snapr/)
[![](https://img.shields.io/github/license/c1m50c/snapr?style=flat-square)](https://github.com/c1m50c/snapr/blob/main/LICENSE)
[![](https://img.shields.io/github/actions/workflow/status/c1m50c/snapr/publish.yml?style=flat-square)](https://github.com/c1m50c/snapr/actions/workflows/publish.yml)

Snapr ([/ˈsnæp ər/](http://ipa-reader.xyz/?text=%CB%88sn%C3%A6p:%C9%99r)) is a library that enables a flexible and frictionless way to render snapshots of maps with overlayed geometries.

## Examples

### snapr

- [Open Street Maps](./examples/open-street-maps/) - Collection of binaries using an OSM tile fetcher.
  - [Point](./examples/open-street-maps/src/point/) - Example showing how to draw a point geometry.
  - [Line](./examples/open-street-maps/src/line/) - Example showing how to draw a line geometry.
  - [Line String](./examples/open-street-maps/src/line_string/) - Example showing how to draw a line string geometry.
  - [Polygon](./examples/open-street-maps/src/polygon/) - Example showing how to draw a polygon geometry.
- [Labels](./examples/label/) - Example showing how to label a point geometry.
- [SVGs](./examples/svg/) - Example showing how to draw an SVG on top of a point geometry.
- [Batch](./examples/batch/) - Example showing how to use a [`TileFetcher::Batch`](https://docs.rs/snapr/latest/snapr/enum.TileFetcher.html#variant.Batch), as opposed to the usual [`TileFetcher::Individual`](https://docs.rs/snapr/latest/snapr/enum.TileFetcher.html#variant.Individual) variant.

### snapr.py

- [Point](./snapr-py/examples/point.py) - Python translation of the "Open Street Maps - Point" example for `snapr`.

## License

Licensed under the [MIT License](./LICENSE) found at the root of the repository.
