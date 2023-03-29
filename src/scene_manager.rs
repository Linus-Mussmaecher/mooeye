use std::collections::LinkedList;

use ggez::event::{self, EventLoop};
use ggez::*;

pub struct SceneManager
 {
    scene_stack: LinkedList<Box<dyn Scene>>,
}

impl SceneManager
 {
    pub fn new (initial_scene: Box<dyn Scene>) -> Self
    {
        SceneManager {
            scene_stack: LinkedList::from([initial_scene]),
        }
    }

    pub fn new_and_run(event_loop: EventLoop<()>, ctx: Context, initial_scene: Box<dyn Scene>) -> !
    {
        let sm = SceneManager::new(initial_scene);
        event::run(ctx, event_loop, sm)
    }

}

impl event::EventHandler<GameError> for SceneManager
 {

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if let Some(back) = self.scene_stack.back_mut() {
            back.update(ctx)?;
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

#[allow(dead_code)]
pub enum SceneSwitch
 {
    None,
    Pop(u32),
    Push(Box<dyn Scene>),
    Replace(u32, Box<dyn Scene>),
}

pub trait Scene: event::EventHandler<GameError> {
    /// Called upon each logic update to the scene.
    /// return the type of scene switch to perform. Most of the time, this will be 'None'. May throw errors.
    fn check_for_scene_switch(&mut self, ctx: &Context) -> Result<SceneSwitch, GameError>;
}