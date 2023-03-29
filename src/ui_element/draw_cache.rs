use ggez::graphics::Rect;

#[derive(Clone, Copy)]
/// This struct remembers the rects a UiElement was drawn to and holds a bool that returns wether or not it can be drawn to those rects again.
pub struct DrawCache {
    pub outer: Rect,
    pub inner: Rect,
    pub target: Rect,
    pub valid: bool,
}

impl Default for DrawCache {
    fn default() -> Self {
        Self {
            outer: Rect {
                x: 0.,
                y: 0.,
                w: 0.,
                h: 0.,
            },
            inner: Rect {
                x: 0.,
                y: 0.,
                w: 0.,
                h: 0.,
            },
            target: Rect {
                x: 0.,
                y: 0.,
                w: 0.,
                h: 0.,
            },
            valid: false,
        }
    }
}