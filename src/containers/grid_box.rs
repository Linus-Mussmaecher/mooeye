use ggez::{graphics::Rect, GameResult};
use std::{hash::{Hash, self}, collections::HashMap};

use crate::{UiElement, UiContent, ui_element::layout::Size};

pub struct GridBox<T: Copy + Eq + Hash>{
    children: HashMap<(u32, u32), UiElement<T>>,
    pub v_spacing: f32,
    pub h_spacing: f32,
}


impl<T: Copy + Eq + Hash> GridBox<T> {
    pub fn new() -> Self{
        Self{
            children: HashMap::new(),
            v_spacing: 5.,
            h_spacing: 5.,
        }
    }

    pub fn add(&mut self, element: UiElement<T>, x: u32, y: u32) -> GameResult{
        self.children.insert((x,y), element);
        Ok(())
    }
}

impl<T: Copy + Eq + Hash> UiContent<T> for GridBox<T>{
    fn to_element(self, id: u32) -> UiElement<T> where Self:Sized + 'static {
        let mut res = UiElement::new(id, self);
        res.layout.x_size = Size::SHRINK(0., f32::INFINITY);
        res.layout.y_size = Size::SHRINK(0., f32::INFINITY);
        res
    }
    
    fn get_children(&self) ->  Option<&[UiElement<T>]> {
        Some(&self.children.into_values().collect())
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        content_bounds: ggez::graphics::Rect,
    ) {
        todo!()
    }
}