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

#[allow(clippy::from_over_into)]
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
    #[pyo3(signature = (foreground = PyColor::new(248, 248, 248, 255), background = PyColor::new(26, 26, 26, 255), anti_alias=true, border=1.0))]
    fn new(
        foreground: PyColor,
        background: PyColor,
        anti_alias: bool,
        border: Option<f32>,
    ) -> Self {
        Self(ColorOptions {
            foreground: foreground.into(),
            background: background.into(),
            anti_alias,
            border,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "Style")]
pub enum PyStyle {
    #[pyo3(constructor = (_0 = PyPointStyle(PointStyle::default())))]
    Point(PyPointStyle),

    #[pyo3(constructor = (_0 = PyLineStyle(LineStyle::default())))]
    Line(PyLineStyle),

    #[pyo3(constructor = (_0 = PyPolygonStyle(PolygonStyle::default())))]
    Polygon(PyPolygonStyle),
}

#[allow(clippy::from_over_into)]
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
    #[pyo3(constructor = (radius = 4.0))]
    Circle { radius: f32 },
}

#[allow(clippy::from_over_into)]
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

#[allow(clippy::from_over_into)]
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
    #[pyo3(signature = (text, color_options = PyColorOptions(ColorOptions::default()), font_family = "Arial".to_string(), font_size = 16.0, offset = (0, 0)))]
    fn new(
        text: String,
        color_options: PyColorOptions,
        font_family: String,
        font_size: f32,
        offset: (i32, i32),
    ) -> Self {
        Self(Label {
            color_options: color_options.0,
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
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions::default()), representation = PyRepresentation::Shape(PyShape::Circle { radius: 4.0 }), label = None))]
    fn new(
        color_options: PyColorOptions,
        representation: PyRepresentation,
        label: Option<PyLabel>,
    ) -> Self {
        Self(PointStyle {
            color_options: color_options.0,
            representation: representation.into(),
            label: label.map(|x| x.0),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "LineStyle")]
pub struct PyLineStyle(LineStyle);

#[pymethods]
impl PyLineStyle {
    #[new]
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions { foreground: Color::from_rgba8(196, 196, 196, 255), border: Some(4.0), ..Default::default() }), width = 3.0))]
    fn new(color_options: PyColorOptions, width: f32) -> Self {
        Self(LineStyle {
            color_options: color_options.0,
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
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions { foreground: Color::from_rgba8(248, 248, 248, 64), border: None, ..Default::default() })))]
    fn new(color_options: PyColorOptions) -> Self {
        Self(PolygonStyle {
            color_options: color_options.0,
        })
    }
}
