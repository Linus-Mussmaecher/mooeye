use ggez::{graphics::Rect, GameResult};
use std::hash::Hash;

use crate::{ui_element::layout::Size, UiContent, UiElement};

pub struct GridBox<T: Copy + Eq + Hash> {
    children: Vec<UiElement<T>>,
    pub v_spacing: f32,
    pub h_spacing: f32,
    rows: usize,
    cols: usize,
}

impl<T: Copy + Eq + Hash> GridBox<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            children: {
                let mut children = Vec::new();
                for _ in 0..width * height {
                    children.push(().to_element(0));
                }
                children
            },
            v_spacing: 5.,
            h_spacing: 5.,
            cols: width,
            rows: height,
        }
    }

    pub fn add(&mut self, element: UiElement<T>, x: usize, y: usize) -> GameResult {
        if x >= self.cols || y >= self.rows {
            Err(ggez::GameError::CustomError(
                "Index Out Of Bounds".to_owned(),
            ))
        } else {
            self.children[x + self.cols * y] = element;
            Ok(())
        }
    }

    /// Returns a Vector with as many entries as this element has columns, each describin the dynamically allocated width for that column.
    fn get_column_widths(&self, width_available: f32) -> Vec<f32> {
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
                            hs || matches!(element.layout.x_size, Size::FILL(_, _))
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
                            hs || matches!(element.layout.x_size, Size::SHRINK(_, _))
                        })
                })
                .collect(),
        );

        res
    }

    /// Returns a Vector with as many entries as this element has rows, each describin the dynamically allocated height for that row.
    fn get_row_heights(&self, height_available: f32) -> Vec<f32> {
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
                            hs || matches!(element.layout.y_size, Size::FILL(_, _))
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
                            hs || matches!(element.layout.y_size, Size::SHRINK(_, _))
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
        res: &mut Vec<f32>,
        ranges: &Vec<(f32, f32)>,
        receives: &Vec<bool>,
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

    fn get_column_ranges(&self) -> Vec<(f32, f32)> {
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

    fn get_row_ranges(&self) -> Vec<(f32, f32)> {
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

impl<T: Copy + Eq + Hash> UiContent<T> for GridBox<T> {
    fn to_element(self, id: u32) -> UiElement<T>
    where
        Self: Sized + 'static,
    {
        let mut res = UiElement::new(id, self);
        res.layout.x_size = Size::SHRINK(0., f32::INFINITY);
        res.layout.y_size = Size::SHRINK(0., f32::INFINITY);
        res
    }

    fn content_width_range(&self) -> (f32, f32) {
        self.get_column_ranges().iter().fold(
            (
                (0.max(self.cols - 1)) as f32 * self.h_spacing,
                (0.max(self.cols - 1)) as f32 * self.h_spacing,
            ),
            |old, range| (old.0 + range.0, old.1 + range.1),
        )
    }

    fn content_height_range(&self) -> (f32, f32) {
        self.get_row_ranges().iter().fold(
            (
                (0.max(self.rows - 1)) as f32 * self.v_spacing,
                (0.max(self.rows - 1)) as f32 * self.v_spacing,
            ),
            |old, range| (old.0 + range.0, old.1 + range.1),
        )
    }

    fn get_children(&self) -> Option<&[UiElement<T>]> {
        Some(&self.children)
    }

    fn add(&mut self, _element: UiElement<T>) -> bool {
        
        true
    }

    fn draw_content(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        content_bounds: ggez::graphics::Rect,
    ) {
        // get column widths
        let column_widths = self.get_column_widths(content_bounds.w);
        // ... and partial sum
        let column_widths_ps = column_widths.iter().fold(Vec::from([content_bounds.x]), |mut vec, val| {
            vec.push(*vec.last().unwrap_or(&0.) + val + self.h_spacing);
            vec
        });

        // get row heights
        let row_heights = self.get_row_heights(content_bounds.h);
        // ... and partial sum
        let row_heights_ps = row_heights.iter().fold(Vec::from([content_bounds.y]), |mut vec, val| {
            vec.push(*vec.last().unwrap_or(&0.) + val + self.v_spacing);
            vec
        });

        for (index, element) in self.children.iter_mut().enumerate() {
            element.draw_to_rectangle(
                ctx,
                canvas,
                Rect::new(
                    *column_widths_ps.get(index % self.cols).unwrap_or(&0.),
                    *row_heights_ps.get(index / self.cols).unwrap_or(&0.),
                    *column_widths.get(index % self.cols).unwrap_or(&0.),
                    *row_heights.get(index / self.cols).unwrap_or(&0.),
                ),
            );
        }
    }
}
