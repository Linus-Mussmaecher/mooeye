use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use ggez::audio::{SoundSource, Source};
use ggez::winit::event::VirtualKeyCode;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Rect},
    Context, GameResult,
};

/// The main struct and traits [UiElement], [UiContent], [UiContainer].
mod ui_element;
pub use ui_element::UiContainer;
pub use ui_element::UiContent;

/// Contains basic UI contents such as text and images.
/// There is nothing actually here, because the basic elements Text, Image and Empty
/// are created by simply implementing UiContent on ggez's Text and Image as well as the basic ().
pub mod basic;
/// Contains UI contents that contain other UI elements, such as vertical boxes and stack boxes.
pub mod containers;

/// Structs and functions to manage how a UI element positions and sizes itself.
mod layout;
pub use layout::Alignment;
pub use layout::Layout;
pub use layout::Size;

/// The [Visuals] structs as well as associated functions that control how an element looks.
mod visuals;
use tinyvec::TinyVec;
pub use visuals::Visuals;

/// The [Transition] struct and associated functions to control an element dynamically changing layout, visuals, content, etc.
mod transition;
pub use transition::Transition;

/// The [DrawCache] struct to remember where an element was drawn in the last frame and (if possible) simply redraw it without recalculating its position.
mod draw_cache;
use draw_cache::DrawCache;

/// The [UiMessage] struct to facilitate communcation between elements and between elements an the game state.
mod message;
pub use message::UiMessage;

/// The [UiElementBuilder] struct for simple construction of UiElements using a basic builder pattern.
mod ui_element_builder;
pub use ui_element_builder::UiElementBuilder;

/// The [UiDrawParam] struct is an extension of the [ggez::graphics::DrawParam] struct and contains some additonal information specific to UiElements.
mod ui_draw_param;
pub use ui_draw_param::UiDrawParam;

/// A UI element. The entire UI tree of mooeye is built out of these elements.
/// This wrapper struct contains all information about look, layout, tooltip, message handling, etc. of the element, while also containing one [UiContent] field that contains the actual content.
pub struct UiElement<T: Copy + Eq + Hash> {
    /// The elements layout.
    layout: Layout,
    /// The elements visuals.
    visuals: Visuals,
    /// The alternative visuals of this element, displayed while the user hovers the mouse cursor above it.
    hover_visuals: Option<Visuals>,
    /// The sound that is played whenever the element is triggered via mouse or key press.
    trigger_sound: Option<Source>,

    /// The elements ID. Not neccessarily guaranteed to be unique.
    id: u32,

    /// This elements draw cache.
    draw_cache: DrawCache,

    /// The conent managed & displayed by this element
    pub content: Box<dyn UiContent<T>>,

    /// The tooltip managed by this element, if it has one.
    tooltip: Option<Box<UiElement<T>>>,

    /// The transition queue
    transitions: VecDeque<Transition<T>>,

    /// The keyboard key triggering events on this element.
    keys: TinyVec<[Option<VirtualKeyCode>; 2]>,

    /// The message handler. This function is called on every frame to handle received message.
    /// The message handler lambda receives each frame a hash set consisting of all internal and external messages received by this element.
    /// It also receives a function pointer. Calling this pointer with a transition pushes that transition to this elements transition queue.
    /// Lastly, it receives the current layout of the element. This allows any transitions to re-use that layout and only change the variables the transition wants to change.
    message_handler: MessageHandler<T>,
}

/// The functional type of a UiElements MessageHandler.
type MessageHandler<T> = Box<dyn Fn(&HashSet<UiMessage<T>>, Layout, &mut VecDeque<Transition<T>>)>;

impl<T: Copy + Eq + Hash> std::fmt::Debug for UiElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiElement")
            .field("layout", &self.layout)
            .field("visuals", &self.visuals)
            .field("hover_visuals", &self.hover_visuals)
            .field("trigger_sound", &self.trigger_sound)
            .field("id", &self.id)
            .field("draw_cache", &self.draw_cache)
            .field("tooltip", &self.tooltip)
            .field("keys", &self.keys)
            .finish()
    }
}

