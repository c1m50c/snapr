from typing import Callable

# region lib.rs

class SnaprError(Exception): ...

class Snapr:
    def __init__(
        self,
        tile_fetcher: Callable[
            [list[tuple[int, int]], int], list[tuple[int, int, bytearray]]
        ],
        tile_size: int = 256,
        height: int = 600,
        width: int = 800,
        zoom: int | None = None,
    ) -> None: ...
    def generate_snapshot_from_geometry(
        self, geometry: Geometry, styles: list[Style] = []
    ) -> bytearray: ...
    def generate_snapshot_from_geometries(
        self, geometries: list[Geometry], styles: list[Style] = []
    ) -> bytearray: ...

# region geo.rs

class Point:
    def __init__(self, latitude: float, longitude: float) -> None: ...

class Line:
    def __init__(
        self, start: Point | tuple[float, float], end: Point | tuple[float, float]
    ) -> None: ...

class LineString:
    def __init__(self, points: list[Point | tuple[float, float]]) -> None: ...

class Polygon:
    def __init__(self, exterior: LineString, interiors: list[LineString]) -> None: ...

class MultiPoint:
    def __init__(self, points: list[Point | tuple[float, float]]) -> None: ...

class MultiLineString:
    def __init__(self, line_strings: list[LineString]) -> None: ...

class MultiPolygon:
    def __init__(self, polygons: list[Polygon]) -> None: ...

class Rect:
    def __init__(
        self,
        corner_1: Point | tuple[float, float],
        corner_2: Point | tuple[float, float],
    ) -> None: ...

class Triangle:
    def __init__(
        self,
        a: Point | tuple[float, float],
        b: Point | tuple[float, float],
        c: Point | tuple[float, float],
    ) -> None: ...

class GeometryCollection:
    def __init__(self, geometries: list[Geometry]) -> None: ...

class Geometry:
    @staticmethod
    def Point(geometry: Point) -> Geometry: ...
    @staticmethod
    def Line(geometry: Line) -> Geometry: ...
    @staticmethod
    def LineString(geometry: LineString) -> Geometry: ...
    @staticmethod
    def Polygon(geometry: Polygon) -> Geometry: ...
    @staticmethod
    def MultiPoint(geometry: MultiPoint) -> Geometry: ...
    @staticmethod
    def MultiLineString(geometry: MultiLineString) -> Geometry: ...
    @staticmethod
    def MultiPolygon(geometry: MultiPolygon) -> Geometry: ...
    @staticmethod
    def GeometryCollection(geometry: GeometryCollection) -> Geometry: ...
    @staticmethod
    def Rect(geometry: Rect) -> Geometry: ...
    @staticmethod
    def Triangle(geometry: Triangle) -> Geometry: ...

def well_known_text_to_geometry(well_known_text: str) -> Geometry: ...
def well_known_texts_to_geometries(well_known_texts: list[str]) -> list[Geometry]: ...

# region style.rs

class Color:
    def __init__(self, r: int, g: int, b: int, a: int) -> None: ...

class ColorOptions:
    def __init__(
        self,
        foreground: Color = Color(248, 248, 248, 255),
        background: Color = Color(26, 26, 26, 255),
        anti_alias: bool = True,
        border: float | None = 1.0,
    ) -> None: ...

class Shape:
    @staticmethod
    def Circle(radius: float = 4.0) -> Shape: ...

class Svg:
    def __init__(self, svg: str, offset: tuple[int, int] = (0, 0)) -> None: ...

class Representation:
    @staticmethod
    def Shape(shape: Shape) -> Representation: ...
    @staticmethod
    def Svg(svg: Svg) -> Representation: ...

class Label:
    def __init__(
        self,
        text: str,
        color_options: ColorOptions = ColorOptions(),
        font_family: str = "Arial",
        font_size: float = 16.0,
        offset: tuple[int, int] = (0, 0),
    ) -> None: ...

class PointStyle:
    def __init__(
        self,
        color_options: ColorOptions = ColorOptions(),
        representation: Representation = Representation.Shape(Shape.Circle()),
        label: Label | None = None,
    ) -> None: ...

class LineStyle:
    def __init__(
        self,
        color_options: ColorOptions = ColorOptions(
            foreground=Color(196, 196, 196, 255), border=4.0
        ),
        width: float = 3.0,
    ) -> None: ...

class PolygonStyle:
    def __init__(
        self,
        color_options: ColorOptions = ColorOptions(
            foreground=Color(248, 248, 248, 64), border=None
        ),
    ) -> None: ...

class Style:
    @staticmethod
    def Point(style: PointStyle = PointStyle()) -> Style: ...
    @staticmethod
    def Line(style: LineStyle = LineStyle()) -> Style: ...
    @staticmethod
    def Polygon(style: PolygonStyle = PolygonStyle()) -> Style: ...
