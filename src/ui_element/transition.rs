use std::time::Duration;

use super::{Layout, Visuals};



#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Transition {
    target_layout: Layout,
    target_visuals: Visuals,
    total_duration: Duration,
    remaining_duration: Duration,
}

#[allow(dead_code)]
impl Transition {
    pub fn new(target_layout: Layout, target_visuals: Visuals, duration: Duration) -> Self {
        Self {
            target_layout,
            target_visuals,
            total_duration: duration,
            remaining_duration: duration,
        }
    }
}