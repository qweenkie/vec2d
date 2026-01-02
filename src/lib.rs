mod test;

/// A 2D, row-major grid backed by a contiguous `Vec<T>`.
///
/// Elements are stored left-to-right, top-to-bottom. Indexing is performed
/// using `(x, y)` coordinates, where `(0, 0)` is the top-left corner.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vec2D<T> {
    cells: Vec<T>,
    width: usize,
}

#[derive(Debug)]
pub enum Vec2DErr {
    EmptySource,
    OutOfBounds,
    WidthMismatch(usize, usize),
    ZeroHeight,
    ZeroWidth,
}
impl std::error::Error for Vec2DErr {}

/// Indexes into the grid using `(x, y)` coordinates.
///
/// # Panics
/// Panics if `x` or `y` is out of bounds.
impl<T> std::ops::Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(
            x < self.width && y < self.height(),
            // Panic message
            "Vec2D index out of bounds: (x: {}, y: {}) in a {}x{} grid.",
            x,
            y,
            self.width,
            self.height()
        );

        &self.cells[y * self.width + x]
    }
}

/// Mutably indexes into the grid using `(x, y)` coordinates.
///
/// # Panics
/// Panics if `x` or `y` is out of bounds.
impl<T> std::ops::IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(
            x < self.width && y < self.height(),
            // Panic message
            "Vec2D index out of bounds: (x: {}, y: {}) in a {}x{} grid.",
            x,
            y,
            self.width,
            self.height()
        );

        &mut self.cells[y * self.width + x]
    }
}

/// Consumes the grid and returns the underlying storage vector.
impl<T> From<Vec2D<T>> for Vec<T> {
    fn from(value: Vec2D<T>) -> Self {
        value.cells
    }
}

impl<T> TryFrom<(Vec<T>, usize)> for Vec2D<T> {
    type Error = Vec2DErr;

    fn try_from(value: (Vec<T>, usize)) -> Result<Self, Self::Error> {
        Self::from_vec(value.0, value.1)
    }
}

/// Consumes the grid and iterates over all elements in row-major order.
impl<T> IntoIterator for Vec2D<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

/// Iterates over shared references to all elements in row-major order.
impl<'a, T> IntoIterator for &'a Vec2D<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

/// Iterates over mutable references to all elements in row-major order.
impl<'a, T> IntoIterator for &'a mut Vec2D<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}

/// Borrows the underlying storage as a slice.
impl<T> AsRef<[T]> for Vec2D<T> {
    fn as_ref(&self) -> &[T] {
        &self.cells
    }
}

/// Mutably borrows the underlying storage as a slice.
impl<T> AsMut<[T]> for Vec2D<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.cells
    }
}

impl<T> Vec2D<T> {
    /// Constructs a new grid filled with cloned default values for the type.
    ///
    /// # Errors
    /// Returns `Vec2DErr::ZeroWidth` if `width == 0`.
    /// Returns `Vec2DErr::ZeroHeight` if `height == 0`.
    pub fn new(width: usize, height: usize) -> Result<Self, Vec2DErr>
    where
        T: Default + Clone,
    {
        if width == 0 {
            return Err(Vec2DErr::ZeroWidth);
        } else if height == 0 {
            return Err(Vec2DErr::ZeroHeight);
        }

        Ok(Self {
            cells: vec![T::default(); width * height],
            width,
        })
    }

    /// Constructs a new grid filled with a cloned default value.
    ///
    /// # Errors
    /// Returns `Vec2DErr::ZeroWidth` if `width == 0`.
    /// Returns `Vec2DErr::ZeroHeight` if `height == 0`.
    pub fn new_with_default(width: usize, height: usize, default: T) -> Result<Self, Vec2DErr>
    where
        T: Clone,
    {
        if width == 0 {
            return Err(Vec2DErr::ZeroWidth);
        } else if height == 0 {
            return Err(Vec2DErr::ZeroHeight);
        }

        Ok(Self {
            cells: vec![default; width * height],
            width,
        })
    }

