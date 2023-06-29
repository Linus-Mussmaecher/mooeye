use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, Mesh, MeshBuilder, Rect},
    *,
};

#[derive(Clone, Copy, Debug, PartialEq)]
/// A struct that describes the additional visual elements of the background added to an element. This background will be drawn first and also contain the padding if any.
pub struct Visuals {
    /// The color of the background.
    pub background: Color,
    /// The color of the border, if present.
    pub border: Color,
    /// The width of the borders.
    /// Layout:
    /// ```text
    ///         0
    ///     +------+
    ///   3 |      | 1
    ///     +------+
    ///         2
    pub border_widths: [f32; 4],
    /// The radius of the corners.
    /// Layout:
    /// ```text
    ///    3        0
    ///     +------+
    ///     |      |
    ///     +------+
    ///    2        1
    pub corner_radii: [f32; 4],
}

impl Visuals {
    /// Returns a new Visuals with the specified contents.
    pub fn new(background: Color, border: Color, border_width: f32, corner_radius: f32) -> Self {
        Self {
            background,
            border,
            border_widths: [border_width; 4],
            corner_radii: [corner_radius; 4],
        }
    }

    /// Returns a new Visuals with the specified contents.
    pub fn new_custom(
        background: Color,
        border: Color,
        border_widths: [f32; 4],
        corner_radii: [f32; 4],
    ) -> Self {
        Self {
            background,
            border,
            border_widths,
            corner_radii,
        }
    }

    /// Draws the background to the canvas, filling the rectangle target.
    pub(crate) fn draw(&self, ctx: &Context, canvas: &mut Canvas, param: super::UiDrawParam) {
        canvas.draw(
            &Mesh::from_data(
                ctx,
                self.create_mesh(param.target).unwrap_or_default().build(),
            ),
            param.param,
        );
    }

