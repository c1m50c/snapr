use tiny_skia::Color;

pub mod geo;

#[derive(Clone, Debug, PartialEq)]
pub enum Style<O, P> {
    Static(O),
    Dynamic(fn(&P) -> O),
}

impl<O: Default, P> Default for Style<O, P> {
    fn default() -> Self {
        Self::Static(O::default())
    }
}

impl<O: Clone, P> Style<O, P> {
    /// Returns the inner option of the [`StyleOptions`].
    #[inline(always)]
    pub fn options(&self, parent: &P) -> O {
        match self {
            Self::Static(options) => options.clone(),
            Self::Dynamic(getter) => getter(parent),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorOptions {
    pub foreground: Color,
    pub background: Color,
    pub anti_alias: bool,
    pub border: Option<f32>,
}

impl Default for ColorOptions {
    fn default() -> Self {
        Self {
            foreground: Color::from_rgba8(248, 248, 248, 255),
            background: Color::from_rgba8(26, 26, 26, 255),
            anti_alias: true,
            border: Some(1.0),
        }
    }
}
