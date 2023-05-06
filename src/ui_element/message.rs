use std::hash::Hash;

/// A simple enum that specififes what kind of messages a UI element can send or receive.
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum UiMessage<T: Copy + Eq + Hash + Hash> {
    /// An extern message, containing another Message of the specified type used by your gamestate. Never sent by elements on their own.
    Extern(T),
    /// A struct that is sent by an element when it is clicked (using the left mouse button), containing its ID. Elements with ID 0 will not send such messages.
    Clicked(u32),
    /// A struct that is sent by an element when it is clicked using the right mouse button, containing its ID. Elements with ID 0 will not send such messages.
    ClickedRight(u32),
    /// A struct that is sent by an element when one of its registered keys are pressed, containing its ID. Elements with ID 0 will not send such messages.
    PressedKey(u32),
    /// A struct that is sent if an element is 'triggered' in any way (key press or click) in addition to the specific event as above
    Triggered(u32),
}