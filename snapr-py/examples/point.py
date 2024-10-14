import requests
from snapr import Geometry, Point, Snapr, Style


def tile_fetcher(x: int, y: int, zoom: int) -> bytearray:
    response = requests.get(
        f"https://a.tile.osm.org/{zoom}/{x}/{y}.png", headers={"User-Agent": "snapr.py"}
    )

    return bytearray(response.content)


snapr = Snapr(tile_fetcher=tile_fetcher, zoom=15)
styles = [Style.Point()]

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

snapshot = snapr.generate_snapshot_from_geometries(geometries=geometries, styles=styles)

with open("exaple.png", "wb") as image:
    image.write(snapshot)
