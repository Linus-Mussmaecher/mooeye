use ggez::GameResult;

use crate::{UiContent, UiElement};
use std::{hash::Hash, time::Duration, slice};

/// A ox that will display a single elements and serves as a wrapper to that element.
/// Also keeps a [Duration] attribute and marks itself as expired after that duration.
pub struct DurationBox<T: Copy + Eq + Hash> {
    /// Contains the UiElements within this box in the right order (front to back).
    child: UiElement<T>,
    /// How long this element has left to live.
    duration: Duration,
}

impl<T: Copy + Eq + Hash> DurationBox<T> {
    pub fn new(duration: Duration, element: UiElement<T>) -> Self {
        Self {
            child: element,
            duration,
        }
    }
}
impl<T: Copy + Eq + Hash> UiContent<T> for DurationBox<T> {
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
        self.child.width_range()
    }

    fn content_height_range(&self) -> (f32, f32) {
        self.child.height_range()
    }

    fn get_children(&self) -> Option<&[UiElement<T>]> {
        Some(slice::from_ref(&self.child))
    }

    fn get_children_mut(&mut self) -> Option<&mut [UiElement<T>]> {
        Some(slice::from_mut(&mut self.child))
    }

    fn add(&mut self, element: UiElement<T>) -> GameResult {
        self.child = element;
        Ok(())
    }

    fn remove_expired(&mut self) -> GameResult {
        if self.child.expired() {
            self.child = UiElement::new(0, ());
        }
        Ok(())
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        param: crate::ui_element::UiDrawParam,
    ) {
        self.duration = self.duration.saturating_sub(ctx.time.delta());
        self.child.draw_to_rectangle(ctx, canvas, param);
    }

    fn expired(&self) -> bool {
        self.duration.is_zero()
    }
}
