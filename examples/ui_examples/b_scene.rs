use std::time::Duration;

use good_web_game::{
    event::GraphicsContext,
    graphics::{DrawParam, Drawable, Text},
    Context,
};

// # Scenes
// This example sets up a very basic scene that pops itself off the stack after two seconds.

/// A very basic scene. A Scene is just any struct containing your game state.
pub struct BScene {
    //put any of your game data here.
    /// The remaining duration this scene will live for.
    duration: Duration,
    /// The text displayed by this scene.
    hello: Text,
}

impl BScene {
    /// Creates a new BScene. There are no special parameters, but, just like in good_web_game, we need to pass a context to most creations.
    pub fn new(_ctx: &Context) -> Self {
        Self {
            duration: Duration::from_secs(2),
            hello: Text::new("This scene will close itself after 2 seconds."),
        }
    }
}

impl mooeye::scene_manager::Scene for BScene {
    fn update(
        &mut self,
        ctx: &mut good_web_game::Context,
        _gfx_ctx: &mut GraphicsContext,
    ) -> Result<mooeye::scene_manager::SceneSwitch, good_web_game::GameError> {
        // Put your game logic, any changes to your game state, here.

        self.duration = self
            .duration
            .saturating_sub(good_web_game::timer::delta(ctx));
        log::info!(
            "{} from {}",
            good_web_game::timer::delta(ctx).as_secs_f32(),
            self.duration.as_secs_f32()
        );

        // Lastly, return a Result containing a (possible) possible SceneSwitch.

        if self.duration.is_zero() {
            // A Pop will remove the current scene from the stack, basically ending it and replacing it with the scene below (if any).
            return Ok(mooeye::scene_manager::SceneSwitch::Pop(1));
        }

        // Default return type should always be None, leaving the scene stack as is and continuing to show the current scene.
        Ok(mooeye::scene_manager::SceneSwitch::None)
    }

    fn draw(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut GraphicsContext,
        _mouse_listen: bool,
    ) -> Result<(), good_web_game::GameError> {
        good_web_game::graphics::clear(
            ctx,
            gfx_ctx,
            good_web_game::graphics::Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            },
        );
        self.hello.draw(ctx, gfx_ctx, DrawParam::default())?;

        Ok(())
    }
}
