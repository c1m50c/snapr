# snapr.py

Python bindings to the [`snapr`](https://crates.io/crates/snapr) library.

Flexible and frictionless way to render snapshots of maps with stylized geometries.

## Examples

### Open Street Maps

```py
from snapr import Geometry, Point, Snapr
import requests

def tile_fetcher(
    coords: list[tuple[int, int]], zoom: int
) -> list[tuple[int, int, bytearray]]:
    tiles = list()

    for x, y in coords:
        response = requests.get(
            f"https://a.tile.osm.org/{zoom}/{x}/{y}.png",
            headers={"User-Agent": "snapr.py"},
        )

        tiles.append((x, y, bytearray(response.content)))

    return tiles


snapr = Snapr(tile_fetcher=tile_fetcher, zoom=15)

geometries = [
    # Chimney Rock, Nebraska
    # https://www.openstreetmap.org/search?lat=41.703811459356196&lon=-103.34835922605679
    Geometry.Point(Point(latitude=41.703811459356196, longitude=-103.34835922605679)),
    # Chimney Rock Cemetery, Nebraska
    # https://www.openstreetmap.org/search?lat=41.702909695820175&lon=-103.33250120288363
    Geometry.Point(Point(latitude=41.69996628239992, longitude=-103.34170814251178)),
    # Chimney Rock Museum, Nebraska
    # https://www.openstreetmap.org/search?lat=41.702909695820175&lon=-103.33250120288363
    Geometry.Point(Point(latitude=41.702909695820175, longitude=-103.33250120288363)),
]

snapshot = snapr.generate_snapshot_from_geometries(geometries=geometries)

with open("example.png", "wb") as image:
    image.write(snapshot)
```
