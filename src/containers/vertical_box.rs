use ggez::graphics::Rect;
use std::hash::Hash;

use crate::{
    ui_element::{Size, UiContainer},
    UiContent, UiElement,
};

/// A vertical box that will group elements from left to right. Stores elements in a vector that determines order of elements within the box.
/// Elements will adhere to their own x and y alignment within the rectangle provided to them by this box.
pub struct VerticalBox<T: Copy + Eq + Hash> {
    /// contains the UiElements within this box in the right order (top to bottom)
    children: Vec<UiElement<T>>,
    /// The amount of spacing between two neighboring elements.
    pub spacing: f32,
}

impl<T: Copy + Eq + Hash> VerticalBox<T> {
    /// Returns a new VerticalBox with a default spacing of 5 pixels.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 5.,
        }
    }

    /// Returns a new VerticalBox with the required spacing.
    pub fn new_spaced(spacing: f32) -> Self {
        Self {
            children: Vec::new(),
            spacing,
        }
    }

    /// Requires an amount of height to _dynamically_ allocate, aka all height that is provided but not taken up by padding.
    /// Returns a vector of the same size as the number of elements in the box, containing the height of the rectangle that can be passed to each child element for drawing.
    /// Respects the size types of the child elements.
    fn get_element_heights(&self, height_available: f32) -> Vec<f32> {
        // create mutable copy of leftover space, subtracting the amount needed for spacing and the minimum amount claimed by each element
        // this must be greater than or equal to 0, or the draw_to_rectangle function would already have returned, but the function will not break if it is.
        let mut leftover = height_available - self.content_height_range().0;

        // create result vector and initialize it with elements min height
        let mut res = vec![0.; self.children.len()];
        for (element, h) in self.children.iter().zip(res.iter_mut()) {
            *h = element.height_range().0;
        }

        // first distribute as much height as possible to elements with the FILL size
        self.distribute_height_to_fitting(&mut leftover, &mut res, |ele| {
            matches!(ele.get_layout().y_size, Size::Fill(_, _))
        });

        // distribute remaining height to elements with the SHRINK size
        self.distribute_height_to_fitting(&mut leftover, &mut res, |ele| {
            matches!(ele.get_layout().y_size, Size::Shrink(_, _))
        });

        res
    }

    /// Iterates over the vector and all elements in this box that fullfil the predicate simulateously, adding height to each element (reducing leftover in parallel) until
    ///  - leftover has reached 0 and no height is left to distribute
    ///  - all elements fulfilling the predicate have reached their maximum height.
    fn distribute_height_to_fitting(
        &self,
        leftover: &mut f32,
        res: &mut [f32],
        pred: impl Fn(&UiElement<T>) -> bool,
    ) {
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
                let growth_left = ele.height_range().1 - *size;

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

impl<T: Copy + Eq + Hash> Default for VerticalBox<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Eq + Hash> UiContent<T> for VerticalBox<T> {
    fn to_element_builder(
        self,
        id: u32,
        _ctx: &ggez::Context,
    ) -> crate::ui_element::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        crate::ui_element::UiElementBuilder::new(id, self).with_size(
            Size::Shrink(0., f32::INFINITY),
            Size::Shrink(0., f32::INFINITY),
        )
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        param: crate::ui_element::UiDrawParam,
    ) {
        // get calculate vector of dynamically allocated total heights for each element

        let dyn_height = self.get_element_heights(param.target.h);

        let mut y = param.target.y;

        // draw subelements
        for (element, ele_dyn_height) in self.children.iter_mut().zip(dyn_height) {
            element.draw_to_rectangle(
                ctx,
                canvas,
                param.target(Rect {
                    x: param.target.x,
                    y,
                    w: param.target.w,
                    h: ele_dyn_height,
                }),
            );
            y += ele_dyn_height + self.spacing;
        }
    }

    fn container(&self) -> Option<&dyn UiContainer<T>> {
        Some(self)
    }

    fn container_mut(&mut self) -> Option<&mut dyn UiContainer<T>> {
        Some(self)
    }
}
impl<T: Copy + Eq + Hash> UiContainer<T> for VerticalBox<T> {
    fn content_width_range(&self) -> (f32, f32) {
        // maximum of all min widths and minimum of all max widths, as all elements are layed out in parallel x direction

        self.children
            .iter()
            .fold((f32::EPSILON, f32::INFINITY), |last, element| {
                (
                    last.0.max(element.width_range().0),
                    last.1.min(element.width_range().1),
                )
            })
    }

    fn content_height_range(&self) -> (f32, f32) {
        // sum of all min widths and sum of all max widths, as elements are stacked in y direction

        let pure_inner = self.children.iter().fold((0., 0.), |last, element| {
            (
                last.0 + element.height_range().0,
                last.1 + element.height_range().1,
            )
        });

        // add padding and return

        (
            pure_inner.0 + (0.max(self.children.len() as i32 - 1)) as f32 * self.spacing,
            pure_inner.1 + (0.max(self.children.len() as i32 - 1)) as f32 * self.spacing,
        )
    }

    fn get_children(&self) -> &[UiElement<T>] {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut [UiElement<T>] {
        &mut self.children
    }

    fn add(&mut self, element: UiElement<T>) {
        self.children.push(element);
    }

    fn remove_expired(&mut self) {
        self.children.retain(|child| !child.expired());
    }

    fn remove_id(&mut self, id: u32) {
        self.children.retain(|child| child.get_id() != id);
    }
}
