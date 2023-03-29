use ggez::{
    glam::Vec2,
    graphics::Rect,
};

#[allow(dead_code)]
#[derive(Copy, Clone)]
/// An enum that describes the alignment behaviour of an element. 3 variants: MIN (top or left), CENTER, MAX (bottom or right).
pub enum Alignment {
    /// Element aligns top or left
    MIN,
    /// Element aligns centered
    CENTER,
    /// Element aligns bottom or right
    MAX,
}

#[derive(Copy, Clone, PartialEq)]
/// An enum that describes the size and growth behaviour of an element. 3 variants: FIXED, FILL and SHRINK. When an element is drawn, any padding will be added _on top of_ these boundaries!
pub enum Size {
    /// Element will always have this size.
    FIXED(f32),
    /// Element tries to grow as big as possible within the given bounds, sharing space equally with other FILL elements.
    FILL(f32, f32),
    /// Element tries to shrink as small as possible within the given bounds, sharing space with other SHRINK elements equally only when no other FILL elements are present.
    SHRINK(f32, f32),
}

impl Size {
    /// Returns the minimum amount of space an element of this size requires.
    pub fn min(&self) -> f32 {
        match self {
            Size::FIXED(s) => *s,
            Size::FILL(min, _) => *min,
            Size::SHRINK(min, _) => *min,
        }
    }

    /// Returns the maximum amount of space an element of this size will grow to.
    pub fn max(&self) -> f32 {
        match self {
            Size::FIXED(s) => *s,
            Size::FILL(_, max) => *max,
            Size::SHRINK(_, max) => *max,
        }
    }

    /// Returs the size this element would prefer within a range.
    /// Elements will never leave their layout bounds, even if that means ignoring the passed  bounds.
    /// Thus, FIXED elements will always just return their own size.
    pub fn pref(&self, min: f32, max: f32) -> f32 {
        match self {
            Size::FIXED(s) => *s,
            Size::FILL(fmin, fmax) => max.max(min).clamp(*fmin, *fmax), //(*fmax).min(max).min(*fmax).max(min).max(*fmin),
            Size::SHRINK(smin, smax) => min.min(max).clamp(*smin, *smax), //(*smin).min(max).min(*smax).max(min).max(*smin),
        }
    }
}

#[derive(Clone, Copy)]
/// A struct that contains information about the layout of an UI-Element: their alignment, size, offset and padding.
pub struct Layout {
    /// Wether this element aligns left, center or right. See mooeye::Alignment.
    pub x_alignment: Alignment,
    /// Wether this element aligns top, center or bottom. See mooeye::Alignment.
    pub y_alignment: Alignment,
    /// How many pixels away from the most left- or rightmost position this element aligns. Should be positive. Does not work with Alignment::CENTER.
    pub x_offset: f32,
    /// How many pixels away from the most top- or bottommost position this element aligns. Should be positive. Does not work with Alignment::CENTER.
    pub y_offset: f32,
    /// The size and growth behaviour of this element in the horizontal direction. See mooeye::Size.
    pub x_size: Size,
    /// The size and growth behaviour of this element in the vertical direction. See mooeye::Size.
    pub y_size: Size,
    /// Specifies the padding, extra space around the cental element(s), of a container in the order top, right, bottom, left.
    pub padding: (f32, f32, f32, f32),
    /// Specifies wether this elements content will only receive draw rectangles in the size of their content min ratio
    pub preserve_ratio: bool,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            x_alignment: Alignment::CENTER,
            y_alignment: Alignment::CENTER,
            x_offset: Default::default(),
            y_offset: Default::default(),
            x_size: Size::FILL(0., f32::INFINITY),
            y_size: Size::FILL(0., f32::INFINITY),
            padding: (5., 5., 5., 5.),
            preserve_ratio: false,
        }
    }
}

impl Layout {
    /// Takes in a rectangle target to which this element is supposed to be drawn.
    /// Returns an (outer rect, inner rect), with the inner rect being the space content is drawn to
    pub fn get_outer_inner_bounds_in_target(&self, target: &Rect, content_min: Vec2) -> (Rect, Rect) {

        //calculate inner sizes via pref method of size

        let mut w = self
            .x_size
            .pref(content_min.x, target.w - self.padding.1 - self.padding.3);

        let mut h = self
            .y_size
            .pref(content_min.y, target.h - self.padding.0 - self.padding.2);


        // calculate outer sizes by adding padding

        let w_out = w + self.padding.1 + self.padding.3;
        let h_out = h + self.padding.0 + self.padding.2;

        
        // try to preserve ratio by calculating the scaling (above content_min) that would be applied an then applying the smaller one to both

        if self.preserve_ratio{
            let scale_x = w/self.x_size.min();
            let scale_y = h/self.y_size.min();
            let scale_min = scale_x.min(scale_y);

            w = self.x_size.min() * scale_min;
            h = self.y_size.min() * scale_min;
        }

        // calculate position of top left of outer box

        let x_out = match self.x_alignment {
            Alignment::MIN => target.x + self.x_offset,
            Alignment::CENTER => target.x + target.w / 2. - w_out / 2.,
            Alignment::MAX => target.x + target.w - w_out - self.x_offset,
        };

        let y_out = match self.y_alignment {
            Alignment::MIN => target.y + self.y_offset,
            Alignment::CENTER => target.y + target.h / 2. - h_out / 2.,
            Alignment::MAX => target.y + target.h - h_out - self.y_offset,
        };

        // calculate inner positions independently. Adding padding does not work, as w/h might have changed as a result of ration preservation

        let x = x_out + match self.x_alignment {
            Alignment::MIN => self.padding.1,
            Alignment::CENTER => (w_out + self.padding.1 - self.padding.1 - w)/2.,
            Alignment::MAX => w_out - w - self.padding.1,
        };
        let y = y_out + match self.y_alignment {
            Alignment::MIN => self.padding.1,
            Alignment::CENTER => (h_out + self.padding.1 - self.padding.3 - h)/2.,
            Alignment::MAX => h_out - h - self.padding.3,
        };

        (Rect::new(x_out, y_out, w_out, h_out), Rect::new(x, y, w, h))
    }
}