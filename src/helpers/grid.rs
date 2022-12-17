//! A two-dimensional array using a flat internal representation.
//!
//! This is a row major implementation, consecutive elements across the x
//! dimension are next to each other, whereas columns are strided.
//!
//! `x` represents variation in row elements (which column the value is in),
//! whereas `y` represents a change in column elements (which row is it in). The
//! grid may be indexed with a tuple using `get_from2d((x,y))` , for example:
//!
//! - get_from2d((5, 0)) returns the sixth element of the first row. It can also be interpreted as the the element at
//!   column 5 and row 0.
//!
//! - get_from2d((1, 5)) returns the second element of the sixth row. In other words, the element at column 1 and row 5.
//!
//! # Indexing
//!
//! Implements the Index trait, so the grid may be read by a tuple inside
//! square brackets. Example:
//!
//! ```
//! use adv20::helpers::grid::Grid;
//! let mut grid = Grid::new(5, 5, 0u8);
//! let v = grid.get_mut(2, 2);
//! *v = 100;
//! assert_eq!(grid[(2,2)], 100);
//! ```
//!
//! ## Beware
//!
//! If no inferring is made, the Default type for tuples in rust is i32.
//!
//! # Panics
//!
//! Panics if the indexing inside square brackets is done with negative values.

// use std::{convert::TryInto, fmt::Debug, ops::Index};

// use super::base2d::Base2d;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    flat: Vec<T>,
    pub len_x: usize,
    pub len_y: usize,
}

impl<T: Clone> Grid<T> {
    /// creates a new grid with al the elements having the `init`ial value
    pub fn new(len_x: usize, len_y: usize, init: T) -> Grid<T> {
        Grid {
            flat: vec![init; len_x * len_y],
            len_x,
            len_y,
        }
    }

    /// If vector `v` is larger than `len_x` * `len_y`, the extra elements are
    /// truncated.
    ///
    /// # Panics
    ///
    /// - The input vector `v` must have at least `len_x` * `len_y` lenght.
    /// Otherwise the program may panic while trying to access the elements of
    /// the inner vector;
    pub fn from_vec(len_x: usize, len_y: usize, mut v: Vec<T>) -> Grid<T> {
        debug_assert!(v.len() >= len_x * len_y);
        v.truncate(len_x * len_y);

        Grid { flat: v, len_x, len_y }
    }
}

impl<T> Grid<T> {
    //------------------------------
    // Getters for single elements
    //------------------------------

    /// returns the value at position x,y.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.flat[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.index(x, y); // must have an aux variable coz mutable borrow
        &mut self.flat[i]
    }

    //------------------------------
    // Getters for multiple elements
    //------------------------------

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.flat.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.flat.iter_mut()
    }

    // returns an array slice for a line of the grid
    pub fn row(&self, y: usize) -> &[T] {
        &self.flat[self.index(0, y)..=self.index(self.len_x - 1, y)]
    }

    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        let idx0 = self.index(0, y);
        let idx1 = self.index(self.len_x - 1, y);
        &mut self.flat[idx0..=idx1]
    }

    pub fn iter_col(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        self.flat.iter().skip(x).step_by(self.len_x)
    }

    pub fn iter_col_mut(&mut self, x: usize) -> impl DoubleEndedIterator<Item = &mut T> + ExactSizeIterator {
        self.flat.iter_mut().skip(x).step_by(self.len_x)
    }

    //------------------------------
    // Helpers
    //------------------------------

    /// returns the total size of the array (len_x * len_y)
    pub fn size(&self) -> usize {
        self.flat.len()
    }

    //------------------------------
    // Private
    //------------------------------

    /// returns the index for acessing the `flat` array from the coordinates `x`
    /// and `y`.
    fn index(&self, x: usize, y: usize) -> usize {
        self.len_x * y + x
    }
}