    fn create_mesh(&self, target: Rect) -> Result<MeshBuilder, GameError> {
        let tolerance = 1.;
        let inner_radius: Vec<f32> = (0..4)
            .map(|i| {
                (self.corner_radii[i] - self.border_widths[i].max(self.border_widths[(i + 1) % 4]))
                    .max(0.)
            })
            .collect();
        // --- Circles for outer corners

        let mut mesh_builder = MeshBuilder::new();
        mesh_builder
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + target.w - self.corner_radii[0],
                    target.y + self.corner_radii[0],
                ),
                self.corner_radii[0],
                tolerance,
                self.border,
            )?
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + target.w - self.corner_radii[1],
                    target.y + target.h - self.corner_radii[1],
                ),
                self.corner_radii[1],
                tolerance,
                self.border,
            )?
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + self.corner_radii[2],
                    target.y + target.h - self.corner_radii[2],
                ),
                self.corner_radii[2],
                tolerance,
                self.border,
            )?
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + self.corner_radii[3],
                    target.y + self.corner_radii[3],
                ),
                self.corner_radii[3],
                tolerance,
                self.border,
            )?;

        // --- Outer Borders

        mesh_builder
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + self.corner_radii[3],
                    target.y,
                    target.w - self.corner_radii[3] - self.corner_radii[0],
                    target.h / 2.,
                ),
                self.border,
            )?
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + target.w / 2.,
                    target.y + self.corner_radii[0],
                    target.w / 2.,
                    target.h - self.corner_radii[0] - self.corner_radii[1],
                ),
                self.border,
            )?
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + self.corner_radii[2],
                    target.y + target.h / 2.,
                    target.w - self.corner_radii[1] - self.corner_radii[2],
                    target.h / 2.,
                ),
                self.border,
            )?
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x,
                    target.y + self.corner_radii[3],
                    target.w / 2.,
                    target.h - self.corner_radii[2] - self.corner_radii[3],
                ),
                self.border,
            )?;

        // Circles for inner corners

        mesh_builder
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + target.w - self.border_widths[1] - inner_radius[0],
                    target.y + self.border_widths[0] + inner_radius[0],
                ),
                inner_radius[0],
                tolerance,
                self.background,
            )?
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + target.w - self.border_widths[1] - inner_radius[1],
                    target.y + target.h - self.border_widths[2] - inner_radius[1],
                ),
                inner_radius[1],
                tolerance,
                self.background,
            )?
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + self.border_widths[3] + inner_radius[2],
                    target.y + target.h - self.border_widths[2] - inner_radius[2],
                ),
                inner_radius[2],
                tolerance,
                self.background,
            )?
            .circle(
                graphics::DrawMode::fill(),
                Vec2::new(
                    target.x + self.border_widths[3] + inner_radius[3],
                    target.y + self.border_widths[0] + inner_radius[3],
                ),
                inner_radius[3],
                tolerance,
                self.background,
            )?;

        // --- Rectangles for inner area.
        mesh_builder
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + self.border_widths[3] + inner_radius[3],
                    target.y + self.border_widths[0],
                    target.w
                        - self.border_widths[1]
                        - self.border_widths[3]
                        - inner_radius[3]
                        - inner_radius[0],
                    target.h / 2. - self.border_widths[0],
                ),
                self.background,
            )?
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + target.w / 2.,
                    target.y + self.border_widths[0] + inner_radius[0],
                    target.w / 2. - self.border_widths[1],
                    target.h
                        - self.border_widths[0]
                        - self.border_widths[2]
                        - inner_radius[0]
                        - inner_radius[1],
                ),
                self.background,
            )?
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + self.border_widths[3] + inner_radius[2],
                    target.y + target.h / 2.,
                    target.w
                        - self.border_widths[1]
                        - self.border_widths[3]
                        - inner_radius[1]
                        - inner_radius[2],
                    target.h / 2. - self.border_widths[2],
                ),
                self.background,
            )?
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    target.x + self.border_widths[3],
                    target.y + self.border_widths[0] + inner_radius[3],
                    target.w / 2. - self.border_widths[3],
                    target.h
                        - self.border_widths[0]
                        - self.border_widths[2]
                        - inner_radius[2]
                        - inner_radius[3],
                ),
                self.background,
            )?;

        Ok(mesh_builder)
    }

    /// Returns another visual that is the weighted (by ratio) average between ```self``` and ```other```.
    /// ```ratio=0``` will return ```self```, ```ratio=1``` will return ```other```.
    pub fn average(&self, other: Self, ratio: f32) -> Self {
        Self {
            background: Color::from_rgba(
                ((self.background.r * (1. - ratio) + other.background.r * ratio) * 256.) as u8,
                ((self.background.g * (1. - ratio) + other.background.g * ratio) * 256.) as u8,
                ((self.background.b * (1. - ratio) + other.background.b * ratio) * 256.) as u8,
                ((self.background.a * (1. - ratio) + other.background.a * ratio) * 256.) as u8,
            ),
            border: Color::from_rgba(
                ((self.border.r * (1. - ratio) + other.border.r * ratio) * 256.) as u8,
                ((self.border.g * (1. - ratio) + other.border.g * ratio) * 256.) as u8,
                ((self.border.b * (1. - ratio) + other.border.b * ratio) * 256.) as u8,
                ((self.border.a * (1. - ratio) + other.border.a * ratio) * 256.) as u8,
            ),
            border_widths: {
                let mut bw = [0.; 4];
                for (i, bw_c) in bw.iter_mut().enumerate() {
                    *bw_c = self.border_widths[i] * (1. - ratio) + other.border_widths[i] * ratio;
                }
                bw
            },
            corner_radii: {
                let mut rc = [0.; 4];
                for (i, rc_c) in rc.iter_mut().enumerate() {
                    *rc_c = self.corner_radii[i] * (1. - ratio) + other.corner_radii[i] * ratio;
                }
                rc
            },
        }
    }
}

impl Default for Visuals {
    fn default() -> Self {
        Self {
            background: Color::from_rgba(0, 0, 0, 0),
            border: Color::from_rgba(0, 0, 0, 0),
            border_widths: [0.; 4],
            corner_radii: [0.; 4],
        }
    }
}
