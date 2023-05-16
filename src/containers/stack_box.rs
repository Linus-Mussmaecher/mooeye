use ggez::GameResult;

use crate::{ui_element::UiContainer, UiContent, UiElement};
use std::hash::Hash;

/// A stack box that will display elements stacked on top of one another.
/// Stores elements in a vector that determines order of elements within the box.
/// Elements will adhere to their own x and y alignment within the rectangle provided to them by this box.
/// Every child element will receive the same rectangle.
pub struct StackBox<T: Copy + Eq + Hash> {
    /// Contains the UiElements within this box in the right order (front to back).
    children: Vec<UiElement<T>>,
}

impl<T: Copy + Eq + Hash> StackBox<T> {
    pub fn new() -> Self {
        Self {
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

    fn container(&self) -> Option<&dyn UiContainer<T>> {
        Some(self)
    }

    fn container_mut(&mut self) -> Option<&mut dyn UiContainer<T>> {
        Some(self)
    }
}

impl<T: Copy + Eq + Hash> UiContainer<T> for StackBox<T> {
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

    fn get_children(&self) -> &[UiElement<T>] {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut [UiElement<T>] {
        &mut self.children
    }

    fn add(&mut self, element: UiElement<T>) {
        self.children.push(element);
    }

    fn remove_expired(&mut self) {
        self.children.retain(|child| !child.expired());
    }

    
    fn remove_id(&mut self, id: u32) {
        self.children.retain(|child| child.get_id() != id);
    }

    
}
