//rules
//outer bounds:
//any element will always stay within the given bound
//if the given rectangle is too small, the element will not display (but may still consume space)
//if the given rectangle is within the bounds, the element will fit to it accoding to the size modifiers
//if the given rectangle is too large, it will align as according to the alignment modifier
//inner bounds:
//if the inner elements are smaller than the bounds, the container will still fit itself to the bounds (elements will then be given too-large rectangles and try to align accordingly)
//if the inner elements exceed the bounds, the container itself will still adhere to its bounds (elements may be given too small rectangles and as such not display)


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
