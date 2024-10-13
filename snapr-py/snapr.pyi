from typing import Callable

# region lib.rs

class SnaprError(Exception): ...

class Snapr:
    def __init__(
        self,
        tile_fetcher: Callable[[int, int, int], bytearray],
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
        self, start: tuple[float, float], longitude: tuple[float, float]
    ) -> None: ...

class LineString:
    def __init__(self, points: list[tuple[float, float]]) -> None: ...

class Polygon:
    def __init__(self, exterior: LineString, interiors: list[LineString]) -> None: ...

class MultiPoint:
    def __init__(self, points: list[tuple[float, float]]) -> None: ...

class MultiLineString:
    def __init__(self, line_strings: list[LineString]) -> None: ...

class MultiPolygon:
    def __init__(self, polygons: list[Polygon]) -> None: ...

class Rect:
    def __init__(
        self, corner_1: tuple[float, float], corner_2: tuple[float, float]
    ) -> None: ...

class Triangle:
    def __init__(
        self, a: tuple[float, float], b: tuple[float, float], c: tuple[float, float]
    ) -> None: ...

class GeometryCollection:
    def __init__(self, geometries: list[Geometry]) -> None: ...

class Geometry:
    @staticmethod
    def Point(geometry: Point) -> None: ...
    @staticmethod
    def Line(geometry: Line) -> None: ...
    @staticmethod
    def LineString(geometry: LineString) -> None: ...
    @staticmethod
    def Polygon(geometry: Polygon) -> None: ...
    @staticmethod
    def MultiPoint(geometry: MultiPoint) -> None: ...
    @staticmethod
    def MultiLineString(geometry: MultiLineString) -> None: ...
    @staticmethod
    def MultiPolygon(geometry: MultiPolygon) -> None: ...
    @staticmethod
    def GeometryCollection(geometry: GeometryCollection) -> None: ...
    @staticmethod
    def Rect(geometry: Rect) -> None: ...
    @staticmethod
    def Triangle(geometry: Triangle) -> None: ...

# region style.rs

class Color:
    def __init__(self, r: int, g: int, b: int, a: int) -> None: ...

class ColorOptions:
    def __init__(
        self,
        foreground: Color,
        background: Color,
        anti_alias: bool = True,
        border: float = 1.0,
    ) -> None: ...

class Shape:
    @staticmethod
    def Circle(radius: float) -> None: ...

class Svg:
    def __init__(self, svg: str, offset: tuple[int, int] = (0, 0)) -> None: ...

class Representation:
    @staticmethod
    def Shape(shape: Shape) -> None: ...
    @staticmethod
    def Svg(svg: Svg) -> None: ...

class Label:
    def __init__(
        self,
        text: str,
        color_options: ColorOptions,
        font_family: str = "Arial",
        font_size: float = 16.0,
        offset: tuple[int, int] = (0, 0),
    ) -> None: ...

class PointStyle:
    def __init__(
        self,
        color_options: ColorOptions,
        representation: Representation,
        label: Label | None = None,
    ) -> None: ...

class LineStyle:
    def __init__(self, color_options: ColorOptions, width: float) -> None: ...

class PolygonStyle:
    def __init__(self, color_options: ColorOptions) -> None: ...

class Style:
    @staticmethod
    def PointStyle(style: PointStyle) -> None: ...
    @staticmethod
    def LineStyle(style: LineStyle) -> None: ...
    @staticmethod
    def PolygonStyle(style: PolygonStyle) -> None: ...
