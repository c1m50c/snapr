use tiny_skia::Color;

#[derive(Debug)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: Color::from_rgba8(248, 248, 248, 255),
            background: Color::from_rgba8(26, 26, 26, 255),
        }
    }
}
