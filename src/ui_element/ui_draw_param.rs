use ggez::graphics::{DrawParam, Rect};

/// An extension of the [ggez::graphics::DrawParam] struct specifically for UiElements.
#[derive(Copy, Clone, Debug)]
pub struct UiDrawParam {
    /// The rectangular target area this element shall be drawn to.
    pub target: Rect,
    /// Wether or not the element should listen to the mouse position and possible change its visuals or display a tooltip when hovered over.
    pub mouse_listen: bool,
    /// A basic draw param struct to manage things like z-level, color and src-rect.
    /// Setting dest_rect may yield unexpected behaviour, as it will likely be overwritten by target when drawing.
    pub param: DrawParam,
}

impl UiDrawParam {
    /// Creates a new [UiDrawParam] with default values.
    pub fn new() -> Self {
        Self {
            target: Rect::default(),
            mouse_listen: true,
            param: DrawParam::new(),
        }
    }

    /// Returns a new [UiDrawParam] with the specified target area.
    pub fn target(self, target: Rect) -> Self {
        Self {
            target: target,
            ..self
        }
    }

    /// Returns a new [UiDrawParam] with the specified mouse_listen value.
    pub fn mouse_listen(self, mouse_listen: bool) -> Self {
        Self {
            mouse_listen: mouse_listen,
            ..self
        }
    }

    /// Returns a new [UiDrawParam] with only the z value of the contained param set to the specified value.
    pub fn z_level(self, z_level: i32) -> Self {
        Self {
            param: self.param.z(z_level),
            ..self
        }
    }

    /// Returns a new [UiDrawParam] with the entire DrawParam replaced by the specified value.
    pub fn param(self, param: DrawParam) -> Self {
        Self {
            param: param,
            ..self
        }
    }
}

impl Default for UiDrawParam {
    fn default() -> Self {
        Self {
            target: Default::default(),
            mouse_listen: Default::default(),
            param: Default::default(),
        }
    }
}

impl From<DrawParam> for UiDrawParam {
    fn from(value: DrawParam) -> Self {
        Self {
            target: Rect::default(),
            mouse_listen: true,
            param: value,
        }
    }
}

impl From<UiDrawParam> for DrawParam {
    fn from(value: UiDrawParam) -> Self {
        value.param
    }
}
