
use ggez::GameResult;

use crate::{UiElement, UiContent};
use std::hash::Hash;


/// A stack box that will display elements stacked on top of one another.
/// Stores elements in a vector that determines order of elements within the box.
/// Elements will adhere to their own x and y alignment within the rectangle provided to them by this box.
/// Every child element will receive the same rectangle.
pub struct StackBox<T: Copy + Eq + Hash>{
    /// Contains the UiElements within this box in the right order (front to back).
    children: Vec<UiElement<T>>,
}

impl<T: Copy + Eq + Hash> StackBox<T> {
    pub fn new() -> Self{
        Self{
            children: Vec::new(),
        }
    }

    /// Adds a UiElement to the top of this stack box (unlike the normal add function, which adds to the bottom).
    pub fn add_top(&mut self, element: UiElement<T>) -> GameResult {
        self.children.insert(0, element);
        Ok(())
    }
}

impl<T: Copy + Eq + Hash> UiContent<T> for StackBox<T> {
    

    fn to_element_builder(
        self,
        id: u32,
        _ctx: &ggez::Context,
    ) -> crate::ui_element::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        crate::ui_element::UiElementBuilder::new(id, self)
        .as_shrink()
        .with_padding((0., 0., 0., 0.))
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

    fn add(&mut self, element: UiElement<T>) -> GameResult {
        self.children.push(element);
        Ok(())
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        param: crate::ui_element::UiDrawParam,
    ) {
        for child in self.children.iter_mut().rev() {
            child.draw_to_rectangle(ctx, canvas, param);
        }
    }
}