use ggez::{audio::Source, winit::event::VirtualKeyCode};

use crate::{UiContent, UiElement};
use std::hash::Hash;

use super::Layout;

/// A builder struct for UiElements. Allows changing of all relevant fields of the built element, and contains shorthand function for changing the components of the elements layout.
/// Also contains shorthand functions for some very frequently used combination of layout settings.
#[derive(Debug)]
pub struct UiElementBuilder<T: Copy + Eq + Hash> {
    element: UiElement<T>,
}

impl<T: Copy + Eq + Hash> UiElementBuilder<T> {
    /// Creates a new builder building an element containing the specified content with the specified id.
    /// If built directly, all fields except content will be set to their default values or the values set in the to_element function of the passed con.
    pub fn new(id: u32, content: impl UiContent<T> + 'static) -> Self {
        Self {
            element: UiElement::new(id, content),
        }
    }

    /// If the elements content is a container, the passed element is added to it.
    /// Otherwise, the passed element is discarded.
    pub fn with_child(mut self, element: UiElement<T>) -> Self {
        if let Some(cont) = self.element.content.container_mut() {
            cont.add(element);
        }
        self
    }

    /// Sets the elements visuals.
    pub fn with_visuals(mut self, visuals: super::Visuals) -> Self {
        self.element.visuals = visuals;
        self
    }

    /// Sets the elements hover_visuals. Pass in None to delete any existing hover_visuals.
    pub fn with_hover_visuals(mut self, hover_visuals: impl Into<Option<super::Visuals>>) -> Self {
        self.element.hover_visuals = hover_visuals.into();
        self
    }

    /// Sets a sound to be played whenever this element is triggered via key press or mouse click.
    pub fn with_trigger_sound(mut self, trigger_sound: impl Into<Option<Source>>) -> Self {
        self.element.trigger_sound = trigger_sound.into();
        self
    }

    /// Sets the elements tooltip to the specified UiContent (or disables any tooltip by passing None).
    /// Tooltips are displayed when hovering over an element with the mouse cursor.
    /// Most specific alignment and sizes of tooltips will be ignored. Tooltips will always be shrink in both dimensions and align as close to the cursor as possible.
    pub fn with_tooltip(mut self, tooltip: impl Into<Option<UiElement<T>>>) -> Self {
        match tooltip.into() {
            None => self.element.tooltip = None,
            Some(mut tt) => {
                tt.layout.x_size = tt.layout.x_size.to_shrink();
                tt.layout.y_size = tt.layout.y_size.to_shrink();
                self.element.tooltip = Some(Box::new(tt))
            }
        }
        self
    }

    /// Sets the elements message handler.
    /// The message handler lambda receives each frame a hash set consisting of all internal and external messages received by this element.
    /// It also receives a function pointer. Calling this pointer with a transition pushes that transition to this elements transition queue.
    /// Lastly, it receives the current layout of the element. This allows any transitions to re-use that layout and only change the variables the transition wants to change.
    pub fn with_message_handler(
        mut self,
        handler: impl Fn(
                &std::collections::HashSet<crate::UiMessage<T>>,
                Layout,
                &mut std::collections::VecDeque<super::Transition<T>>,
            ) + 'static,
    ) -> Self {
        self.element.message_handler = Box::new(handler);
        self
    }

    /// Sets the elements entire layout.
    pub fn with_layout(mut self, layout: super::Layout) -> Self {
        self.element.layout = layout;
        self
    }

    /// Sets only the offset values of the elements layout. Pass None in any argument to leave that offset as-is.
    pub fn with_offset(
        mut self,
        x_offset: impl Into<Option<f32>>,
        y_offset: impl Into<Option<f32>>,
    ) -> Self {
        match x_offset.into() {
            None => {}
            Some(offset) => self.element.layout.x_offset = offset,
        };

        match y_offset.into() {
            None => {}
            Some(offset) => self.element.layout.y_offset = offset,
        };

        self
    }

