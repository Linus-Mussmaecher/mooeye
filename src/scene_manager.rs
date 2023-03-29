use std::collections::LinkedList;

use ggez::event::{self, EventLoop};
use ggez::*;


/// A SceneManager instance. When using a game with multiple scenes, the scene_handler replaces you usual game manager.
/// SceneManager implements EventHandler as a usual gamestate would and can thus interact with ggez without problems.
pub struct SceneManager
 {
    scene_stack: LinkedList<Box<dyn Scene>>,
}

impl SceneManager
 {

    /// Creates a new SceneManger with the specified initial Scene. This SceneManager can then be run as any EventHandler by ggez::event::run.
    pub fn new<T: Scene + 'static> (initial_scene: T) -> Self
    {
        let mut sm = SceneManager {
            scene_stack: LinkedList::new(),
        };
        sm.scene_stack.push_back(Box::new(initial_scene));
        sm
    }

    /// All-in-one-method to create a new SceneManger with the specified initial scene and immediately run it.
    /// If using SceneManager, calling this method should be the last line of your main function.
    /// The running of the game will end as soon as the scene stack is emptied. 
    pub fn new_and_run<T: Scene + 'static>(event_loop: EventLoop<()>, ctx: Context, initial_scene: T) -> !
    {
        let sm = SceneManager::new(initial_scene);
        event::run(ctx, event_loop, sm)
    }

}

impl event::EventHandler<GameError> for SceneManager
 {

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {

        // Get current top scene of the stack

        if let Some(back) = self.scene_stack.back_mut() {

            // Run update function

            back.update(ctx)?;

            // Check if a scene switch has occured and resolve it

            match back.check_for_scene_switch(ctx) {
                Ok(switch) => match switch {
                    SceneSwitch::None => {}
                    SceneSwitch::Pop(n) => {
                        for _ in 0..n {
                            self.scene_stack.pop_back();
                        }
                    },
                    SceneSwitch::Replace(n, scene_box) => {
                        for _ in 0..n {
                            self.scene_stack.pop_back();
                        }
                        self.scene_stack.push_back(scene_box.into());
                    }
                    SceneSwitch::Push(scene_box) => {
                        self.scene_stack.push_back(scene_box.into());
                    }
                },
                Err(e) => return Err(e),
            }
        }

        // The game ends as soon as the stack is emptied

        if self.scene_stack.is_empty() {
            ctx.request_quit();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        for e in self.scene_stack.iter_mut() {
            e.draw(ctx)?;
        }

        Ok(())
    }
}

/// A SceneSwitch. An element of this type is returned from every scene every frame to check if the scene wants to switch to another scene
pub enum SceneSwitch
 {
    /// No scene switch. The current scene stays the top scene and continues running. This should be the default return value of [Scene::check_for_scene_switch]
    None,
    /// Removes the specified number of scenes from the top of the scene stack (especially the current scene, ending it).
    Pop(u32),
    /// Pushes a new Scene on top of the scene stack, thus temporarily halting running of the current scene. Current scene will resume as soon as this scene above is popped of the stack.
    Push(Box<dyn Scene>),
    /// Pops a specified numer of scenes (as with Pop(u32)) of the stack and pushes a new one in the same action.
    Replace(u32, Box<dyn Scene>),
}


/// A scene in your game. Any struct implementing scene must also be a basic event handler (these scenes replace your basic game state).
pub trait Scene: event::EventHandler<GameError> {
    /// Called upon each logic update to the scene.
    /// return the type of scene switch to perform. Most of the time, this will be 'None'. May throw errors.
    fn check_for_scene_switch(&mut self, ctx: &Context) -> Result<SceneSwitch, GameError>;
}