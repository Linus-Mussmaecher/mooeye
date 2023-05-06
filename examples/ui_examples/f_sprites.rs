use std::{time::Duration};

use mooeye::{
    scene_manager::{Scene, SceneSwitch},
    UiMessage, *,
};

use ggez::{
    context::Context,
    glam::Vec2,
    graphics::{Color, DrawParam, Rect},
    *,
};
use mooeye::{UiContent, UiElement};

// # Sprites
// In this example, we have a look at the sprite class that can be used for drawing animated images.

pub struct FScene {
    gui: UiElement<()>,
    // A sprite can be used as an UI element, but also simply as part of your game state separated from the UI
    sprite: sprite::Sprite,
    pos: Vec2,
    v: Vec2,
}

impl FScene {
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
        // Reusing the visuals from E.

        let vis = ui_element::Visuals {
            background: Color::from_rgb(180, 120, 60),
            border: Color::from_rgb(18, 12, 6),
            border_width: 1.,
            rounded_corners: 0.,
        };

        let hover_vis = ui_element::Visuals {
            background: Color::from_rgb(160, 100, 40),
            border: Color::from_rgb(18, 12, 6),
            border_width: 3.,
            rounded_corners: 0.,
        };

        let cont_vis = ui_element::Visuals {
            background: Color::from_rgb(60, 120, 180),
            border: Color::from_rgb(180, 180, 190),
            border_width: 1.,
            rounded_corners: 0.,
        };

        // A sprite can be loaded by specifying a path, just like an Image.
        // Additionaly, you need to inform the sprite of the grid size of its sheet and the duration each frame is displayed.
        let ui_sprite = sprite::Sprite::from_path(
            "/moo-sheet_16_16.png",
            ctx,
            16,
            24,
            Duration::from_secs_f32(0.25),
        )?
        // Just like any UI element, a sprite can have visuals, tooltip, ect.
        .to_element_builder(1, ctx)
        .scaled(5., 5.)
        .with_visuals(vis)
        .with_hover_visuals(hover_vis)
        .with_tooltip(
            graphics::Text::new("This is a sprite! Click it to end the scene.")
                .set_scale(28.)
                .set_font("Bahnschrift")
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(cont_vis)
                .with_tooltip_layout()
                .build(),
        )
        .as_shrink()
        .build();

        // A sprite can also be loaded without specifying the size of a single image on the sheet, IF the filename contains that information in the right format.
        // This is especially useful if loading every sheet in a folder within a loop.
        let non_ui_sprite = sprite::Sprite::from_path_fmt(
            "/mage-sheet_8_16.png",
            ctx,
            Duration::from_secs_f32(0.2),
        )?;

        Ok(Self {
            gui: ui_sprite,
            sprite: non_ui_sprite,
            pos: Vec2::new(50., 200.),
            v: Vec2::new(4., 4.),
        })
    }
}

impl Scene for FScene {
    fn update(&mut self, ctx: &mut Context) -> Result<SceneSwitch, GameError> {
        // Actually implementing some game state logic.

        // Pressing space changes the variant of the sprite.
        if ctx.keyboard.is_key_just_pressed(winit::event::VirtualKeyCode::Space){
            self.sprite.set_variant(self.sprite.get_variant() + 1);
        }

        // Move the sprite.
        self.pos += self.v;

        // Make the sprite bounce off the screen edges.
        let scaling = 5.;

        if self.pos.x - scaling * 4. < 0. || self.pos.x + scaling * 4. >= ctx.gfx.drawable_size().0 {
            self.v.x = self.v.x * -1.;
        }

        if self.pos.y - scaling * 8. < 0. || self.pos.y + scaling * 8. >= ctx.gfx.drawable_size().1 {
            self.v.y = self.v.y * -1.;
        }

        // And handle messages as usual

        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&UiMessage::Triggered(1)) {
            // If it is, we end the current scene (and return to the previous one) by popping it off the stack.
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Once again, we first create a canvas and set a pixel sampler. Note that this time, we dont clear the background.

        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        // Drawing of our (limited) game state.

        let mirroring = if self.v.x > 0. { 1. } else { -1. };
        let scaling = 5.;

        self.sprite.draw_sprite(
            ctx,
            &mut canvas,
            DrawParam::new()
                .dest_rect(Rect::new(
                    self.pos.x - scaling * 4. * mirroring,
                    self.pos.y - scaling * 8.,
                    scaling* mirroring, 
                    scaling,
                )),
        );

        // And once again drawing the GUI.

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;

        Ok(())
    }
}
