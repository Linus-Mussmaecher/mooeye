use good_web_game::{
    graphics::{Drawable, Point2, Vector2},
    Context,
};
use std::hash::Hash;

use crate::ui;

impl<T: Copy + Eq + Hash> ui::UiContent<T> for good_web_game::graphics::Text {
    fn to_element_builder(self, id: u32, ctx: &Context) -> ui::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        let size = self.dimensions(ctx);

        ui::UiElementBuilder::new(id, self)
            .with_size(
                ui::Size::Fill(size.w, f32::INFINITY),
                ui::Size::Fixed(size.h),
            )
            .with_preserve_ratio(true)
    }

    fn draw_content(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
        param: ui::UiDrawParam,
    ) {
        let dim = self.dimensions(ctx);
        self.draw(
            ctx,
            gfx_ctx,
            param
                .param
                .dest(Point2 {
                    x: param.target.x,
                    y: param.target.y,
                })
                .scale(Vector2 {
                    x: param.target.w / dim.w,
                    y: param.target.h / dim.h,
                }),
        )
        .expect("[ERROR/Mooeye] Drawing text error.");
    }
}
