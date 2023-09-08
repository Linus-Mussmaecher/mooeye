//! # Mooeye
//! A simple static UI library for the good_web_game game library.
//!
//! [![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/mooeye)
//! [![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/mooeye/LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/mooeye.svg)](https://crates.io/crates/mooeye)
//! [![Crates.io](https://img.shields.io/crates/d/mooeye.svg)](https://crates.io/crates/mooeye)
//!
//! See the
//! [README](https://github.com/Linus-Mussmaecher/mooeye/tree/main#readme)
//! and
//! [examples](https://github.com/Linus-Mussmaecher/mooeye/tree/main/examples/ui_examples)
//! for instructions on how to use this library to
//! * create UIs for your games or applications in the good_web_game game engige.
//! * use the scene manager to handle multiple different gamestates and menu layers gracefully.
//! * use sprites to animate objects in your UIs or in game.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Contains a scene manager and the scene trait.
pub mod scene_manager;
/// Contains a sprite struct that can be used to display animated images.
pub mod sprite;
/// Contains the main components for creating UIs.
pub mod ui;
