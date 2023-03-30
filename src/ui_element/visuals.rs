use ggez::{
    graphics::{Canvas, Color, DrawParam, Rect, MeshBuilder, Mesh},
    *, glam::Vec2,
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
    pub(crate) fn draw(&self, ctx: &Context, canvas: &mut Canvas, target: Rect) {
        let tolerance = 1.;

        let mut outer_mesh = MeshBuilder::new();
        outer_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.rounded_corners, target.y + self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        outer_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x + target.w - self.rounded_corners, target.y + self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        outer_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.rounded_corners, target.y + target.h - self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        outer_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x + target.w - self.rounded_corners, target.y + target.h - self.rounded_corners), self.rounded_corners, tolerance, self.border).expect("Adding circle did not work.");
        outer_mesh.rectangle(graphics::DrawMode::fill(), Rect::new(target.x, target.y + self.rounded_corners, target.w, target.h - 2. * self.rounded_corners), self.border).expect("Adding rect went wrong");
        outer_mesh.rectangle(graphics::DrawMode::fill(), Rect::new(target.x + self.rounded_corners, target.y, target.w - 2. * self.rounded_corners, target.h), self.border).expect("Adding rect went wrong");

        canvas.draw(&Mesh::from_data(ctx, outer_mesh.build()), DrawParam::default());

        let inner_radius = (self.rounded_corners - self.border_width).max(0.);

        
        let mut inner_mesh = MeshBuilder::new();
        inner_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.border_width + inner_radius, target.y + self.border_width + inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        inner_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x - self.border_width + target.w - inner_radius, target.y + self.border_width + inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        inner_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x + self.border_width + inner_radius, target.y - self.border_width + target.h - inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        inner_mesh.circle(graphics::DrawMode::fill(), Vec2::new(target.x - self.border_width + target.w - inner_radius, target.y - self.border_width + target.h - inner_radius), inner_radius, tolerance, self.background).expect("Adding circle did not work.");
        inner_mesh.rectangle(graphics::DrawMode::fill(), Rect::new(target.x + self.border_width, target.y + self.border_width + inner_radius, target.w - 2. * self.border_width, target.h - 2. * self.border_width - 2. * inner_radius), self.background).expect("Adding rect went wrong");
        inner_mesh.rectangle(graphics::DrawMode::fill(), Rect::new(target.x + self.border_width + inner_radius, target.y + self.border_width, target.w - 2. * self.border_width - 2. * inner_radius, target.h - 2. * self.border_width), self.background).expect("Adding rect went wrong");

        canvas.draw(&Mesh::from_data(ctx, inner_mesh
.build()), DrawParam::default());
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