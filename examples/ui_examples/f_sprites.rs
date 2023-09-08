use std::time::Duration;

use mooeye::{scene_manager, sprite, ui, ui::UiContent};

use glam::Vec2;
use good_web_game::{
    event::GraphicsContext,
    graphics::{Color, DrawParam},
    *,
};

// # Sprites
// In this example, we have a look at the sprite class that can be used for drawing animated images.

/// A struct of 'game data' for scene F.
/// In addition to our GUI, this also contains sprite and position data for a game entity.
pub struct FScene {
    /// The root element of FScene's GUI.
    gui: ui::UiElement<()>,
    /// A sprite for a wizard.
    /// A sprite can be used as an UI element, but also simply as part of your game state separated from the UI
    sprite: sprite::Sprite,
    /// Another sprite displaying a cow.
    /// It will graze next to the wizard.
    sprite2: sprite::Sprite,
    /// The position of the wizard sprite on the screen.
    pos: Vec2,
    /// The speed of the wizard sprite on the screen
    v: Vec2,
}

impl FScene {
    /// Creates a new FScene in the mooeye-idiomatic way.
    pub fn new(ctx: &mut Context, gfx_ctx: &mut GraphicsContext) -> Result<Self, GameError> {
        // Reusing the visuals from E.

        let vis = ui::Visuals::new(
            Color::from_rgb(180, 120, 60),
            Color::from_rgb(18, 12, 6),
            1.,
            0.,
        );

        let hover_vis = ui::Visuals::new(
            Color::from_rgb(160, 100, 40),
            Color::from_rgb(18, 12, 6),
            3.,
            0.,
        );

        let cont_vis = ui::Visuals::new(
            Color::from_rgb(60, 120, 180),
            Color::from_rgb(180, 180, 190),
            1.,
            0.,
        );

        // A sprite can be loaded by specifying a path, just like an Image.
        // Additionaly, you need to inform the sprite of the grid size of its sheet and the duration each frame is displayed.
        let ui_sprite = sprite::Sprite::from_path(
            "./moo-sheet_16_16.png",
            ctx,
            gfx_ctx,
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
                //.set_font("Bahnschrift", 28.)
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(cont_vis)
                .build(),
        )
        .as_shrink()
        .build();

        // Sprites can also be initiated from a sprite pool, to make repeated file system access unneccessary
        // and streamline loading of multiple sprites. This requires sprites in the folder to be formatted appropriately.

        let sprite_pool = sprite::SpritePool::new()
            // with_folder loads all .png/.bmp/.jpg/.jpeg files from the passed folder and optionally its subfolders
            .with_path_list(ctx, gfx_ctx, "./sprites.txt", true);

        // We can now init a sprite from the pool. Sprites are saved in the pool with a key corresponding to their relative path
        // (from the resource folder) with the format information and file ending removed.
        let non_ui_sprite =
            sprite_pool.init_sprite("./mage-sheet", Duration::from_secs_f32(0.2))?;
        let other_sprite = sprite_pool.init_sprite("./moo-sheet", Duration::from_secs_f32(0.2))?;

        Ok(Self {
            gui: ui_sprite,
            sprite: non_ui_sprite,
            sprite2: other_sprite,
            pos: Vec2::new(50., 200.),
            v: Vec2::new(4., 4.),
        })
    }
}

impl scene_manager::Scene for FScene {
    fn update(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, GameError> {
        // Actually implementing some game state logic.

        // Pressing space changes the variant of the sprite.
        if good_web_game::input::keyboard::is_key_pressed(
            ctx,
            good_web_game::input::keyboard::KeyCode::Space,
        ) {
            self.sprite.set_variant(self.sprite.get_variant() + 1);
        }

        // Move the sprite.
        self.pos += self.v;

        // Make the sprite bounce off the screen edges.
        let scaling = 5.;

        if self.pos.x - scaling * 4. < 0. || self.pos.x + scaling * 4. >= gfx_ctx.screen_size().0 {
            self.v.x *= -1.;
        }

        if self.pos.y - scaling * 8. < 0. || self.pos.y + scaling * 8. >= gfx_ctx.screen_size().1 {
            self.v.y *= -1.;
        }

        // And handle messages as usual

        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
        mouse_listen: bool,
    ) -> Result<(), GameError> {
        // Drawing of our (limited) game state.

        // see if we need to mirror our sprite if it moves left
        let mirroring = if self.v.x > 0. { 1. } else { -1. };
        let scaling = 5.;

        self.sprite.draw_sprite(
            ctx,
            gfx_ctx,
            DrawParam::new()
                .dest(graphics::Point2 {
                    x: self.pos.x - scaling * 4. * mirroring,
                    y: self.pos.y - scaling * 8.,
                })
                .scale(graphics::Vector2 {
                    x: scaling * mirroring,
                    y: scaling,
                }),
        );

        let scaling = 2.;

        self.sprite2.draw_sprite(
            ctx,
            gfx_ctx,
            DrawParam::new()
                .dest(graphics::Point2 {
                    x: self.pos.x - scaling * 4. + 32.,
                    y: self.pos.y - scaling * 8. + 32.,
                })
                .scale(graphics::Vector2 {
                    x: scaling,
                    y: scaling,
                }),
        );

        // And once again drawing the GUI.

        self.gui.draw_to_screen(ctx, gfx_ctx, mouse_listen);

        Ok(())
    }
}
