use ggez::{
    graphics,
    Context,
};
use std::hash::Hash;

use crate::UiContent;

impl<T: Copy + Eq + Hash> UiContent<T> for () {
    fn draw_content(
        &mut self,
        _ctx: &mut Context,
        _canvas: &mut graphics::Canvas,
        _param: crate::ui_element::UiDrawParam,
    ) {
    }
}
