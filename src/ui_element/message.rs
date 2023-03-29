


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UiMessage<T: Sized + Copy> {
    Extern(T),
    Clicked(u32),
    ClickedRight(u32),
}