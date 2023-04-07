use ggez::graphics::Rect;

#[derive(Clone, Copy)]
/// This struct remembers the rects a UiElement was drawn to and holds a bool that returns wether or not it can be drawn to those rects again.
pub enum DrawCache {
    Invalid,
    Valid {
        outer: Rect,
        inner: Rect,
        target: Rect,
    },
}

impl Default for DrawCache {
    fn default() -> Self {
        Self::Invalid
    }
}