    /// Constructs a new grid by calling `constructor` for each cell.
    ///
    /// # Errors
    /// Returns `Vec2DErr::ZeroWidth` if `width == 0`.
    /// Returns `Vec2DErr::ZeroHeight` if `height == 0`.
    pub fn new_with_constructor(
        width: usize,
        height: usize,
        constructor: impl Fn() -> T,
    ) -> Result<Self, Vec2DErr> {
        if width == 0 {
            return Err(Vec2DErr::ZeroWidth);
        } else if height == 0 {
            return Err(Vec2DErr::ZeroHeight);
        }

        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            cells.push(constructor());
        }
        Ok(Self { cells, width })
    }

    /// Constructs a grid from an existing vector and a given width.
    /// The vector length must be a multiple of `width`.
    ///
    /// # Errors
    /// Returns `Vec2DErr::ZeroWidth` if `width == 0`.
    ///
    /// Returns `Vec2DErr::EmptySource` if the source vector is empty.
    ///
    /// Returns `Vec2DErr::WidthMismatch(vec.len(), width)` if the source
    /// vector's length is not divisible by the given `width`.
    pub fn from_vec(vec: Vec<T>, width: usize) -> Result<Self, Vec2DErr> {
        if width == 0 {
            return Err(Vec2DErr::ZeroWidth);
        } else if vec.is_empty() {
            return Err(Vec2DErr::EmptySource);
        }
        if !vec.len().is_multiple_of(width) {
            return Err(Vec2DErr::WidthMismatch(vec.len(), width));
        }

        Ok(Self { cells: vec, width })
    }

    /// Returns a shared slice of all cells in row-major order.
    pub fn cells(&self) -> &[T] {
        &self.cells
    }

    // This function is commented out, because I realized that we SHOULD NOT be
    // giving out mutable references to the backbone of our vector.
    //
    // /// Returns a mutable slice of all cells in row-major order.
    // pub fn cells_mut(&mut self) -> &mut [T] {
    //     &mut self.cells
    // }

    /// Returns the width of the grid.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub const fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    /// Converts `(x, y)` coordinates into a linear index.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height() {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    /// Converts the given index into 2D coordinates (for this 2d vector).
    ///
    /// Returns `None` if the coordinates are out of bounds.
    pub fn coords(&self, idx: usize) -> Option<(usize, usize)> {
        let (x, y) = (idx / self.width, idx % self.width);

        if x < self.width && y < self.height() {
            Some((x, y))
        } else {
            None
        }
    }

    /// Converts `(x, y)` coordinates into a linear index that would work for
    /// a flat representation of a 2d vector with the specified width.
    #[inline]
    pub fn create_index(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }

    /// Converts the given index into 2D coordinates that would work for a
    /// 2d vector that has the specified width.
    #[inline]
    pub fn create_coords(idx: usize, width: usize) -> (usize, usize) {
        (idx % width, idx / width)
    }

    /// Returns a shared reference to the cell at `(x, y)`, if it exists.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height() {
            return None;
        }

        self.cells.get(y * self.width + x)
    }

    /// Returns a mutable reference to the cell at `(x, y)`, if it exists.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height() {
            return None;
        }

        self.cells.get_mut(y * self.width + x)
    }

    /// Returns a shared slice representing row `y`, if it exists.
    pub fn get_row(&self, y: usize) -> Option<&[T]> {
        if y >= self.cells.len() / self.width {
            return None;
        }

        let start = y * self.width;
        let end = (y + 1) * self.width;
        Some(&self.cells[start..end])
    }

    /// Returns a mutable slice representing row `y`, if it exists.
    pub fn get_row_mut(&mut self, y: usize) -> Option<&mut [T]> {
        if y >= self.cells.len() / self.width {
            return None;
        }

        let start = y * self.width;
        let end = (y + 1) * self.width;
        Some(&mut self.cells[start..end])
    }

    /// Appends a row (or rows) at the end of the vector.
    ///
    /// The row's size has to be a multiple of the 2D vector's width.
    /// If the row's size is bigger that the 2D vector's width, but is still
    /// a multiple of width, the row will be inserted as multiple rows.
    ///
    /// This function implies it **will empty the original container upon insertion**.
    /// If you want to keep the container's contents, consider `extend_cloned`.
    ///
    /// # Errors:
    /// Returns `Vec2DErr::WidthMismatch(*rows_length*, *2d_vectors_width*)` if the
    /// row's length is not a multiple of the 2D vector's width.
    pub fn extend(&mut self, row: Vec<T>) -> Result<(), Vec2DErr> {
        if !row.len().is_multiple_of(self.width) {
            return Err(Vec2DErr::WidthMismatch(row.len(), self.width));
        }

        self.cells.extend(row);
        Ok(())
    }

    /// Appends a row (or rows) at the end of the vector.
    ///
    /// The row's size has to be a multiple of the 2D vector's width.
    /// If the row's size is bigger that the 2D vector's width, but is still
    /// a multiple of width, the row will be inserted as multiple rows.
    ///
    /// This function implies that the row's elements can be cloned.
    ///
    /// This should only be used if you need to preserve the row's contents in
    /// the original container, and is slower than `extend`.
    ///
    /// # Errors:
    /// Returns `Vec2DErr::WidthMismatch(*rows_length*, *2d_vectors_width*)` if the
    /// row's length is not a multiple of the 2D vector's width.
    pub fn extend_cloned(&mut self, row: &[T]) -> Result<(), Vec2DErr>
    where
        T: Clone,
    {
        if !row.len().is_multiple_of(self.width) {
            return Err(Vec2DErr::WidthMismatch(row.len(), self.width));
        }

        self.cells.extend(row.iter().cloned());
        Ok(())
    }

    /// Inserts a row (or rows) at a given y coordinate.
    ///
    /// The row's size has to be a multiple of the 2D vector's width.
    /// If the row's size is bigger that the 2D vector's width, but is still
    /// a multiple of width, the row will be inserted as multiple rows.
    ///
    /// Note, that the function will discard the given row, so if you want to
    /// keep it for some reason, consider cloning it before passing it in, or
    /// using `insert_row_cloned`.
    pub fn insert_row(&mut self, y: usize, row: Vec<T>) -> Result<(), Vec2DErr> {
        if y >= self.height() {
            return Err(Vec2DErr::OutOfBounds);
        }
        if !row.len().is_multiple_of(self.width) {
            return Err(Vec2DErr::WidthMismatch(row.len(), self.width));
        }

        let idx = y * self.width;
        self.cells.splice(idx..idx, row);
        Ok(())
    }

    /// Inserts a row (or rows) at a given y coordinate.
    ///
    /// The row's size has to be a multiple of the 2D vector's width.
    /// If the row's size is bigger that the 2D vector's width, but is still
    /// a multiple of width, the row will be inserted as multiple rows.
    ///
    /// This function implies that the row's contents can be cloned.
    pub fn insert_row_cloned(&mut self, y: usize, row: &[T]) -> Result<(), Vec2DErr>
    where
        T: Clone,
    {
        if y >= self.height() {
            return Err(Vec2DErr::OutOfBounds);
        }
        if !row.len().is_multiple_of(self.width) {
            return Err(Vec2DErr::WidthMismatch(row.len(), self.width));
        }

        let idx = y * self.width;
        self.cells.splice(idx..idx, row.iter().cloned());
        Ok(())
    }

    /// Discards a row at the given y coordinate, without preserving the values.
    pub fn discard_row(&mut self, y: usize) -> Result<(), Vec2DErr> {
        if y >= self.height() {
            return Err(Vec2DErr::OutOfBounds);
        }

        let start = y * self.width;
        let end = start + self.width;
        self.cells.drain(start..end);

        Ok(())
    }

    /// Removes a row at the given y coordinate, returning the removed values.
    ///
    /// This function implies that the vector's contents can be cloned.
    pub fn remove_row(&mut self, y: usize) -> Result<Vec<T>, Vec2DErr>
    where
        T: Clone,
    {
        if y >= self.height() {
            return Err(Vec2DErr::OutOfBounds);
        }

        let start = y * self.width;
        let end = start + self.width;

        Ok(self.cells.drain(start..end).collect())
    }

    /// Iterates over all cells, yielding their `(x, y)` coordinates and values.
    pub fn iter_xy(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let width = self.width;
        self.cells
            .iter()
            .enumerate()
            .map(move |(idx, cell)| ((idx % width, idx / width), cell))
    }

    /// Iterates mutably over all cells, yielding their `(x, y)` coordinates and values.
    pub fn iter_xy_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        let width = self.width;
        self.cells
            .iter_mut()
            .enumerate()
            .map(move |(idx, cell)| ((idx % width, idx / width), cell))
    }

    /// Iterates over grid rows as shared slices.
    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.cells.chunks_exact(self.width)
    }

    /// Iterates over grid rows as mutable slices
    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.cells.chunks_exact_mut(self.width)
    }

    /// Applies a function `f` to each cell without cloning the grid.
    pub fn map_in_place<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for cell in &mut self.cells {
            f(cell);
        }
    }

    /// Given the wanted height of our vector, cuts off all the exess rows.
    pub fn truncate_rows(&mut self, new_height: usize) -> Result<(), Vec2DErr> {
        if new_height > self.height() {
            return Err(Vec2DErr::OutOfBounds);
        } else if new_height == self.height() {
            return Ok(());
        }

        let start = new_height * self.width;
        self.cells.drain(start..);

        Ok(())
    }

    /// Given the wanted width of our vector, cuts off all the exess columns.
    pub fn truncate_cols(&mut self, new_width: usize) -> Result<(), Vec2DErr> {
        if new_width > self.width {
            return Err(Vec2DErr::WidthMismatch(new_width, self.width));
        } else if new_width == self.width {
            return Ok(());
        }

        for row in (0..self.height()).rev() {
            let start = row * self.width + new_width;
            let end = (row + 1) * self.width;

            self.cells.drain(start..end);
        }
        self.width = new_width;

        Ok(())
    }

    #[inline]
    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height()
    }

    /// Returns an iterator over the von Neumann neighborhood (4-connected)
    /// of the cell at `(x, y)`.
    ///
    /// This includes the north, south, east, and west neighbors.
    /// Out of bound neighbors are skipped.
    ///
    /// The values are represented as `((neighbour's coordinates), neighbor's value)`
    pub fn neighbors_von_neumann(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let x = x as isize;
        let y = y as isize;

        const OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        OFFSETS.iter().filter_map(move |(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;

            if self.in_bounds(nx, ny) {
                let (ux, uy) = (nx as usize, ny as usize);

                Some(((ux, uy), &self[(ux, uy)]))
            } else {
                None
            }
        })
    }

    /// Returns an iterator over the Moore neighborhood (8-connected)
    /// of the cell at `(x, y)`.
    ///
    /// This includes all surrounding cells except the center cell itself.
    /// Out of bound neighbors are skipped.
    ///
    /// The values are represented as `((neighbour's coordinates), neighbor's value)`
    pub fn neighbors_moore(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let x = x as isize;
        let y = y as isize;

        const OFFSETS: [(isize, isize); 8] = [
            // Top row
            (-1, -1),
            (0, -1),
            (1, -1),
            // Middle row
            (-1, 0),
            /* (0, 0), */
            (1, 0),
            // Bottom row
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        OFFSETS.iter().filter_map(move |(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;

            if self.in_bounds(nx, ny) {
                let (ux, uy) = (nx as usize, ny as usize);

                Some(((ux, uy), &self[(ux, uy)]))
            } else {
                None
            }
        })
    }
}

impl std::fmt::Display for Vec2DErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec2DErr::EmptySource => write!(f, "Source vector is empty."),
            Vec2DErr::OutOfBounds => {
                write!(f, "Attempted to acces an index which is out of bounds.")
            }
            Vec2DErr::WidthMismatch(width1, width2) => write!(
                f,
                "Widths {} and {} are not compatible for the given task.",
                width1, width2
            ),
            Vec2DErr::ZeroWidth => write!(f, "Width must be bigger than 0."),
            Vec2DErr::ZeroHeight => write!(f, "Height must be bigger than 0."),
        }
    }
}
