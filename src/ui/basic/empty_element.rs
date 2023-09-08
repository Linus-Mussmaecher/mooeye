use good_web_game::Context;
use std::hash::Hash;

use crate::ui;

impl<T: Copy + Eq + Hash> ui::UiContent<T> for () {
    fn to_element_builder(self, id: u32, _ctx: &Context) -> ui::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        ui::UiElementBuilder::new(id, self).as_shrink()
    }

    fn draw_content(
        &mut self,
        _ctx: &mut Context,
        _canvas: &mut good_web_game::event::GraphicsContext,
        _param: ui::UiDrawParam,
    ) {
    }
}
