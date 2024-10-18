# snapr.py

[![](https://img.shields.io/pypi/v/snapr?style=flat-square&color=blue)](https://pypi.org/project/snapr/)
[![](https://img.shields.io/github/license/c1m50c/snapr?style=flat-square)](https://github.com/c1m50c/snapr/blob/main/LICENSE)
[![](https://img.shields.io/github/actions/workflow/status/c1m50c/snapr/publish.yml?style=flat-square)](https://github.com/c1m50c/snapr/actions/workflows/publish.yml)

Snapr ([/ˈsnæp ər/](http://ipa-reader.xyz/?text=%CB%88sn%C3%A6p:%C9%99r)) is a library that enables a flexible and frictionless way to render snapshots of maps with overlayed geometries. The backing library is written in Rust, these are bindings to said library.

## Example

```python
from snapr import Geometry, Point, Snapr
import requests

def tile_fetcher(coords: list[tuple[int, int]], zoom: int) -> list[tuple[int, int, bytearray]]:
    tiles = []

    for x, y in coords:
        response = requests.get(
            f"https://a.tile.osm.org/{zoom}/{x}/{y}.png",
            headers={"User-Agent": "snapr.py"},
        )

        tiles.append((x, y, bytearray(response.content)))

    return tiles

snapr = Snapr(tile_fetcher=tile_fetcher, zoom=15)

geometries = [
    Geometry.Point(Point(latitude=41.703811459356196, longitude=-103.34835922605679)),
]

snapshot = snapr.generate_snapshot_from_geometries(geometries=geometries)

with open("example.png", "wb") as image:
    image.write(snapshot)
```

![Example Output](https://github.com/user-attachments/assets/7abd9d2e-ea71-4cfb-81d6-0a03150ddd1e)

## License

Licensed under the [MIT License](https://github.com/c1m50c/snapr/blob/main/LICENSE) found at the root of the repository.
