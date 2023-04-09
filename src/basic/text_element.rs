use ggez::{
    graphics::{Canvas, Drawable, Rect},
    Context,
};
use std::hash::Hash;


use crate::{UiContent, ui_element::Size};

impl<T: Copy + Eq + Hash> UiContent<T> for ggez::graphics::Text {
    fn to_element_builder(self, id: u32, ctx: &Context) -> crate::ui_element::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        let size = self.dimensions(&ctx.gfx).unwrap_or(Rect {
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
        });

        crate::ui_element::UiElementBuilder::new(id, self)
            .with_size(
                Size::Fill(size.w, f32::INFINITY),
                Size::Fixed(size.h),
            )
            .with_preserve_ratio(true)
    }


    fn draw_content(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        param: crate::ui_element::UiDrawParam,
    ) {
        if let Some(dim) = self.dimensions(ctx) {
            canvas.draw(
                self,
                param.param.dest_rect(Rect::new(
                    param.target.x,
                    param.target.y,
                    param.target.w / dim.w,
                    param.target.h / dim.h,
                )),
            );
        }
    }
}
