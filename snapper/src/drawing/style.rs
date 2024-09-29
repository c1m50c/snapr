#[derive(Debug)]
pub struct Style {
    pub foreground: image::Rgba<u8>,
    pub background: image::Rgba<u8>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: image::Rgba([255, 255, 255, 255]),
            background: image::Rgba([0, 0, 0, 255]),
        }
    }
}
