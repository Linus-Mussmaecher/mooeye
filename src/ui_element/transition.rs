use std::{hash::Hash, time::Duration};

use ggez::graphics::Rect;

use crate::{UiContent, UiElement};

use super::{Layout, Visuals};


/// A Transition stuct that can be added to an UiElement to slowly change that elements properties over time.
/// A transition can change the elements layout, visuals, hover_visuals, content and tooltip by first augmenting the transition with the relevant methods.
pub struct Transition<T: Copy + Eq + Hash> {
    /// The layout transitioned to.
    pub(crate) new_layout: Option<Layout>,
    /// The visuals transitioned to.
    pub(crate) new_visuals: Option<Visuals>,
    /// The hover visuals transitioned to.
    pub(crate) new_hover_visuals: Option<Option<Visuals>>,
    /// The content transitioned to.
    pub(crate) new_content: Option<Box<dyn UiContent<T>>>,
    /// The tooltip transitioned to.
    pub(crate) new_tooltip: Option<Option<Box<UiElement<T>>>>,

    total_duration: Duration,
    progressed_duration: Duration,
}

impl<T: Copy + Eq + Hash> Transition<T> {

    /// Creates a new transition with the specified duration and all possible augmentations set to none. Can be used without adding changes to delay transitions added later.
    pub fn new(duration: Duration) -> Self {
        Self {
            new_layout: None,
            new_visuals: None,
            new_hover_visuals: None,
            new_content: None,
            new_tooltip: None,

            total_duration: duration,
            progressed_duration: Duration::ZERO,
        }
    }

    /// Augments this transition to now (gradually) change the layout of the UiElement it is added to, moving it smoothly between the two positions.
    pub fn with_new_layout(mut self, new_layout: Layout) -> Self {
        self.new_layout = Some(new_layout);
        self
    }

    /// Augments the transition to now (gradually) change the visuals of the UiElement it is added to, blending over smoothly.
    pub fn with_new_visuals(mut self, new_visuals: Visuals) -> Self {
        self.new_visuals = Some(new_visuals);
        self
    }

    /// Augments this transition to now (gradually) change the visuals of the UiElement it is added to when hovered, blending over smoothly.
    /// Can be set to None to remove any special visuals when hovered. In this case, the transition will make the elements hover visuals slowly blend to the elements non-hover visuals.
    pub fn with_new_hover_visuals(mut self, new_hover_visuals: Option<Visuals>) -> Self {
        self.new_hover_visuals = Some(new_hover_visuals);
        self
    }

    /// Augment this transition to now change the content of the UiElement it is added to. This change happens in a single frame as soon as the transitions duration has elapsed.
    pub fn with_new_content<E>(mut self, new_content: E) -> Self
    where
        E: UiContent<T> + 'static,
    {
        self.new_content = Some(Box::new(new_content));
        self
    }

    /// Augment this transition to now change the tooltip of the UiElement it is added to. This change happens in a single frame as soon as the transitions duration has elapsed.
    pub fn with_new_tooltip(mut self, new_tooltip: Option<UiElement<T>>) -> Self {
        self.new_tooltip = match new_tooltip {
            Some(element) => Some(Some(Box::new(element))),
            None => None,
        };
        self
    }

    /// Progresses the internal timer of this transition by the specified amount. Returns true if the Transition is now complete and false otherwise.
    pub fn progress(&mut self, delta: Duration) -> bool{
        self.progressed_duration += delta;
        self.progressed_duration >= self.total_duration
    }

    /// Returns a float in [0;1] describung how much of this transitions total duration has elapsed already.
    pub fn get_progress_ratio(&self) -> f32 {
        self.progressed_duration.as_secs_f32() / self.total_duration.as_secs_f32()
    }
}


/// Returns the average of two rectangles (all four values are averaged). rect1 is weighted by (1-ratio), while rect2 is weighted by ratio.
/// Thus, ratio=0 returns rect1 and ratio=1 returns rect2. The progression between the two is linear and continuous.
pub(crate) fn average_rect(rect1: &Rect, rect2: &Rect, ratio: f32) -> Rect {
    Rect {
        x: rect1.x * (1. - ratio) + rect2.x * ratio,
        y: rect1.y * (1. - ratio) + rect2.y * ratio,
        w: rect1.w * (1. - ratio) + rect2.w * ratio,
        h: rect1.h * (1. - ratio) + rect2.h * ratio,
    }
}
