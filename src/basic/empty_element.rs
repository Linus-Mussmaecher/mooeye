use ggez::{
    graphics::{self},
    Context, 
};

use crate::mooeye::UiContent;

impl UiContent for (){
    fn draw_content(
        &mut self,
        _ctx: &mut Context,
        _canvas: &mut graphics::Canvas,
        _content_bounds: graphics::Rect,
    ) {
        
    }
}