use std::{collections::VecDeque, time::Duration};

use ggez::{glam::Vec2, graphics::{Rect, Color}, Context};

use super::{Layout, Visuals};

#[allow(dead_code)]
pub struct TransitionManager {
    transitions: VecDeque<(Layout, Visuals, Duration, Duration)>,
}

#[allow(dead_code)]
impl TransitionManager {
    pub fn new() -> Self {
        Self {
            transitions: VecDeque::new(),
        }
    }

    pub fn add_transition(
        &mut self,
        target_layout: Layout,
        target_visuals: Visuals,
        duration: Duration,
    ) {
        self.transitions
            .push_back((target_layout, target_visuals, duration, duration));
    }

    pub(crate) fn current_rect(
        &self,
        base_layout: &Layout,
        content_min: Vec2,
        target: &Rect,
    ) -> (Rect, Rect) {
        if self.transitions.is_empty() {
            base_layout.get_outer_inner_bounds_in_target(target, content_min)
        } else {
            let (base_outer, base_inner) =
                base_layout.get_outer_inner_bounds_in_target(target, content_min);
            let my_trans = self.transitions.get(0).unwrap();
            let (my_inner, my_outer) = my_trans
                .0
                .get_outer_inner_bounds_in_target(target, content_min);
            let factor = my_trans.2.as_millis() as f32 / my_trans.3.as_millis() as f32;

            (
                Rect {
                    x: base_outer.x * (1. - factor) + my_outer.x * factor,
                    y: base_outer.y * (1. - factor) + my_outer.y * factor,
                    w: base_outer.w * (1. - factor) + my_outer.w * factor,
                    h: base_outer.h * (1. - factor) + my_outer.h * factor,
                },
                Rect {
                    x: base_inner.x * (1. - factor) + my_inner.x * factor,
                    y: base_inner.y * (1. - factor) + my_inner.y * factor,
                    w: base_inner.w * (1. - factor) + my_inner.w * factor,
                    h: base_inner.h * (1. - factor) + my_inner.h * factor,
                },
            )
        }
    }

    pub(crate) fn current_visuals(&self, base_visuals: &Visuals) -> Visuals {
        if self.transitions.is_empty() {
            *base_visuals
        } else {
            let my_trans = self.transitions.get(0).unwrap();
            let my_vis = my_trans.1;
            let factor = my_trans.2.as_millis() as f32 / my_trans.3.as_millis() as f32;

            Visuals {
                background: Color::from_rgba(
                    (base_visuals.background.r * (1. - factor) + my_vis.background.r * factor)
                        as u8,
                    (base_visuals.background.g * (1. - factor) + my_vis.background.g * factor)
                        as u8,
                    (base_visuals.background.b * (1. - factor) + my_vis.background.b * factor)
                        as u8,
                    (base_visuals.background.a * (1. - factor) + my_vis.background.a * factor)
                        as u8,
                ),
                border: Color::from_rgba(
                    (base_visuals.border.r * (1. - factor) + my_vis.border.r * factor) as u8,
                    (base_visuals.border.g * (1. - factor) + my_vis.border.g * factor) as u8,
                    (base_visuals.border.b * (1. - factor) + my_vis.border.b * factor) as u8,
                    (base_visuals.border.a * (1. - factor) + my_vis.border.a * factor) as u8,
                ),
                border_width: base_visuals.border_width * (1. - factor)
                    + my_vis.border_width * factor,
            }
        }
    }

    pub(crate) fn switch_layout(&mut self, ctx: Context) -> Option<(Layout, Visuals)> {
        if self.transitions.is_empty() {
            None
        } else {
            let my_trans = self.transitions.get_mut(0).unwrap();
            my_trans.2 += ctx.time.delta();
            if my_trans.2 >= my_trans.3 {
                let res = Some((my_trans.0, my_trans.1));
                self.transitions.pop_front();
                res
            }else{
                None
            }
        }
    }
}