impl<T: Copy + Eq + Hash> UiElement<T> {
    /// Creates a new UiElement containig the specified content and the specified ID.
    /// The element will be treated as a leaf node, even if its implements [UiContainer].
    /// ID should be as unique as you require it.
    /// Layout and visuals will be set to default values, hover_visuals is initialized as None.
    pub fn new<E: UiContent<T> + 'static>(id: u32, content: E) -> Self {
        Self {
            layout: Layout::default(),
            visuals: Visuals::default(),
            hover_visuals: None,
            trigger_sound: None,
            id,
            draw_cache: DrawCache::default(),
            content: Box::new(content),
            tooltip: None,
            transitions: VecDeque::new(),
            keys: TinyVec::new(),
            message_handler: Box::new(|_messages, _layout, _transition_queue| {}),
        }
    }

    /// Adds an element to this element (or its children), recursively searching until an element with a fitting ID is found.
    /// The element is discarded there is no container child with fitting ID.
    pub fn add_element(&mut self, id: u32, element: UiElement<T>) -> Option<UiElement<T>> {
        match self.content.container_mut() {
            Some(cont) => {
                if self.id == id {
                    cont.add(element);
                    return None;
                };

                let mut element_option = Some(element);

                for child in cont.get_children_mut().iter_mut() {
                    // check if element is still there
                    if let Some(element) = element_option {
                        // yes: try children
                        element_option = child.add_element(id, element);
                    } else {
                        // no: an element with correct id has been found in this child, propagate none up the chain
                        break;
                    }
                }

                element_option
            }
            None => Some(element),
        }
    }

    /// Removes all elements with the given ID from this element and (recursively) all its children.
    pub fn remove_elements(&mut self, id: u32) {
        if let Some(cont) = self.content.container_mut() {
            cont.remove_id(id);
            for child in cont.get_children_mut() {
                child.remove_elements(id);
            }
        }
    }

    /// Returns this elements (not neccessarily unique) ID within this UI. This ID is used to indentify the source of intern messages.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Returns this elements (current) layout.
    pub fn get_layout(&self) -> Layout {
        self.layout
    }

    /// Receives a data structure containing all messages triggered by your game_state this frame (or None if there were no messages).
    /// It then collects all messages sent by this element and its children and redistributes all of those messages to this element and all children.
    /// Returns all internal messages to act on them.
    /// In addition, if this element has children, all children whose [UiContent::expired] function returns true are removed from the container.
    pub fn update(
        &mut self,
        ctx: &ggez::Context,
        extern_messages: impl Into<Option<HashSet<UiMessage<T>>>>,
    ) -> HashSet<UiMessage<T>> {
        // Message handling

        let intern_messages = self.collect_messages(ctx);

        let all_messages = match extern_messages.into() {
            None => intern_messages.clone(),
            Some(extern_messages) => intern_messages.union(&extern_messages).copied().collect(),
        };

        self.distribute_messages(&all_messages).expect("Something went wrong delivering or executing messages. Probably you wrote a bad handler function.");

        intern_messages
    }

    /// Returns wether this element should be removed by its parents.
    pub(crate) fn expired(&self) -> bool {
        self.content.expired()
    }

    /// Deprecated version of [UiElement::update].
    pub fn manage_messages(
        &mut self,
        ctx: &ggez::Context,
        extern_messages: impl Into<Option<HashSet<UiMessage<T>>>>,
    ) -> HashSet<UiMessage<T>> {
        self.update(ctx, extern_messages)
    }

    /// Iterates over this element and all successors and collects all internal messages (clicks) sent during the last frame.
    fn collect_messages(&self, ctx: &Context) -> HashSet<UiMessage<T>> {
        let mut res: HashSet<UiMessage<T>> = HashSet::new();

        if self.id != 0
            && match self.draw_cache {
                DrawCache::Invalid => false,
                DrawCache::Valid {
                    outer,
                    inner: _,
                    target: _,
                } => outer.contains(ctx.mouse.position()),
            }
        {
            if ctx
                .mouse
                .button_just_pressed(ggez::event::MouseButton::Left)
            {
                res.insert(UiMessage::Clicked(self.id));
                res.insert(UiMessage::Triggered(self.id));
                if let Some(sound) = &self.trigger_sound {
                    if sound.play_later().is_err() && cfg!(debug_assertions) {
                        println!("[ERROR] Failed to play sound.");
                    }
                }
            }

            if ctx
                .mouse
                .button_just_pressed(ggez::event::MouseButton::Right)
            {
                res.insert(UiMessage::ClickedRight(self.id));
            }
        }

        if self.id != 0
            && self.keys.iter().any(|key_opt| {
                if let Some(key) = key_opt {
                    ctx.keyboard.is_key_just_pressed(*key)
                } else {
                    false
                }
            })
        {
            res.insert(UiMessage::PressedKey(self.id));
            res.insert(UiMessage::Triggered(self.id));
        }

        if let Some(cont) = self.content.container() {
            for child in cont.get_children() {
                res.extend(child.collect_messages(ctx));
            }
        }

        res
    }

    /// Distributes the passed set of [UiMessage]s to this element and all its successors, letting their message handlers react to the messages.
    fn distribute_messages(&mut self, messages: &HashSet<UiMessage<T>>) -> GameResult {
        (self.message_handler)(messages, self.layout, &mut self.transitions);

        if let Some(cont) = self.content.container_mut() {
            // actual distribution
            for child in cont.get_children_mut() {
                child.distribute_messages(messages)?;
            }
            // remove expired children
            cont.remove_expired();
        }

        Ok(())
    }

    /// Adds a transition to the end of the transition queue. It will be executed as soon as all transitions added beforehand have run their course.
    pub fn add_transition(&mut self, transition: Transition<T>) {
        self.transitions.push_back(transition);
    }

    /// Progresses the currently active transition by the time of the last frame.
    /// If this ends the current transition, the values of this element are updated to the values given by the transition and it is removed from the queue.
    fn progress_transitions(&mut self, ctx: &Context) {
        if !self.transitions.is_empty() && self.transitions[0].progress(ctx.time.delta()) {
            let trans = self.transitions.pop_front().expect(
                "Transitions did not contain a first element despite being not empty 2 lines ago.",
            );

            if let Some(layout) = trans.new_layout {
                self.layout = layout;
                self.draw_cache = DrawCache::Invalid;
            }
            if let Some(visuals) = trans.new_visuals {
                self.visuals = visuals;
            }
            if let Some(hover_visuals) = trans.new_hover_visuals {
                self.hover_visuals = hover_visuals;
            }
            if let Some(content) = trans.new_content {
                self.content = content;
            }
            if let Some(tooltip) = trans.new_tooltip {
                self.tooltip = tooltip;
            }
        }
    }

    /// First checks wether the user is currently hovering this element or not and chooses to return visuals or hover visuals accordingly.
    /// Then checks if the transition queue contains a (hover-)visual-changing element and returns an average visuals if needed.
    fn get_current_visual(&self, ctx: &Context, param: UiDrawParam) -> Visuals {
        // check if this element is being hovered

        if param.mouse_listen
            && match self.draw_cache {
                DrawCache::Invalid => false,
                DrawCache::Valid {
                    outer,
                    inner: _,
                    target: _,
                } => outer.contains(ctx.mouse.position()),
            }
        {
            // yes: get what this element, diregarding transitions, would display on hover
            let own_vis = if let Some(hover_visuals) = self.hover_visuals {
                hover_visuals
            } else {
                self.visuals
            };

            // check wether there are transitions in the queue
            if self.transitions.is_empty() {
                //no: just return own visuals
                own_vis
            } else {
                // yes: check wether the top transition wants to change hover_visuals
                let trans = &self.transitions[0];
                match trans.new_hover_visuals {
                    // yes: find out what it wants to display on hover and take the average
                    Some(vis) => {
                        let trans_vis = if let Some(hover_visuals) = vis {
                            hover_visuals
                        } else {
                            self.visuals
                        };
                        own_vis.average(trans_vis, trans.get_progress_ratio())
                    }
                    // no: just return own visuals
                    None => own_vis,
                }
            }
        } else {
            // not hovered: check wether there are transitons in the queue
            if self.transitions.is_empty() {
                // no transitions: just return own visuals
                self.visuals
            } else {
                // transitions: check wether the top transition wants to change visuals
                let trans = &self.transitions[0];
                match trans.new_visuals {
                    // yes: find average between the two visuals
                    Some(vis) => self.visuals.average(vis, trans.get_progress_ratio()),
                    // no: just return own visuals
                    None => self.visuals,
                }
            }
        }
    }

    /// Updates this element's draw cache by checking for validity.
    /// If the draw cache is still valid (see [UiElement::cache_valid]), nothing happens.
    /// Otherwise, the function uses ```content_min```, the ```layout``` and the currently active ```Transition``` to generate a valid draw cache
    /// If no valid draw chache can be generated, the draw_cache wil be reset to default value.
    /// The function will only change ```draw_cache::valid``` to ```true``` if the generated rectangles fit within the target ```rect```.
    fn update_draw_cache(&mut self, _ctx: &Context, target: Rect) {
        // check wether draw cache needs to be updated at all (or a transition is going on)
        if !self.cache_valid(target) {
            // first calculate the target of this element if it were on its own
            let (own_outer, own_inner) = self
                .layout
                .get_outer_inner_bounds_in_target(&target, self.content_min());
            // check if there is a transition going on
            let (outer, inner) = if !self.transitions.is_empty() {
                // the transitions are not empty: check if the top transitions wants to change the layout
                if let Some(new_layout) = self.transitions[0].new_layout {
                    let (trans_outer, trans_inner) =
                        new_layout.get_outer_inner_bounds_in_target(&target, self.content_min());
                    (
                        transition::average_rect(
                            &own_outer,
                            &trans_outer,
                            self.transitions[0].get_progress_ratio(),
                        ),
                        transition::average_rect(
                            &own_inner,
                            &trans_inner,
                            self.transitions[0].get_progress_ratio(),
                        ),
                    )
                } else {
                    (own_outer, own_inner)
                }
            } else {
                // draw cache was invalidated by some other means (e.g. by sub element having a transition, the element not being initalized, etc.) -> calculate target
                (own_outer, own_inner)
            };

            // checking bounds, adding 0.01 to deal with problems stemming from imprecise multiplication
            if outer.w > target.w + 0.01
                || outer.h > target.h + 0.01
                || outer.x < 0.
                || outer.y < 0.
            //|| outer.x + outer.w > ctx.gfx.window().inner_size().width as f32 + 0.01
            //|| outer.y + outer.h > ctx.gfx.window().inner_size().height as f32 + 0.01
            {
                if cfg!(test) {
                    println!(
                        "Skipped Element due to bounds violation. Outer: {:?}, Target: {:?}",
                        outer, target
                    );
                }
                self.draw_cache = DrawCache::Invalid;
            } else {
                self.draw_cache = DrawCache::Valid {
                    outer,
                    inner,
                    target,
                };
            }
        }
    }

    /// Returns wether this elements cache is still valid. The cache may be invalidated manually or because the target_rect has changed.
    /// Any chache is considered invalid if there is currently an active transition that is actively changing the layout
    /// In the case of containers, the cache may also be invalidated because the cache of a child element has turned invalid. The default implementation for this case can e.g. be found in the code for [VerticalBox].
    fn cache_valid(&self, target: Rect) -> bool {
        let init = match self.draw_cache {
            DrawCache::Invalid => false,
            DrawCache::Valid {
                outer: _,
                inner: _,
                target: cache_target,
            } => cache_target == target,
        } && (self.transitions.is_empty()
            || matches!(self.transitions[0].new_layout, None));
        match self.content.container() {
            Some(cont) => cont
                .get_children()
                .iter()
                .fold(init, |valid, child| valid && child.cache_valid(target)),
            None => init,
        }
    }

    /// Returns the minimum and maximum width this element this element can have. Calculated from adding left and right padding to the size-data.
    pub fn width_range(&self) -> (f32, f32) {
        let layout = self.layout;
        (
            // get min width by taking minimum of inner min width, clamping it within the bounds given by the layout and adding padding
            self.content
                .container()
                .map(|cont| cont.content_width_range())
                .unwrap_or((0., f32::INFINITY))
                .0
                .clamp(layout.x_size.min(), layout.x_size.max())
                + layout.padding.1
                + layout.padding.3,
            // get max width by adding padding, overruling inner max width
            layout.x_size.max() + layout.padding.1 + layout.padding.3,
        )
    }

    /// Returns the minimum and maximum height this element this element can have. Calculated from adding top and bottom padding to the size-data.
    pub fn height_range(&self) -> (f32, f32) {
        let layout = self.layout;
        (
            // get min width by taking minimum of inner min width, clamping it within the bounds given by the layout and adding padding
            self.content
                .container()
                .map(|cont| cont.content_height_range())
                .unwrap_or((0., f32::INFINITY))
                .0
                .clamp(layout.y_size.min(), layout.y_size.max())
                + layout.padding.0
                + layout.padding.2,
            // get max width by adding padding, overruling inner max width
            layout.y_size.max() + layout.padding.0 + layout.padding.2,
        )
    }

    /// Returns the minimum size required by the content of this element.
    fn content_min(&self) -> Vec2 {
        Vec2 {
            x: self
                .content
                .container()
                .map(|cont| cont.content_width_range().0)
                .unwrap_or_default(),
            y: self
                .content
                .container()
                .map(|cont| cont.content_height_range().0)
                .unwrap_or_default(),
        }
    }

    /// Takes in a rectangle target, a canvas, a context and draws the UiElement to that rectangle within that canvas using that context.
    /// The element will either completely fit within the rectangle (including its padding) or not be drawn at all.
    /// The element will align and offset itself within the rectangle.
    pub(crate) fn draw_to_rectangle(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        param: UiDrawParam,
    ) {
        self.progress_transitions(ctx);

        // update draw_cache
        self.update_draw_cache(ctx, param.target);

        // if draw chache is still invalid, early return and try again next frame

        let (outer, inner) = match self.draw_cache {
            DrawCache::Invalid => return,
            DrawCache::Valid {
                outer,
                inner,
                target: _,
            } => (outer, inner),
        };

        // draw visuals
        self.get_current_visual(ctx, param)
            .draw(ctx, canvas, param.target(outer));

        // draw content

        self.content.draw_content(ctx, canvas, param.target(inner));

        // draw tooltip
        if param.mouse_listen && outer.contains(ctx.mouse.position()) {
            if let Some(tt) = &mut self.tooltip {
                // get relevant positions
                let mouse_pos = ctx.mouse.position();
                let screen_size = ctx.gfx.window().inner_size();
                let tt_size = (tt.width_range().0, tt.height_range().0);

                // check if element center is left or right on the screen
                let x = if 2. * inner.x + inner.w > screen_size.width as f32 {
                    mouse_pos.x - tt_size.0 - 10.
                } else {
                    mouse_pos.x + 10.
                }
                .clamp(0., screen_size.width as f32 - tt_size.0);

                // check if element is on the top or bottom of the screen
                let y = (if 2. * inner.y + inner.h > screen_size.height as f32 {
                    mouse_pos.y - tt_size.1
                } else {
                    mouse_pos.y
                } - 10.)
                    .max(0.);

                // draw the tooltip
                tt.draw_to_rectangle(
                    ctx,
                    canvas,
                    param
                        .target(Rect::new(x, y, tt_size.0, tt_size.1))
                        .z_level(param.param.z + 1),
                );
            }
        }
    }

    /// Draws this UiElement to the current screen. Call this on your root element every frame.
    pub fn draw_to_screen(&mut self, ctx: &mut Context, canvas: &mut Canvas, mouse_listen: bool) {
        self.draw_to_rectangle(
            ctx,
            canvas,
            UiDrawParam::default()
                .target(Rect::new(
                    0.,
                    0.,
                    ctx.gfx.window().inner_size().width as f32,
                    ctx.gfx.window().inner_size().height as f32,
                ))
                .mouse_listen(mouse_listen),
        );
    }
}
