use good_web_game::{graphics::Rect, GameResult};
use std::hash::Hash;
use tinyvec::TinyVec;

/// The default size for tinyvecs in this module.
const VECSIZE: usize = 32;

use crate::ui;
use crate::ui::UiContainer;

/// A Grid Box that is initialized with a fixed width and height an can display elements in every cell.
pub struct GridBox<T: Copy + Eq + Hash> {
    /// The contents of this grid box, organized by rows
    children: Vec<ui::UiElement<T>>,

    /// The distance between two rows of this grid box.
    pub vertical_spacing: f32,
    /// The distance between two columns of this grid box.
    pub horizontal_spacing: f32,

    /// The number of rows in this grid box.
    rows: usize,
    /// The number of columns in thei grid box.
    cols: usize,
    ///// A rectangle cache to prevent recalculation of child boxes every frame.
    //children_rects: Vec<Rect>,
}

impl<T: Copy + Eq + Hash> GridBox<T> {
    /// Creates a new GridBox with the specified number of columns and rows.
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            children: (0..columns * rows)
                .map(|_| ui::UiElement::new(0, ()))
                .collect(),
            vertical_spacing: 5.,
            horizontal_spacing: 5.,
            cols: columns,
            rows,
            //children_rects: vec![Rect::default(); rows * columns],
        }
    }

    /// Returns a new GridBox with the required spacing.
    pub fn new_spaced(
        columns: usize,
        rows: usize,
        horizontal_spacing: f32,
        vertical_spacing: f32,
    ) -> Self {
        Self {
            children: (0..columns * rows)
                .map(|_| ui::UiElement::new(0, ()))
                .collect(),
            vertical_spacing,
            horizontal_spacing,
            cols: columns,
            rows,
            //children_rects: vec![Rect::default(); rows * columns],
        }
    }

    /// Adds an element to the specified position in the grid, overwriting any element previously there.
    /// If the index is out of bounds, this function will return an error.
    /// Keep in mind that the basic [ui::UiElement::add_element] function will not work on a [GridBox].
    pub fn add(&mut self, element: ui::UiElement<T>, x: usize, y: usize) -> GameResult {
        if x >= self.cols || y >= self.rows {
            Err(good_web_game::GameError::CustomError(format!(
                "Index out of bounds: ({}, {}) does not fit in ({}, {}).",
                x, y, self.cols, self.rows
            )))
        } else {
            self.children[x + self.cols * y] = element;
            Ok(())
        }
    }

    /// Returns a Vector with as many entries as this element has columns, each describin the dynamically allocated width for that column.
    fn get_column_widths(&self, width_available: f32) -> TinyVec<[f32; VECSIZE]> {
        // Use helper function to calculate the width range of each column.
        let ranges = self.get_column_ranges();

        // Initalize result vector with minimum sizes of columns.
        let mut res = ranges.iter().map(|(a, _)| *a).collect();

        // Calculate amount of remaining width to distribute
        let mut rem_width = width_available - self.content_width_range().0;

        // First, distribute among those columns that have at least one fill element
        self.distribute_to_fitting(
            &mut rem_width,
            &mut res,
            &ranges,
            &(0..self.cols)
                .map(|col| {
                    self.children
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index % self.cols == col)
                        .fold(false, |hs, (_, element)| {
                            hs || matches!(element.get_layout().x_size, ui::Size::Fill(_, _))
                        })
                })
                .collect(),
        );

        // Then, distribute among those columns that have at least one shrink element.
        // While this may hit some columns twice, the ones that had fill and shrink won't grow further anyway
        self.distribute_to_fitting(
            &mut rem_width,
            &mut res,
            &ranges,
            &(0..self.cols)
                .map(|col| {
                    self.children
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index % self.cols == col)
                        .fold(false, |hs, (_, element)| {
                            hs || matches!(element.get_layout().x_size, ui::Size::Shrink(_, _))
                        })
                })
                .collect(),
        );

        res
    }

    /// Returns a Vector with as many entries as this element has rows, each describin the dynamically allocated height for that row.
    fn get_row_heights(&self, height_available: f32) -> TinyVec<[f32; VECSIZE]> {
        // Use helper function to calculate the height range of each row.
        let ranges = self.get_row_ranges();

        // Initalize result vector with minimum sizes of rows.
        let mut res = ranges.iter().map(|(a, _)| *a).collect();

        // Calculate amount of remaining height to distribute
        let mut rem_height = height_available - self.content_height_range().0;

        // First, distribute among those columns that have at least one fill element
        self.distribute_to_fitting(
            &mut rem_height,
            &mut res,
            &ranges,
            &(0..self.rows)
                .map(|col| {
                    self.children
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index / self.cols == col)
                        .fold(false, |hs, (_, element)| {
                            hs || matches!(element.get_layout().y_size, ui::Size::Fill(_, _))
                        })
                })
                .collect(),
        );

        // Then, distribute among those columns that have at least one shrink element.
        // While this may hit some columns twice, the ones that had fill and shrink won't grow further anyway
        self.distribute_to_fitting(
            &mut rem_height,
            &mut res,
            &ranges,
            &(0..self.rows)
                .map(|col| {
                    self.children
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index / self.cols == col)
                        .fold(false, |hs, (_, element)| {
                            hs || matches!(element.get_layout().y_size, ui::Size::Shrink(_, _))
                        })
                })
                .collect(),
        );

        res
    }

    /// Iterates over the result vector, the ranges and the receives vector in parallel, adding height to each entry (reducing leftover in parallel) until
    ///  - leftover has reached 0 and no height is left to distribute
    ///  - all entries with 'receive' set as true have reached their maximum height.
    fn distribute_to_fitting(
        &self,
        leftover: &mut f32,
        res: &mut TinyVec<[f32; VECSIZE]>,
        ranges: &TinyVec<[(f32, f32); VECSIZE]>,
        receives: &TinyVec<[bool; VECSIZE]>,
    ) {
        // get the number of elements fulfilling the predicate
        let mut element_count = receives.iter().filter(|a| **a).count();

        // check for early return
        if element_count == 0 || *leftover <= 0. {
            return;
        }

        // while their is still space to distribute and elements left to receive it
        while *leftover > 0. && element_count > 0 {
            // divide the space evenly between eligible elements
            let per_element = *leftover / element_count as f32;
            // then iterate over all elements
            for ((size, receive), range) in res.iter_mut().zip(receives).zip(ranges) {
                // check how much more this element could grow
                let growth_left = range.1 - *size;

                // check if the element fulfils the predicate and can still grow
                if *receive && growth_left > 0. {
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

    /// Returns a vector containing for every column in this grid the width_range of that column.
    /// Width range is calculated by taking the maximum min_width and minimum max_width of all children in each column.
    fn get_column_ranges(&self) -> TinyVec<[(f32, f32); VECSIZE]> {
        (0..self.cols)
            .map(|col| {
                self.children
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index % self.cols == col)
                    .fold((f32::EPSILON, f32::INFINITY), |old, (_, element)| {
                        (
                            old.0.max(element.width_range().0),
                            old.1.min(element.width_range().1),
                        )
                    })
            })
            .collect()
    }

    /// Returns a vector containing for every row in this grid the height_range of that row.
    /// Height range is calculated by taking the maximum min_height and minimum max_height of all children in each row.
    fn get_row_ranges(&self) -> TinyVec<[(f32, f32); VECSIZE]> {
        (0..self.rows)
            .map(|row| {
                self.children
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index / self.cols == row)
                    .fold((f32::EPSILON, f32::INFINITY), |old, (_, element)| {
                        (
                            old.0.max(element.height_range().0),
                            old.1.min(element.height_range().1),
                        )
                    })
            })
            .collect()
    }
}

impl<T: Copy + Eq + Hash> ui::UiContent<T> for GridBox<T> {
    fn to_element_builder(self, id: u32, _ctx: &good_web_game::Context) -> ui::UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        ui::UiElementBuilder::new(id, self).with_size(
            ui::Size::Shrink(0., f32::INFINITY),
            ui::Size::Shrink(0., f32::INFINITY),
        )
    }

    fn draw_content(
        &mut self,
        ctx: &mut good_web_game::Context,
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
        param: ui::UiDrawParam,
    ) {
        // get column widths
        let column_widths = self.get_column_widths(param.target.w);
        // ... and partial sum
        let column_widths_ps =
            column_widths
                .iter()
                .fold(Vec::from([param.target.x]), |mut vec, val| {
                    vec.push(*vec.last().unwrap_or(&0.) + val + self.horizontal_spacing);
                    vec
                });

        // get row heights
        let row_heights = self.get_row_heights(param.target.h);
        // ... and partial sum
        let row_heights_ps =
            row_heights
                .iter()
                .fold(Vec::from([param.target.y]), |mut vec, val| {
                    vec.push(*vec.last().unwrap_or(&0.) + val + self.vertical_spacing);
                    vec
                });

        // actually draw children
        for (index, element) in self.children.iter_mut().enumerate() {
            element.draw_to_rectangle(
                ctx,
                gfx_ctx,
                param.target(Rect::new(
                    *column_widths_ps.get(index % self.cols).unwrap_or(&0.),
                    *row_heights_ps.get(index / self.cols).unwrap_or(&0.),
                    *column_widths.get(index % self.cols).unwrap_or(&0.),
                    *row_heights.get(index / self.cols).unwrap_or(&0.),
                )),
            );
        }
    }

    fn container(&self) -> Option<&dyn ui::UiContainer<T>> {
        Some(self)
    }

    fn container_mut(&mut self) -> Option<&mut dyn ui::UiContainer<T>> {
        Some(self)
    }
}

