use good_web_game::{
    graphics::{self, Drawable},
    Context,
};
use std::hash::Hash;

use crate::ui;

impl<T: Copy + Eq + Hash> ui::UiContent<T> for good_web_game::graphics::Image {
    fn to_element_builder(self, id: u32, ctx: &Context) -> ui::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        let size = self.dimensions();

        ui::UiElementBuilder::new(id, self)
            .with_size(
                ui::Size::Fill(size.w, f32::INFINITY),
                ui::Size::Fill(size.h, f32::INFINITY),
            )
            .with_preserve_ratio(true)
    }

    fn draw_content(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
        param: ui::UiDrawParam,
    ) {
        self.draw(
            ctx,
            gfx_ctx,
            param
                .param
                .dest(graphics::Point2 {
                    x: param.target.x,
                    y: param.target.y,
                })
                .scale(graphics::Vector2 {
                    x: param.target.w / self.dimensions().w,
                    y: param.target.h / self.dimensions().h,
                }),
        );
    }
}
