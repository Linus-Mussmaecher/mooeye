use good_web_game::graphics::Rect;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// This struct remembers the rects a UiElement was drawn to and holds a bool that returns wether or not it can be drawn to those rects again.
pub enum DrawCache {
    /// An invalid draw cache that needs to be recalculated.
    #[default]
    Invalid,
    /// A valid draw cache that contains the last target this element received as well as the outer (visuals) and inner (content) rectangles it was drawn to.
    Valid {
        /// The rectangle this elements visual background was drawn to last frame.
        outer: Rect,
        /// The rectangle this elements content was drawn to last frame.
        inner: Rect,
        /// The rectangle this elements received as a target area last frame.
        target: Rect,
    },
}
