use crate::ui;
use std::{hash::Hash, slice, time::Duration};

/// A Box that will display a single elements and serves as a wrapper to that element.
/// Also keeps a [Duration] attribute and marks itself as expired after that duration.
pub struct DurationBox<T: Copy + Eq + Hash> {
    /// Contains the UiElements within this box in the right order (front to back).
    child: ui::UiElement<T>,
    /// How long this element has left to live.
    duration: Duration,
}

impl<T: Copy + Eq + Hash> DurationBox<T> {
    /// Creates a new [DurationBox] with the initial duration and child element.
    pub fn new(duration: Duration, element: ui::UiElement<T>) -> Self {
        Self {
            child: element,
            duration,
        }
    }
}
impl<T: Copy + Eq + Hash> ui::UiContent<T> for DurationBox<T> {
    fn to_element_builder(self, id: u32, _ctx: &ggez::Context) -> ui::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        ui::UiElementBuilder::new(id, self)
            .as_shrink()
            .with_padding((0., 0., 0., 0.))
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        param: ui::UiDrawParam,
    ) {
        self.duration = self.duration.saturating_sub(ctx.time.delta());
        self.child.draw_to_rectangle(ctx, canvas, param);
    }

    fn expired(&self) -> bool {
        self.duration.is_zero()
    }

    fn container(&self) -> Option<&dyn ui::UiContainer<T>> {
        Some(self)
    }

    fn container_mut(&mut self) -> Option<&mut dyn ui::UiContainer<T>> {
        Some(self)
    }
}
impl<T: Copy + Eq + Hash> ui::UiContainer<T> for DurationBox<T> {
    fn content_width_range(&self) -> (f32, f32) {
        self.child.width_range()
    }

    fn content_height_range(&self) -> (f32, f32) {
        self.child.height_range()
    }

    fn get_children(&self) -> &[ui::UiElement<T>] {
        slice::from_ref(&self.child)
    }

    fn get_children_mut(&mut self) -> &mut [ui::UiElement<T>] {
        slice::from_mut(&mut self.child)
    }

    fn add(&mut self, element: ui::UiElement<T>) {
        self.child = element;
    }

    fn remove_expired(&mut self) {
        if self.child.expired() {
            self.child = ui::UiElement::new(0, ());
        }
    }

    fn remove_id(&mut self, id: u32) {
        if self.child.get_id() == id {
            self.child = ui::UiElement::new(0, ());
        }
    }
}
