use crate::{UiElement, UiContent, ui_element::layout::Size};
use std::hash::Hash;


pub struct StackBox<T: Copy + Eq + Hash>{
    pub children: Vec<UiElement<T>>,
}

impl<T: Copy + Eq + Hash> StackBox<T> {
    pub fn new() -> Self{
        Self{
            children: Vec::new(),
        }
    }
}

impl<T: Copy + Eq + Hash> UiContent<T> for StackBox<T> {
    

    fn to_element(self, id: u32) -> UiElement<T>
    where
        Self: Sized + 'static,
    {
        let mut res = UiElement::new(id, self);
            res.layout.x_size = Size::SHRINK(0., f32::INFINITY);
            res.layout.y_size = Size::SHRINK(0., f32::INFINITY);
            res
    }

    fn content_width_range(&self) -> (f32, f32) {
        
        // maximum of all min widths and minimum of all max widths, as all elements are layed out in parallel x direction

        self.children
            .iter()
            .fold((f32::EPSILON, f32::INFINITY), |last, element| {
                (
                    last.0.max(element.width_range().0),
                    last.1.min(element.width_range().1),
                )
            })
    }

    fn content_height_range(&self) -> (f32, f32) {
        
        // maximum of all min heights and minimum of all max heights, as all elements are layed out in parallel y direction

        self.children
            .iter()
            .fold((f32::EPSILON, f32::INFINITY), |last, element| {
                (
                    last.0.max(element.height_range().0),
                    last.1.min(element.height_range().1),
                )
            })
    }

    fn get_children(&self) -> Option<&[UiElement<T>]> {
        Some(&self.children)
    }

    fn get_children_mut(&mut self) -> Option<&mut [UiElement<T>]> {
        Some(&mut self.children)
    }

    fn add(&mut self, element: UiElement<T>) -> bool {
        self.children.push(element);
        true
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        content_bounds: ggez::graphics::Rect,
    ) {
        for child in self.children.iter_mut() {
            child.draw_to_rectangle(ctx, canvas, content_bounds);
        }
    }
}