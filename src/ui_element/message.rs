use std::hash::Hash;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum UiMessage<T: Copy + Eq + Hash + Hash> {
    Extern(T),
    Clicked(u32),
    ClickedRight(u32),
}