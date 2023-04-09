use std::time::Duration;

use ggez::{graphics::{Text, Drawable, Color, DrawParam}, Context};




// # Scenes
// This example sets up a very basic scene that pops itself off the stack after two seconds.


/// A very basic scene. A Scene is just any struct containing your game state.
pub struct BScene{

    //put any of your game data here.

    /// The remaining duration this scene will live for.
    duration: Duration,
    /// The text displayed by this scene.
    hello: Text,
}

impl  BScene {
    pub fn new(_ctx: &Context) -> Self{
        Self { duration: Duration::from_secs(2), hello: Text::new("This scene will close itself after 2 seconds.") }
    }
}

impl mooeye::scene_manager::Scene for BScene {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<mooeye::scene_manager::SceneSwitch, ggez::GameError> {
        
        // Put your game logic, any changes to your game state, here.
        
        self.duration = self.duration.saturating_sub(ctx.time.delta());

        // Lastly, return a Result containing a (possible) possible SceneSwitch.

        if self.duration.is_zero() {
            // A Pop will remove the current scene from the stack, basically ending it and replacing it with the scene below (if any).
            return Ok(mooeye::scene_manager::SceneSwitch::Pop(1));
        }

        // Default return type should always be None, leaving the scene stack as is and continuing to show the current scene.
        Ok(mooeye::scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut ggez::Context, _mouse_listen: bool) -> Result<(), ggez::GameError> {
        // As in ggez, your draw function draws the contents of your scene and always start by getting a canvas.
        // This function should not alter your game state (even if it can access self mutably), only the display of that state.
        // Clearing the canvas should not be your default option - sometimes you may want a scene (like a pause menu) to 
        // occupy only a small part of the screen while still drawing other scenes behind it. In that case, pass None into 'clear'.
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 100, 150));
        // Use this sampler setting if you are using pixel graphics.
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());


        self.hello.draw(&mut canvas, DrawParam::default());

        // The draw function should always end by finishing the canvas.
        canvas.finish(ctx)?;

        Ok(())
    }
}


