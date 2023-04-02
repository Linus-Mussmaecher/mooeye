use std::{time::Duration, hash::Hash};

use ggez::graphics::Rect;

use crate::UiContent;

use super::{Layout, Visuals};

pub struct Transition<T: Copy + Eq + Hash> {
    pub(crate) new_layout: Option<Layout>,
    pub(crate) new_visuals: Option<Visuals>,
    pub(crate) new_hover_visuals: Option<Option<Visuals>>,
    pub(crate) new_content: Option<Box<dyn UiContent<T>>>,
    pub(crate) total_duration: Duration,
    pub(crate) remaining_duration: Duration,
}

impl<T: Copy + Eq + Hash> Transition<T> {
    pub fn new(duration: Duration) -> Self {
        Self {
            new_layout: None,
            new_visuals: None,
            new_hover_visuals: None,
            new_content: None,
            total_duration: duration,
            remaining_duration: duration,
        }
    }

    pub fn with_new_layout(mut self, new_layout: Layout) -> Self{
        self.new_layout = Some(new_layout);
        self
    }

    pub fn with_new_visuals(mut self, new_visuals: Visuals) -> Self{
        self.new_visuals = Some(new_visuals);
        self
    }

    pub fn with_new_hover_visuals(mut self, new_hover_visuals: Option<Visuals>) -> Self{
        self.new_hover_visuals = Some(new_hover_visuals);
        self
    }

    pub fn with_new_content<E>(mut self, new_content: E) -> Self
    where E: UiContent<T> + 'static{
        self.new_content = Some(Box::new(new_content));
        self
    }

    pub fn get_progress_ratio(&self) -> f32{
        1. - self.remaining_duration.as_secs_f32() / self.total_duration.as_secs_f32()
    }
}

pub(crate) fn average_rect(rect1: &Rect, rect2: &Rect, ratio: f32) -> Rect{
    Rect {
        x: rect1.x * (1. - ratio) + rect2.x * ratio,
        y: rect1.y * (1. - ratio) + rect2.y * ratio,
        w: rect1.w * (1. - ratio) + rect2.w * ratio,
        h: rect1.h * (1. - ratio) + rect2.h * ratio,
    }
}