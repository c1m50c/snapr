use tiny_skia::Pixmap;

use crate::Snapper;

pub trait Drawable {
    fn draw(
        &self,
        snapper: &Snapper,
        pixmap: &mut Pixmap,
        center: geo::Point,
    ) -> Result<(), crate::Error>;
}