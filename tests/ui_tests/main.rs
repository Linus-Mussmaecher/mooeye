use std::{env, path};

use ggez::*;
use mooeye::scene_manager::SceneManager;

mod scene_1;
mod scene_2;

const WIDTH: f32 = 768.;
const HEIGHT: f32 = 512.;

fn main() -> GameResult {
    //code snippet to fetch and set resource dir

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    //generate game context (window etc.)

    let (mut ctx, event_loop): (ggez::context::Context, ggez::event::EventLoop<()>) =
        ContextBuilder::new("Mooeye Test", "Linus Mußmächer")
            .add_resource_path(resource_dir)
            .window_setup(conf::WindowSetup::default().title("Rosemary"))
            .window_mode(
                conf::WindowMode::default()
                    .fullscreen_type(conf::FullscreenType::Windowed)
                    .resizable(true)
                    .dimensions(WIDTH, HEIGHT),
            )
            .build()?;

    //add fonts

    ctx.gfx.add_font(
        "Alagard",
        graphics::FontData::from_path(&ctx, "/alagard.ttf")?,
    );


    // create Start Scene
    
    let start_scene = scene_1::Scene1::new(&ctx)?;

    //create Scene Manager

    SceneManager::new_and_run(event_loop, ctx, start_scene)
}
