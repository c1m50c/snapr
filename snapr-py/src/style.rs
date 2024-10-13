use pyo3::prelude::*;
use snapr::{
    drawing::{
        geometry::{
            line::LineStyle,
            point::{PointStyle, Representation, Shape},
            polygon::PolygonStyle,
        },
        style::{ColorOptions, Style},
        svg::{Label, Svg},
    },
    tiny_skia::Color,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(name = "Color")]
pub struct PyColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[pymethods]
impl PyColor {
    #[new]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<Color> for PyColor {
    fn into(self) -> Color {
        Color::from_rgba8(self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "ColorOptions")]
pub struct PyColorOptions(ColorOptions);

#[pymethods]
impl PyColorOptions {
    #[new]
    #[pyo3(signature = (foreground, background, anti_alias=true, border=1.0))]
    fn new(
        foreground: PyRef<PyColor>,
        background: PyRef<PyColor>,
        anti_alias: bool,
        border: Option<f32>,
    ) -> Self {
        Self(ColorOptions {
            foreground: foreground.clone().into(),
            background: background.clone().into(),
            anti_alias,
            border,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Style")]
pub enum PyStyle {
    Point(PyPointStyle),
    Line(PyLineStyle),
    Polygon(PyPolygonStyle),
}

impl Into<Style> for PyStyle {
    fn into(self) -> Style {
        match self {
            Self::Point(style) => Style::Point(style.0),
            Self::Line(style) => Style::Line(style.0),
            Self::Polygon(style) => Style::Polygon(style.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Shape")]
pub enum PyShape {
    Circle { radius: f32 },
}

impl Into<Shape> for PyShape {
    fn into(self) -> Shape {
        match self {
            Self::Circle { radius } => Shape::Circle { radius },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Svg")]
pub struct PySvg(Svg);

#[pymethods]
impl PySvg {
    #[new]
    #[pyo3(signature = (svg, offset = (0, 0)))]
    fn new(svg: String, offset: (i32, i32)) -> Self {
        Self(Svg { offset, svg })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Representation")]
pub enum PyRepresentation {
    Shape(PyShape),
    Svg(PySvg),
}

impl Into<Representation> for PyRepresentation {
    fn into(self) -> Representation {
        match self {
            Self::Shape(shape) => Representation::Shape(shape.into()),
            Self::Svg(svg) => Representation::Svg(svg.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Label")]
pub struct PyLabel(Label);

#[pymethods]
impl PyLabel {
    #[new]
    #[pyo3(signature = (text, color_options, font_family = "Arial".to_string(), font_size = 16.0, offset = (0, 0)))]
    fn new(
        text: String,
        color_options: PyRef<PyColorOptions>,
        font_family: String,
        font_size: f32,
        offset: (i32, i32),
    ) -> Self {
        Self(Label {
            color_options: color_options.clone().0,
            font_family,
            font_size,
            offset,
            text,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "PointStyle")]
pub struct PyPointStyle(PointStyle);

#[pymethods]
impl PyPointStyle {
    #[new]
    #[pyo3(signature = (color_options, representation, label = None))]
    fn new(
        color_options: PyRef<PyColorOptions>,
        representation: PyRef<PyRepresentation>,
        label: Option<PyRef<PyLabel>>,
    ) -> Self {
        Self(PointStyle {
            color_options: color_options.clone().0,
            representation: representation.clone().into(),
            label: label.map(|x| x.clone().0),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "LineStyle")]
pub struct PyLineStyle(LineStyle);

#[pymethods]
impl PyLineStyle {
    #[new]
    fn new(color_options: PyRef<PyColorOptions>, width: f32) -> Self {
        Self(LineStyle {
            color_options: color_options.clone().0,
            width,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "PolygonStyle")]
pub struct PyPolygonStyle(PolygonStyle);

#[pymethods]
impl PyPolygonStyle {
    #[new]
    fn new(color_options: PyRef<PyColorOptions>) -> Self {
        Self(PolygonStyle {
            color_options: color_options.clone().0,
        })
    }
}
