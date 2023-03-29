use ggez::{
    graphics::{Canvas, Color, DrawParam, Rect},
    *,
};

#[derive(Clone, Copy)]
/// A struct that describes the additional visual elements of the background added to an element. This background will be drawn first and also contain the padding if any.
pub struct Visuals {
    /// The color of the background.
    pub background: Color,
    /// The color of the border, if present.
    pub border: Color,
    /// The width of the Border
    pub border_width: f32,
}

impl Visuals {
    /// Returns a new Visuals with the specified contents.
    pub fn new(background: Color, border: Color, border_width: f32) -> Self {
        Self {
            background,
            border,
            border_width,
        }
    }

    /// Draws the background to the canvas, filling the rectangle target.
    pub(crate) fn draw(&self, canvas: &mut Canvas, target: Rect) {
        canvas.draw(
            &graphics::Quad,
            DrawParam::default().dest_rect(target).color(self.border),
        );

        canvas.draw(
            &graphics::Quad,
            DrawParam::default()
                .dest_rect(Rect::new(
                    target.x + self.border_width,
                    target.y + self.border_width,
                    (target.w - 2. * self.border_width).max(0.),
                    (target.h - 2. * self.border_width).max(0.),
                ))
                .color(self.background),
        );
    }
}

impl Default for Visuals {
    fn default() -> Self {
        Self {
            background: Color::from_rgba(0, 0, 0, 0),
            border: Color::from_rgba(0, 0, 0, 0),
            border_width: 0.,
        }
    }
}