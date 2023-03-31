use ggez::graphics::{Image, Drawable, Rect};




pub struct Sprite{
    frame_time: u32,
    w: u32,
    h: u32,
    spritesheet: Image,

    current_frame: u32,
    current_variant: u32,
}

impl Sprite{
    pub fn new(spritesheet: Image, w: u32, h: u32, frame_time: u32) -> Self{
        Self{
            frame_time,
            w,
            h,
            spritesheet,
            current_frame: 0,
            current_variant: 0,
        }
    }

    pub fn set_variant(&mut self, variant: u32){
        self.current_variant = variant;
    }
}

impl Drawable for Sprite{

    fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<ggez::graphics::DrawParam>) {
        
    }

    fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<ggez::graphics::Rect> {
        Some(Rect::new(0., 0., self.w as f32, self.h as f32))
    }

}