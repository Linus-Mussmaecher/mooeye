use std::collections::{HashSet};

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Rect},
    Context, GameResult,
};

pub mod layout;
pub use layout::Layout;

pub mod visuals;
pub use visuals::Visuals;

pub mod transition;
pub use transition::Transition;

pub mod draw_cache;
pub use draw_cache::DrawCache;

pub mod message;
pub use message::UiMessage;

pub struct UiElement {
    /// The elements layout.
    pub layout: Layout,
    /// The elements visuals.
    pub visuals: Visuals,
    /// The alternative visuals of this element, displayed while the user hovers the mouse cursor above it.
    pub hover_visuals: Option<Visuals>,

    /// The elements ID. Not neccessarily guaranteed to be unique.
    id: u32,

    /// This elements draw cache.
    pub(crate) draw_cache: DrawCache,

    content: Box<dyn UiContent>,
}

impl UiElement {
    pub fn new<E: UiContent + 'static>(id: u32, content: E) -> Self {
        Self {
            layout: Layout::default(),
            visuals: Visuals::default(),
            hover_visuals: None,
            id,
            draw_cache: DrawCache::default(),
            content: Box::new(content),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Returns wether this elements cache is still valid. The cache may be invalidated manually or because the target_rect has changed.
    /// In the case of containers, the cache may also be invalidated because the cache of a child element has turned invalid. The default implementation for this case can e.g. be found in the code for [VerticalBox].
    pub fn cache_valid(&self, target: &Rect) -> bool {
        self.content.get_children().unwrap_or(&[]).iter().fold(
            self.draw_cache.valid && *target == self.draw_cache.target,
            |valid, child| valid && child.cache_valid(target),
        )
    }

    /// Receives a data structure containing all messages triggered by your game_state this frame.
    /// It then collects all messages sent by this element and its children and redistributes all of those messages to this element and all children.
    /// Returns all internal messages to act on them
    pub fn manage_messages(&self, ctx: &ggez::Context, extern_messages: &HashSet<UiMessage<()>>) -> HashSet<UiMessage<()>> {
        
        let intern_messages = self.collect_messages(ctx);

        let all_messages = intern_messages.union(extern_messages).copied().collect();

        self.distribute_messages(ctx, &all_messages).expect("Something went wrong delivering or executing messages. Probably you wrote a bad handler function.");

        intern_messages
    }

    fn collect_messages(&self, ctx: &Context) -> HashSet<UiMessage<()>>{
        let mut res: HashSet<UiMessage<()>> = HashSet::new();

        if self.draw_cache.outer.contains(ctx.mouse.position()){
            if ctx.mouse.button_just_pressed(ggez::event::MouseButton::Left){
                res.insert(UiMessage::Clicked(self.id));
            }

            if ctx.mouse.button_just_pressed(ggez::event::MouseButton::Right){
                res.insert(UiMessage::ClickedRight(self.id));
            }
        }

        if let Some(children) = self.content.get_children(){
            for child in children {
                res.extend(child.collect_messages(ctx));
            }
        }

        res
    }

    fn distribute_messages(&self, ctx: &Context, messages: &HashSet<UiMessage<()>>) -> GameResult{
        //TODO: Do something with those messages
        
        if let Some(children) = self.content.get_children(){
            for child in children {
                child.distribute_messages(ctx, messages)?;
            }
        }

        Ok(())
    }

    /// Returns the minimum and maximum width this element this element can have. Calculated from adding left and right padding to the size-data.
    pub fn width_range(&self) -> (f32, f32) {
        let layout = self.layout;
        (
            // get min width by taking minimum of inner min width, clamping it within the bounds given by the layout and adding padding
            self.content
                .content_width_range()
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
                .content_height_range()
                .0
                .clamp(layout.y_size.min(), layout.y_size.max())
                + layout.padding.0
                + layout.padding.2,
            // get max width by adding padding, overruling inner max width
            layout.y_size.max() + layout.padding.0 + layout.padding.2,
        )
    }

    pub fn content_min(&self) -> Vec2 {
        Vec2 {
            x: self.content.content_width_range().0,
            y: self.content.content_height_range().0,
        }
    }

    /// Takes in a rectangle target, a canvas, a context and draws the UiElement to that rectangle within that canvas using that context.
    /// The element will either completely fit within the rectangle (including its padding) or not be drawn at all.
    /// The element will align and offset itself within the rectangle.
    pub fn draw_to_rectangle(&mut self, ctx: &mut Context, canvas: &mut Canvas, rect: Rect) {
        // if cache is invalidated or we are drawing to a differen target than before, update cache
        if !self.cache_valid(&rect) {
            // calculate actual size and update cache

            let (outer, inner) = self
                .layout
                .get_outer_inner_bounds_in_target(&rect, self.content_min());
            self.draw_cache = DrawCache {
                outer: outer,
                inner: inner,
                target: rect,
                valid: false,
            };

            // premature return if the preferred size does not actually fit within the window

            if outer.w > rect.w
                || outer.h > rect.h
                || outer.x < 0.
                || outer.y < 0.
                || outer.x + outer.w > ctx.gfx.window().inner_size().width as f32
                || outer.y + outer.h > ctx.gfx.window().inner_size().height as f32
            {
                self.draw_cache = DrawCache::default();
                return;
            }
        }

        // bind variables

        let (outer_bounds, inner_bounds) = (self.draw_cache.outer, self.draw_cache.inner);

        // draw visuals
        if self.draw_cache.outer.contains(ctx.mouse.position()){
            self.hover_visuals.unwrap_or(self.visuals)
        } else {
            self.visuals
        }.draw(canvas, outer_bounds);

        // draw content

        self.content.draw_content(ctx, canvas, inner_bounds);
    }
}

pub trait UiContent {
    fn to_element(self, id: u32) -> UiElement
    where
        Self: Sized + 'static,
    {
        UiElement::new(id, self)
    }

    fn to_element_measured(self, id: u32, _ctx: &Context) -> UiElement
    where
        Self: Sized + 'static,
    {
        self.to_element(id)
    }

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

    /// Takes in a rectangle target, a canvas, a context and draws the contents (not the border etc.) to that rectangle within that canvas using that context.
    /// Normally, this will only be called from within the [UiElement::draw_to_rectangle] function, when the cache has been modified appropiately and only use the inner rectangle of the draw cache as content_bounds. Do not call otherwise.
    fn draw_content(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        content_bounds: graphics::Rect,
    );

    /// Returns access to this elements children, if there are any. Returns None if this is a leaf node.
    fn get_children(&self) -> Option<&[UiElement]> {
        None
    }

    /// Attempts to add a UiElement to this elements children.
    /// Returns true if the operation succeeds.
    /// Returns false if this is a leaf node that cannot have any children.
    fn add(&mut self, _element: UiElement) -> bool {
        false
    }
}
