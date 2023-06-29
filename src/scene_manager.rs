use ggez::event::{self, EventLoop};
use ggez::graphics::Color;
use ggez::*;
use std::collections::VecDeque;

/// A SceneManager instance. When using a game with multiple scenes, the scene_handler replaces you usual game manager.
/// SceneManager implements EventHandler as a usual gamestate would and can thus interact with ggez without problems.
pub struct SceneManager {
    /// The stack of scenes managed by this struct. Scenes are added to and popped from the back and draw front to back. Only the last element runs [Scene::update].
    scene_stack: VecDeque<Box<dyn Scene>>,
}

impl SceneManager {
    /// Creates a new SceneManger with the specified initial Scene. This SceneManager can then be run as any EventHandler by ggez::event::run.
    pub fn new<T: Scene + 'static>(initial_scene: T) -> Self {
        let mut sm = SceneManager {
            scene_stack: VecDeque::new(),
        };
        sm.scene_stack.push_back(Box::new(initial_scene));
        sm
    }

    /// All-in-one-method to create a new SceneManger with the specified initial scene and immediately run it.
    /// If using SceneManager, calling this method should be the last line of your main function.
    /// The running of the game will end as soon as the scene stack is emptied.
    pub fn new_and_run<T: Scene + 'static>(
        event_loop: EventLoop<()>,
        ctx: Context,
        initial_scene: T,
    ) -> ! {
        let sm = SceneManager::new(initial_scene);
        event::run(ctx, event_loop, sm)
    }
}

impl event::EventHandler<GameError> for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // Get current top scene of the stack

        if let Some(scene) = self.scene_stack.back_mut() {
            // Run update method

            let switch = scene.update(ctx)?;

            // Resolve scene switch

            match switch {
                SceneSwitch::None => {}
                SceneSwitch::Pop(n) => {
                    for _ in 0..n {
                        self.scene_stack.pop_back();
                    }
                }
                SceneSwitch::Replace(n, scene_box) => {
                    for _ in 0..n {
                        self.scene_stack.pop_back();
                    }
                    self.scene_stack.push_back(scene_box);
                }
                SceneSwitch::Push(scene_box) => {
                    self.scene_stack.push_back(scene_box);
                }
            }
        }

        // Get current top scene of the stack

        // The game ends as soon as the stack is emptied

        if self.scene_stack.is_empty() {
            ctx.request_quit();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // Clear the background (scenes should in general not clear the background, as they may be on top of other scenes)
        let canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(0, 0, 0));
        canvas.finish(ctx)?;

        // iterate over all elements, only the last (=top) element may listen to the mouse position for hover-related visual changes
        let mut it = self.scene_stack.iter_mut().peekable();

        while let Some(scenebox) = it.next() {
            scenebox.draw(ctx, it.peek().is_none())?;
        }

        Ok(())
    }
}

/// A SceneSwitch. An element of this type is returned from every scene every frame to check if the scene wants to switch to another scene.
pub enum SceneSwitch {
    /// No scene switch. The current scene stays the top scene and continues running. This should be the default return value of [Scene::update]
    None,
    /// Removes the specified number of scenes from the top of the scene stack (especially the current scene, ending it).
    Pop(u32),
    /// Pushes a new Scene on top of the scene stack, thus temporarily halting running of the current scene. Current scene will resume as soon as this scene above is popped of the stack.
    Push(Box<dyn Scene>),
    /// Pops a specified numer of scenes (as with [SceneSwitch::Pop]) of the stack and pushes a new one in the same action.
    Replace(u32, Box<dyn Scene>),
}

impl SceneSwitch {
    /// Creates an instance of [SceneSwitch::None].
    pub fn none() -> Self {
        Self::None
    }

    /// Creates an instance of [SceneSwitch::Push], handling the boxing for you.
    pub fn push(scene: impl Scene + 'static) -> Self {
        Self::Push(Box::new(scene))
    }

    /// Creates an instance of [SceneSwitch::Pop].
    pub fn pop(pop_amount: u32) -> Self {
        Self::Pop(pop_amount)
    }

    /// Creates an instance of [SceneSwitch::Replace], handling the boxing for you.
    pub fn replace(scene: impl Scene + 'static, pop_amount: u32) -> Self {
        Self::Replace(pop_amount, Box::new(scene))
    }
}

/// A scene in your game. This is basically a version of the EventHandler usually used with ggez that also returns a possible scene switch in its update function.
pub trait Scene {
    /// A function that fulfils the same purpose as [ggez::event::EventHandler::update] but also returns if the scene is to be switched.
    fn update(&mut self, ctx: &mut Context) -> Result<SceneSwitch, GameError>;

    /// A function that fulfils the same purposes of [ggez::event::EventHandler::draw], but can take an additional parameter that manages wether or not the scene reacts to the mouse (as in, tooltips show and visuals may show on hover).
    /// In general, you should NOT clear the background when drawing your scene, as it may be on top of other scenes that also need to be drawn.
    /// If you want those scenes to remain hidden, clear the background.
    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError>;
}
