use ggez::graphics::{Rect, DrawParam};


#[derive(Copy, Clone)]
pub struct UiDrawParam{
    pub target: Rect,
    pub mouse_listen: bool,
    pub param: DrawParam,
}

impl UiDrawParam{
    pub fn new() -> Self{
        Self { target: Rect::default(), mouse_listen: true, param: DrawParam::new() }
    }

    pub fn target(self, target: Rect) -> Self{
        Self{
            target: target,
            ..self
        }
    }

    pub fn mouse_listen(self, mouse_listen: bool) -> Self{
        Self{
            mouse_listen: mouse_listen,
            ..self
        }
    }

    pub fn z_level(self, z_level: i32) -> Self{
        Self{
            param: self.param.z(z_level),
            ..self
        }
    }
}

impl Default for UiDrawParam{
    fn default() -> Self {
        Self { target: Default::default(), mouse_listen: Default::default(), param: Default::default() }
    }
}

impl From<DrawParam> for UiDrawParam {
    fn from(value: DrawParam) -> Self {
        Self { target: Rect::default(), mouse_listen: true, param: value }
    }
}

impl From<UiDrawParam> for DrawParam{
    fn from(value: UiDrawParam) -> Self {
        value.param
    }
}