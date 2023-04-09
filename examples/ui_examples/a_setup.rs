use std::{env, path};

use ggez::*;
use mooeye::scene_manager::SceneManager;

// # Setup
// This example contains the setup of any mooeye application using the provided SceneManager.
// Except for names, loaded fonts and the specific initial scene, you can copy & paste this into the main method of your own projects.

/// A setup function that initializes a ggez environment and mooeye scene manager.
/// The only reason this isn't the main function is that then it could not be in the top most file.
pub fn setup_and_run() -> GameResult{

    // Fetch and set resource directory.

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Generate game context and event loop.

    let (mut ctx, event_loop): (ggez::context::Context, ggez::event::EventLoop<()>) =
        ContextBuilder::new("Mooeye Examples", "Linus Mußmächer")
            .add_resource_path(resource_dir)
            .window_setup(conf::WindowSetup::default().title("Mooeye Examples"))
            .window_mode(
                conf::WindowMode::default()
                    .fullscreen_type(conf::FullscreenType::Windowed)
                    .resizable(true)
                    .dimensions(800., 600.),
            )
            .build()?;

    // Add fonts from the resource folder.

    ctx.gfx.add_font(
        "Alagard",
        graphics::FontData::from_path(&ctx, "/alagard.ttf")?,
    );

    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // Everyting above is normal ggez initialization and not specific to mooeye.
    // Below, we will start our game loop not with event::run as one would normally, but use a SceneManager instead.


    // Create StartScene.
    
    let start_scene = super::i_selector_scene::SelectorScene::new(&ctx)?;

    // Create Scene Manager and run it immediately.

    SceneManager::new_and_run(event_loop, ctx, start_scene)
}