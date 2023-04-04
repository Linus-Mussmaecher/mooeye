use ggez::{
    graphics::{Canvas, Color, DrawParam, Rect, MeshBuilder, Mesh},
    *, glam::Vec2,
};

#[derive(Clone, Copy, Debug)]
/// A struct that describes the additional visual elements of the background added to an element. This background will be drawn first and also contain the padding if any.
pub struct Visuals {
    /// The color of the background.
    pub background: Color,
    /// The color of the border, if present.
    pub border: Color,
    /// The width of the Border
    pub border_width: f32,
    /// The rounded corner
    pub rounded_corners: f32,
}

impl Visuals {
    /// Returns a new Visuals with the specified contents.
    pub fn new(background: Color, border: Color, border_width: f32, rounded_corners: f32) -> Self {
        Self {
            background,
            border,
            border_width,
            rounded_corners,
        }
    }

    /// Draws the background to the canvas, filling the rectangle target.
    pub(crate) fn draw(&self, ctx: &Context, canvas: &mut Canvas, target: Rect,
        param: DrawParam,) {

        let tolerance = 1.;
        let inner_radius = (self.rounded_corners - self.border_width).max(0.);

        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.rounded_corners, target.y + self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x + target.w - self.rounded_corners, target.y + self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.rounded_corners, target.y + target.h - self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x + target.w - self.rounded_corners, target.y + target.h - self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        mesh_builder.rectangle(graphics::DrawMode::fill(), Rect::new(target.x, target.y + self.rounded_corners, target.w, target.h - 2. * self.rounded_corners), self.border).expect("Adding rect went wrong");
        mesh_builder.rectangle(graphics::DrawMode::fill(), Rect::new(target.x + self.rounded_corners, target.y, target.w - 2. * self.rounded_corners, target.h), self.border).expect("Adding rect went wrong");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.border_width + inner_radius, target.y + self.border_width + inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x - self.border_width + target.w - inner_radius, target.y + self.border_width + inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.border_width + inner_radius, target.y - self.border_width + target.h - inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        mesh_builder.circle(graphics::DrawMode::fill(), Vec2::new(target.x - self.border_width + target.w - inner_radius, target.y - self.border_width + target.h - inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        mesh_builder.rectangle(graphics::DrawMode::fill(), Rect::new(target.x + self.border_width, target.y + self.border_width + inner_radius, target.w - 2. * self.border_width, target.h - 2. * self.border_width - 2. * inner_radius), self.background).expect("Adding rect went wrong");
        mesh_builder.rectangle(graphics::DrawMode::fill(), Rect::new(target.x + self.border_width + inner_radius, target.y + self.border_width, target.w - 2. * self.border_width - 2. * inner_radius, target.h - 2. * self.border_width), self.background).expect("Adding rect went wrong");

        canvas.draw(&Mesh::from_data(ctx, mesh_builder.build()), param);

    }

    pub fn average(&self, other: Self, ratio: f32) -> Self{
        Self {
            background: Color::from_rgba(
                ((self.background.r * (1. - ratio) + other.background.r * ratio) * 256.)
                    as u8,
                ((self.background.g * (1. - ratio) + other.background.g * ratio) * 256.)
                    as u8,
                ((self.background.b * (1. - ratio) + other.background.b * ratio) * 256.)
                    as u8,
                ((self.background.a * (1. - ratio) + other.background.a * ratio) * 256.)
                    as u8,
            ),
            border: Color::from_rgba(
                ((self.border.r * (1. - ratio) + other.border.r * ratio) * 256.) as u8,
                ((self.border.g * (1. - ratio) + other.border.g * ratio) * 256.) as u8,
                ((self.border.b * (1. - ratio) + other.border.b * ratio) * 256.) as u8,
                ((self.border.a * (1. - ratio) + other.border.a * ratio) * 256.) as u8,
            ),
            border_width: self.border_width * (1. - ratio)
                + other.border_width * ratio,
            rounded_corners: self.rounded_corners * (1. - ratio) + other.rounded_corners * ratio,
        }
    }

}

impl Default for Visuals {
    fn default() -> Self {
        Self {
            background: Color::from_rgba(0, 0, 0, 0),
            border: Color::from_rgba(0, 0, 0, 0),
            border_width: 0.,
            rounded_corners: 0.,
        }
    }
}