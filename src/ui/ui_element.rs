use std::hash::Hash;

use super::*;

use good_web_game::{event::GraphicsContext, Context};

/// A trait that marks any struct that can be the content of a UI element. Should not be used directly, only when wrapped in such an element.
/// ### Basic elements
/// For basic elements, most default implementations will suffice, and only [UiContent::draw_content] needs to be implemented.
/// If your element has special default layout requirements, you can overwrite the [UiContent::to_element_builder] constructor function.
pub trait UiContent<T: Copy + Eq + Hash> {
    /// Wraps the content into a [UiElementBuilder] and returns the builder.
    /// Use ID 0 iff you do not want this element to send any messages by itself.
    /// Overwrite this if your element should use special defaults.
    /// Context may be passed in if some elements (image element, text element) need to use context to measure themselves.
    fn to_element_builder(self, id: u32, _ctx: &Context) -> UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        UiElementBuilder::new(id, self)
    }

    /// A shorthand for creating an element builder and immediately building an element. Useful only if you do not want to diverge from any default layouts/visuals.
    fn to_element(self, id: u32, ctx: &Context) -> UiElement<T>
    where
        Self: Sized + 'static,
    {
        self.to_element_builder(id, ctx).build()
    }

    /// Takes in a rectangle target, a canvas, a context and draws the contents (not the border etc.) to that rectangle within that canvas using that context.
    /// Normally, this will only be called from within private functions, when the cache has been modified appropiately and only use the inner rectangle of the draw cache as content_bounds.
    /// Do not call otherwise.
    fn draw_content(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
        param: UiDrawParam,
    );

    /// Returns a bool value. Returning true indicates to any container this element is a child of that this element wishes to be removed from the container (and discarded).
    fn expired(&self) -> bool {
        false
    }

    /// Returns an immutable reference to Self (cast to a container) if this element also implements [UiContainer].
    /// If it does not, returns None.
    /// Remember to overwrite this function for all of your custom containers!
    fn container(&self) -> Option<&dyn UiContainer<T>> {
        None
    }

    /// Returns a mutable reference to Self (cast to a container) if this element also implements [UiContainer].
    /// If it does not, returns None.
    /// Remember to overwrite this function for all of your custom containers!
    fn container_mut(&mut self) -> Option<&mut dyn UiContainer<T>> {
        None
    }
}

/// This trait marks a special type of UiContent that contains other UiElements.
/// Remember to overwrite the [UiContent::container] and [UiContent::container_mut] functions of [UiContent].
pub trait UiContainer<T: Copy + Eq + Hash>: UiContent<T> {
    /// Returns any dynamic width restrictions induced by the content, not the layout. Usually, this refers to the layout of child elements of containers.
    /// Default implementation returns (0., infinty) (no restrictions).
    fn content_width_range(&self) -> (f32, f32) {
        (0., f32::INFINITY)
    }

    /// Returns any dynamic width restrictions induced by the content, not the layout. Usually, this refers to the layout of child elements of containers.
    /// Default implementation returns (0., infinty) (no restrictions).
    fn content_height_range(&self) -> (f32, f32) {
        (0., f32::INFINITY)
    }

    /// Returns access to this elements children, if there are any. Returns None if this is a leaf node.
    fn get_children(&self) -> &[UiElement<T>];

    /// Returns mutatble access to this elements children, if there are any. Returns None if this is a leaf node.
    fn get_children_mut(&mut self) -> &mut [UiElement<T>];

    /// Attempts to add a UiElement to this elements children.
    fn add(&mut self, element: UiElement<T>);

    /// Removes all elements from this container whose [UiContent::expired]-function returns true.
    fn remove_expired(&mut self);

    /// Removes all elements from this container whose ids match.
    fn remove_id(&mut self, id: u32);
}
