use std::{env, path};

use good_web_game::*;
use mooeye::scene_manager::SceneManager;

// # Setup
// This example contains the setup of any mooeye application using the provided SceneManager.
// Except for names, loaded fonts and the specific initial scene, you can copy & paste this into the main method of your own projects.

/// A setup function that initializes a good_web_game environment and mooeye scene manager.
/// The only reason this isn't the main function is that then it could not be in the top most file.
pub fn setup_and_run() -> GameResult {
    // Fetch and set resource directory.

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("./resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Generate game context and event loop.

    let conf = good_web_game::conf::Conf::default()
        .physical_root_dir(Some(resource_dir))
        .cache(Some(include_bytes!("resources.tar")));

    good_web_game::start(conf, |ctx, mut _gfx_ctx| {
        // Add fonts from the resource folder.
        let data = ctx
            .filesystem
            .open("./bahnschrift.ttf")
            .unwrap()
            .bytes
            .into_inner();

        super::BAHNSCHRIFT.with(|bs| {
            *bs.borrow_mut() = good_web_game::graphics::Font::new_glyph_font_bytes(ctx, &data).ok();
        });
        let start_scene = super::g_selector_scene::SelectorScene::new(ctx).unwrap();
        let sm = SceneManager::new(start_scene);
        Box::new(sm)
    })
}
