//! # Mooeye
//! A simple static UI library for the ggez game library. WORK IN PROGRESS.

// Other TODO:
// * Delayed scene switch
// * with_children
// * multidirectional borders

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Contains the main UI element struct as well as related structures such as content, transitions, UI draw parameters etc..
pub mod ui_element;
pub use ui_element::UiContent;
pub use ui_element::UiElement;
pub use ui_element::UiMessage;

/// Contains basic UI contents such as text and images.
pub mod basic;
/// Contains UI contents that contain other UI elements, such as vertical boxes and stack boxes.
pub mod containers;

/// Contains a scene manager and the scene trait.
pub mod scene_manager;
/// Contains a sprite struct that can be used to display animated images.
pub mod sprite;
