//! # Mooeye
//! A simple static UI library for the ggez game library.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Contains a scene manager and the scene trait.
pub mod scene_manager;
/// Contains a sprite struct that can be used to display animated images.
pub mod sprite;
/// Contains the main components for creating UIs.
pub mod ui;