    /// Sets only the alingment of the elements layout. Pass None in any argument to leave that alignment as-is.
    pub fn with_alignment(
        mut self,
        x_alignment: impl Into<Option<super::Alignment>>,
        y_alignment: impl Into<Option<super::Alignment>>,
    ) -> Self {
        match x_alignment.into() {
            None => {}
            Some(alignment) => self.element.layout.x_alignment = alignment,
        };

        match y_alignment.into() {
            None => {}
            Some(alignment) => self.element.layout.y_alignment = alignment,
        };

        self
    }

    /// Attaches a key code to this element. Pressing this key will send the same trigger event as clicking the element.
    pub fn with_trigger_key(mut self, key: VirtualKeyCode) -> Self {
        self.element.keys.push(Some(key));
        self
    }

    /// Sets only the padding of the elements layout.
    pub fn with_padding(mut self, padding: (f32, f32, f32, f32)) -> Self {
        self.element.layout.padding = padding;
        self
    }

    /// Sets only the presever_ratio parameter of the elements layout.
    pub fn with_preserve_ratio(mut self, preserve_ratio: bool) -> Self {
        self.element.layout.preserve_ratio = preserve_ratio;
        self
    }

    /// Sets only the size of the elements layout. Pass None in any argument to leave that size as-is.
    pub fn with_size(
        mut self,
        x_size: impl Into<Option<super::Size>>,
        y_size: impl Into<Option<super::Size>>,
    ) -> Self {
        match x_size.into() {
            None => {}
            Some(size) => self.element.layout.x_size = size,
        };

        match y_size.into() {
            None => {}
            Some(size) => self.element.layout.y_size = size,
        };

        self
    }

    /// Changes both sizes of the element to SHRINK, taking any boundaries from previous size.
    pub fn as_shrink(mut self) -> Self {
        self.element.layout.x_size = self.element.layout.x_size.to_shrink();
        self.element.layout.y_size = self.element.layout.y_size.to_shrink();
        self
    }

    /// Changes both sizes of the element to FILL, taking any boundaries from previous size.
    pub fn as_fill(mut self) -> Self {
        self.element.layout.x_size = self.element.layout.x_size.to_fill();
        self.element.layout.y_size = self.element.layout.y_size.to_fill();
        self
    }

    /// Scales any boundaries of the sizes of this element by the respective factor. Pass in None or 1. to not scale any dimension.
    pub fn scaled(
        mut self,
        x_scale: impl Into<Option<f32>>,
        y_scale: impl Into<Option<f32>>,
    ) -> Self {
        match x_scale.into() {
            None => {}
            Some(scale) => self.element.layout.x_size = self.element.layout.x_size.scale(scale),
        };

        match y_scale.into() {
            None => {}
            Some(scale) => self.element.layout.y_size = self.element.layout.y_size.scale(scale),
        };

        self
    }

    /// Takes in a layout and sets the elements layout to be as you would want for a container wrapping the passed layout.
    /// Sets size to fill, taking boundaries from the passed layout + padding, and own padding to 0.
    pub fn with_wrapper_layout(self, wrapped_layout: Layout) -> Self {
        self.with_size(
            super::Size::Fill(
                wrapped_layout.x_size.min() + wrapped_layout.padding.1 + wrapped_layout.padding.3,
                f32::INFINITY,
            ),
            super::Size::Fill(
                wrapped_layout.y_size.min() + wrapped_layout.padding.0 + wrapped_layout.padding.2,
                f32::INFINITY,
            ),
        )
        .with_padding((0., 0., 0., 0.))
    }

    /// Returns the underlying built element.
    pub fn build(self) -> UiElement<T> {
        self.element
    }
}

impl<T: Copy + Eq + Hash> From<UiElement<T>> for UiElementBuilder<T> {
    fn from(value: UiElement<T>) -> Self {
        Self { element: value }
    }
}

impl<T: Copy + Eq + Hash> From<UiElementBuilder<T>> for UiElement<T> {
    fn from(value: UiElementBuilder<T>) -> Self {
        value.element
    }
}
