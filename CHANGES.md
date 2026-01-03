# Changelog

## 0.1.0
- Added functions `insert_row` and `insert_row_with_copy`.
- Slightly improved the docs for the neighborhood functions.

## 0.2.0 => 0.2.1
- Renamed `insert_row` and `insert_row_with_copy` to `extend` and
  `extend_cloned` respectively. Slightly changed the way the functions
  work.
- Added functions `insert_row` and `insert_row_cloned`, which insert
  a row at a given index.
- Wrote unit tests for `extend`, `extend_cloned`, `insert_row`, and
  `insert_row_cloned`.

## 0.3.0
- Added utilities for removing rows, and truncating (cutting off)
  rows/columns.
- Deprecated `cells_mut`, since it allows for more bugs than I would
  like.
- Added unit tests for all newly introduced functions.

## 0.4.0
- Fixed major bug in `coords`, where `x` and `y` values were
  mismatched.
- Reduced the amount of unnecessary bounds checks in `get`, `get_mut`,
  `get_row`, and `get_row_mut`.
- Removed the redundant requirement for `T: Clone` in `remove_row`.
- Added `ZeroWidth` and `ZeroHeight` errors for `truncate_rows` and
  `truncate_cols` respectively.
- Massively optimized `truncate_cols` by reducing the time complexity
  from `O(n ^ 2)` to `O(n)` *(while mantaining the `O(1)` space
  complexity)*.
- Fixed bug in private `in_bounds` function used for bounds checking,
  where an overflow was possible when casting from `usize` to `isize`.
- Fixed typo in implementation of `std::fmt::Display` for
  `Vec2DErr::OutOfBounds`.
- Added extra unit tests to assert bounds checking logic.

