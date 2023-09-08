use good_web_game::graphics::Rect;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
/// An enum that describes the alignment behaviour of an element.
pub enum Alignment {
    /// Element aligns top or left
    Min,
    #[default]
    /// Element aligns centered
    Center,
    /// Element aligns bottom or right
    Max,
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// An enum that describes the size and growth behaviour of an element.
pub enum Size {
    /// Element will always have this size.
    Fixed(f32),
    /// Element tries to grow as big as possible within the given bounds, sharing space equally with other [Size::Fill] elements.
    Fill(f32, f32),
    /// Element tries to shrink as small as possible within the given bounds, sharing space with other [Size::Shrink] elements equally only when no other FILL elements are present.
    Shrink(f32, f32),
}

impl Default for Size {
    fn default() -> Self {
        Self::Fill(0., f32::INFINITY)
    }
}

impl Size {
    /// Returns the minimum amount of space an element of this size requires.
    pub fn min(&self) -> f32 {
        match self {
            Self::Fixed(s) => *s,
            Self::Fill(min, _) => *min,
            Self::Shrink(min, _) => *min,
        }
    }

    /// Returns the maximum amount of space an element of this size can grow to.
    pub fn max(&self) -> f32 {
        match self {
            Self::Fixed(s) => *s,
            Self::Fill(_, max) => *max,
            Self::Shrink(_, max) => *max,
        }
    }

    /// Returs the size this element would prefer within a range.
    /// Elements will never leave their layout bounds, even if that means ignoring the passed  bounds.
    /// Thus, FIXED elements will always just return their own size.
    pub(crate) fn pref(&self, min: f32, max: f32) -> f32 {
        match self {
            Self::Fixed(s) => *s,
            Self::Fill(fmin, fmax) => max.max(min).clamp(*fmin, *fmax), //(*fmax).min(max).min(*fmax).max(min).max(*fmin),
            Self::Shrink(smin, smax) => min.min(max).clamp(*smin, *smax), //(*smin).min(max).min(*smax).max(min).max(*smin),
        }
    }

    /// Returns a new [Size] of the same variant, but with all boundaries scaled by the given factor.
    pub fn scale(&self, scale: f32) -> Self {
        match self {
            Self::Fixed(s) => Self::Fixed(scale * s),
            Self::Fill(fmin, fmax) => Self::Fill(fmin * scale, fmax * scale), //(*fmax).min(max).min(*fmax).max(min).max(*fmin),
            Self::Shrink(smin, smax) => Self::Shrink(smin * scale, smax * scale), //(*smin).min(max).min(*smax).max(min).max(*smin),
        }
    }

    /// Returns a new [Size] with the same boundaries, but variant changed to [Size::Shrink].
    pub fn to_shrink(self) -> Self {
        match self {
            Size::Fixed(s) => Self::Shrink(s, f32::INFINITY),
            Size::Fill(min, max) => Self::Shrink(min, max),
            Size::Shrink(min, max) => Self::Shrink(min, max),
        }
    }

    /// Returns a new [Size] with the same boundaries, but variant changed to [Size::Fill].
    pub fn to_fill(self) -> Self {
        match self {
            Size::Fixed(s) => Self::Fill(s, f32::INFINITY),
            Size::Fill(min, max) => Self::Fill(min, max),
            Size::Shrink(min, max) => Self::Fill(min, max),
        }
    }

    /// Returns a new [Size] with the same boundaries, but variant changed to [Size::Fixed].
    pub fn to_fixed(self) -> Self {
        match self {
            Size::Fixed(s) => Self::Fixed(s),
            Size::Fill(min, _) => Self::Fixed(min),
            Size::Shrink(min, _) => Self::Fixed(min),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// A struct that contains information about the layout of an UI-Element: their alignment, size, offset and padding.
pub struct Layout {
    /// Wether this element aligns left, center or right. See [Alignment].
    pub x_alignment: Alignment,
    /// Wether this element aligns top, center or bottom. See [Alignment].
    pub y_alignment: Alignment,
    /// How many pixels away from the most left- or rightmost position this element aligns. Should be positive. Does not work with [Alignment::Center].
    pub x_offset: f32,
    /// How many pixels away from the most top- or bottommost position this element aligns. Should be positive. Does not work with [Alignment::Center].
    pub y_offset: f32,
    /// The size and growth behaviour of this element in the horizontal direction. See [Size].
    pub x_size: Size,
    /// The size and growth behaviour of this element in the vertical direction. See [Size].
    pub y_size: Size,
    /// Specifies the padding, extra space around the cental element(s), of a container in the order top, right, bottom, left.
    pub padding: (f32, f32, f32, f32),
    /// Specifies wether this elements content will only receive draw rectangles in the size of their content min ratio.
    pub preserve_ratio: bool,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            x_alignment: Default::default(),
            y_alignment: Default::default(),
            x_offset: Default::default(),
            y_offset: Default::default(),
            x_size: Default::default(),
            y_size: Default::default(),
            padding: (5., 5., 5., 5.),
            preserve_ratio: Default::default(),
        }
    }
}

impl Layout {
    /// Takes in a rectangle target to which this element is supposed to be drawn.
    /// Returns an (outer rect, inner rect), with the inner rect being the space content is drawn to
    pub fn get_outer_inner_bounds_in_target(
        &self,
        target: &Rect,
        content_min: glam::Vec2,
    ) -> (Rect, Rect) {
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

        if self.preserve_ratio {
            let scale_x = w / self.x_size.min();
            let scale_y = h / self.y_size.min();
            let scale_min = scale_x.min(scale_y);

            w = self.x_size.min() * scale_min;
            h = self.y_size.min() * scale_min;
        }

        // calculate position of top left of outer box

        let x_out = match self.x_alignment {
            Alignment::Min => target.x,
            Alignment::Center => target.x + target.w / 2. - w_out / 2.,
            Alignment::Max => target.x + target.w - w_out,
        } + self.x_offset;

        let y_out = match self.y_alignment {
            Alignment::Min => target.y,
            Alignment::Center => target.y + target.h / 2. - h_out / 2.,
            Alignment::Max => target.y + target.h - h_out,
        } + self.y_offset;

        // calculate inner positions independently. Adding padding does not work, as w/h might have changed as a result of ration preservation

        let x = x_out
            + match self.x_alignment {
                Alignment::Min => self.padding.3,
                Alignment::Center => (w_out + self.padding.3 - self.padding.1 - w) / 2.,
                Alignment::Max => w_out - w - self.padding.1,
            };
        let y = y_out
            + match self.y_alignment {
                Alignment::Min => self.padding.0,
                Alignment::Center => (h_out + self.padding.0 - self.padding.2 - h) / 2.,
                Alignment::Max => h_out - h - self.padding.2,
            };

        (Rect::new(x_out, y_out, w_out, h_out), Rect::new(x, y, w, h))
    }
}
