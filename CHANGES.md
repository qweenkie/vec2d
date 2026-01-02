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
