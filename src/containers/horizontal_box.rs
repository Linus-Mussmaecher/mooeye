use ggez::{graphics::Rect};

use crate::{UiElement, UiContent, ui_element::layout::Size};

/// A horizontal box that will group elements from left to right. Stores elements in a vector that determines order of elements within the box.
/// Elements will adhere to their own x and y alignment within the box provided to them
pub struct HorizontalBox {
    /// contains the UiElements within this box in the right order
    pub children: Vec<UiElement>,
    /// the amount of spacing between two neighboring elements
    pub spacing: f32,
}

impl UiContent for HorizontalBox {
    fn to_element(self, id: u32) -> UiElement where Self:Sized + 'static {
            let mut res = UiElement::new(id, self);
            res.layout.x_size = Size::SHRINK(0., f32::INFINITY);
            res.layout.y_size = Size::SHRINK(0., f32::INFINITY);
            res
    }
    fn content_width_range(&self) -> (f32, f32) {
        // sum of all min widths and sum of all max widths, as elements are stacked in y direction

        let pure_inner = self.children.iter().fold((0., 0.), |last, element| {
            (
                last.0 + element.width_range().0,
                last.1 + element.width_range().1,
            )
        });

        // add spacing and return

        (
            pure_inner.0 + (0.max(self.children.len() as i32 - 1)) as f32 * self.spacing,
            pure_inner.1 + (0.max(self.children.len() as i32 - 1)) as f32 * self.spacing,
        )
    }

    fn content_height_range(&self) -> (f32, f32) {
        // maximum of all min widths and minimum of all max widths, as all children are layed out in parallel x direction

        self.children
            .iter()
            .fold((f32::EPSILON, f32::INFINITY), |last, element| {
                (
                    last.0.max(element.height_range().0),
                    last.1.min(element.height_range().1),
                )
            })
    }

    fn get_children(&self) -> Option<&[UiElement]> {
        Some(&self.children)
    }

    fn add(&mut self, element: UiElement) -> bool {
        self.children.push(element);
        true
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        content_bounds: ggez::graphics::Rect,
    ) {
        // get calculate vector of dynamically allocated total heights for each element

        let dyn_width = self.get_element_widths(content_bounds.w);
        let mut x = content_bounds.x;
        // draw subelements
        for (element, ele_dyn_width) in self.children.iter_mut().zip(dyn_width) {
            element.draw_to_rectangle(
                ctx,
                canvas,
                Rect {
                    x: x,
                    y: content_bounds.y,
                    w: ele_dyn_width,
                    h: content_bounds.h,
                },
            );
            x += ele_dyn_width + self.spacing;
        }
    }
}

impl HorizontalBox {
    /// Returns a new HorizontalBox with a default spacing of 5 pixels.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 5.,
        }
    }

    /// Requires an amount of height to _dynamically_ allocate, aka all height that is provided but not taken up by padding.
    /// Returns a vector of the same size as the number of elements in the box, containing the height of the rectangle that can be passed to each child element for drawing.
    /// Respects the size types of the child elements.
    fn get_element_widths(&self, width_available: f32) -> Vec<f32> {
        // create mutable copy of leftover space, subtracting the amount needed for spacing and the minimum amount claimed by each element
        // this must be greater than or equal to 0, or the draw_to_rectangle function would already have returned, but the function will not break if it is.
        let mut leftover = width_available - self.content_width_range().0;

        // create result vector and initialize it with elements min height
        let mut res = vec![0.; self.children.len()];
        for (element, h) in self.children.iter().zip(res.iter_mut()) {
            *h = element.width_range().0;
        }

        // first distribute as much height as possible to elements with the FILL size
        self.distribute_width_to_fitting(&mut leftover, &mut res, |ele| {
            matches!(ele.layout.x_size, Size::FILL(_, _))
        });

        // distribute remaining height to elements with the SHRINK size
        self.distribute_width_to_fitting(&mut leftover, &mut res, |ele| {
            matches!(ele.layout.x_size, Size::SHRINK(_, _))
        });

        res
    }

    /// Iterates over the vector and all elements in this box that fullfil the predicate simulateously, adding height to each element (reducing leftover in parallel) until
    ///  - leftover has reached 0 and no height is left to distribute
    ///  - all elements fulfilling the predicate have reached their maximum height.
    fn distribute_width_to_fitting<F>(&self, leftover: &mut f32, res: &mut Vec<f32>, pred: F)
    where
        F: Fn(&UiElement) -> bool,
    {
        // get the number of elements fulfilling the predicate
        let mut element_count = self.children.iter().filter(|e| pred(*e)).count();

        // check for early return
        if element_count == 0 || *leftover <= 0. {
            return;
        }

        // while their is still space to distribute and elements left to receive it
        while *leftover > 0. && element_count > 0 {
            // divide the space evenly between eligible elements
            let per_element = *leftover / element_count as f32;
            // then iterate over all elements
            for (ele, size) in self.children.iter().zip(res.iter_mut()) {
                // check how much more this element could grow
                let growth_left = ele.width_range().1 - *size;

                // check if the element fulfils the predicate and can still grow
                if pred(ele) && growth_left > 0. {
                    // calculate actual growth (may be bounded by element max size)
                    let growth = if growth_left > per_element {
                        per_element
                    } else {
                        // if max size reached, element is no longer eligible for next round
                        element_count -= 1;
                        growth_left
                    };

                    // add the growth to the size in the vector while simultaneously subtracting it from the leftover value
                    *size += growth;
                    *leftover -= growth;
                }
            }
        }
    }
}
