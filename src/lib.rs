
//TODOmaybe: Debug impls
//TODO: Restructure to proper project with lib.rs and put on GitHub
//TODO: Message System
//TODO: Transitions (visual, layout)
//TODO: Grid Box?
//TODOmaybe: wrappin in hori/verti box

pub mod ui_element;
pub use ui_element::UiContent;
pub use ui_element::UiElement;

pub mod basic;
pub mod containers;

pub mod scene_manager;
