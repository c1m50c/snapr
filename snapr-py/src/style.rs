use pyo3::prelude::*;
use snapr::{
    drawing::{
        geometry::{
            line::{LineStringStyle, LineStyle},
            point::{PointStyle, Representation, Shape},
            polygon::PolygonStyle,
        },
        style::{ColorOptions, Effect},
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

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(name = "PointStyle")]
pub struct PyPointStyle(PointStyle);

#[pymethods]
impl PyPointStyle {
    #[new]
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions::default()), representation = PyRepresentation::Shape(PyShape::Circle { radius: 4.0 }), label = None, effect = None))]
    fn new(
        color_options: PyColorOptions,
        representation: PyRepresentation,
        label: Option<PyLabel>,
        effect: Option<Py<PyAny>>,
    ) -> Self {
        let effect = effect.map(callable_to_effect::<geo::Point<f64>, PointStyle>);

        Self(PointStyle {
            color_options: color_options.0,
            representation: representation.into(),
            label: label.map(|x| x.0),
            effect,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(name = "LineStyle")]
pub struct PyLineStyle(LineStyle);

#[pymethods]
impl PyLineStyle {
    #[new]
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions { foreground: Color::from_rgba8(196, 196, 196, 255), border: Some(4.0), ..Default::default() }), point_style = PyPointStyle::default(), width = 3.0, effect = None))]
    fn new(
        color_options: PyColorOptions,
        point_style: PyPointStyle,
        width: f32,
        effect: Option<Py<PyAny>>,
    ) -> Self {
        let effect = effect.map(callable_to_effect::<geo::Line<f64>, LineStyle>);

        Self(LineStyle {
            color_options: color_options.0,
            width,
            point_style: point_style.0,
            effect,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(name = "LineStyle")]
pub struct PyLineStringStyle(LineStringStyle);

#[pymethods]
impl PyLineStringStyle {
    #[new]
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions { foreground: Color::from_rgba8(196, 196, 196, 255), border: Some(4.0), ..Default::default() }), point_style = PyPointStyle::default(), width = 3.0, effect = None))]
    fn new(
        color_options: PyColorOptions,
        point_style: PyPointStyle,
        width: f32,
        effect: Option<Py<PyAny>>,
    ) -> Self {
        let effect = effect.map(callable_to_effect::<geo::LineString<f64>, LineStringStyle>);

        Self(LineStringStyle {
            color_options: color_options.0,
            width,
            point_style: point_style.0,
            effect,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(name = "PolygonStyle")]
pub struct PyPolygonStyle(PolygonStyle);

#[pymethods]
impl PyPolygonStyle {
    #[new]
    #[pyo3(signature = (color_options = PyColorOptions(ColorOptions { foreground: Color::from_rgba8(248, 248, 248, 64), border: None, ..Default::default() }), line_style = PyLineStringStyle::default(), point_style = PyPointStyle::default(), effect = None))]
    fn new(
        color_options: PyColorOptions,
        line_style: PyLineStringStyle,
        point_style: PyPointStyle,
        effect: Option<Py<PyAny>>,
    ) -> Self {
        let effect = effect.map(callable_to_effect::<geo::Polygon<f64>, PolygonStyle>);

        Self(PolygonStyle {
            color_options: color_options.0,
            line_style: line_style.0,
            point_style: point_style.0,
            effect,
        })
    }
}

fn callable_to_effect<T, S>(_callable: Py<PyAny>) -> Effect<T, S> {
    todo!("Call `callable` and return an `Effect`")
}