impl<T: Copy + Eq + Hash> ui::UiContainer<T> for GridBox<T> {
    fn content_width_range(&self) -> (f32, f32) {
        self.get_column_ranges().iter().fold(
            (
                (0.max(self.cols - 1)) as f32 * self.horizontal_spacing,
                (0.max(self.cols - 1)) as f32 * self.horizontal_spacing,
            ),
            |old, range| (old.0 + range.0, old.1 + range.1),
        )
    }

    fn content_height_range(&self) -> (f32, f32) {
        self.get_row_ranges().iter().fold(
            (
                (0.max(self.rows - 1)) as f32 * self.vertical_spacing,
                (0.max(self.rows - 1)) as f32 * self.vertical_spacing,
            ),
            |old, range| (old.0 + range.0, old.1 + range.1),
        )
    }

    fn get_children(&self) -> &[ui::UiElement<T>] {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut [ui::UiElement<T>] {
        &mut self.children
    }

    fn add(&mut self, _element: ui::UiElement<T>) {
        panic!("You tried to add an element to a grid box without using an index. This may happen because you tried to use the add-to-id functionality from live adding. Don't do this with grid boxes.")
    }

    fn remove_expired(&mut self) {
        for i in 0..self.children.len() {
            if self.children[i].expired() {
                self.children[i] = ui::UiElement::new(0, ());
            }
        }
    }

    fn remove_id(&mut self, id: u32) {
        for i in 0..self.children.len() {
            if self.children[i].get_id() == id {
                self.children[i] = ui::UiElement::new(0, ());
            }
        }
    }
}
