//! # Mooeye
//! A simple static UI library for the ggez game library.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Contains the main UI element struct as well as related structures such as content, transitions, UI draw parameters and more.
pub mod ui_element;
pub use ui_element::UiContent;
pub use ui_element::UiElement;
pub use ui_element::UiMessage;

/// Contains basic UI contents such as text and images.
/// There is nothing actually here, because the basic elements Text, Image and Empty
/// are created by simply implementing UiContent on ggez's Text and Image as well as the basic ().
pub mod basic;
/// Contains UI contents that contain other UI elements, such as vertical boxes and stack boxes.
pub mod containers;

/// Contains a scene manager and the scene trait.
pub mod scene_manager;
/// Contains a sprite struct that can be used to display animated images.
pub mod sprite;
