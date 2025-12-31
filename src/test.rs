#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new_creates_correct_size() {
        let grid: Vec2D<u8> = Vec2D::new(4, 3).unwrap();

        assert_eq!(grid.width(), 4);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.cells().len(), 12);
    }

    #[test]
    fn new_errors_on_zero_dimensions() {
        assert!(matches!(Vec2D::<u8>::new(0, 5), Err(Vec2DErr::ZeroWidth)));
        assert!(matches!(Vec2D::<u8>::new(5, 0), Err(Vec2DErr::ZeroHeight)));
    }

    #[test]
    fn from_vec_valid() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let grid = Vec2D::from_vec(data.clone(), 3).unwrap();

        assert_eq!(grid.cells(), data);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
    }

    #[test]
    fn from_vec_invalid_width() {
        let data = vec![1, 2, 3, 4, 5];

        assert!(matches!(
            Vec2D::from_vec(data, 3),
            Err(Vec2DErr::WidthMismatch(5, 3))
        ));
    }

    #[test]
    fn indexing_valid() {
        let grid = Vec2D::from_vec((0..9).collect(), 3).unwrap();

        assert_eq!(grid[(0, 0)], 0);
        assert_eq!(grid[(1, 0)], 1);
        assert_eq!(grid[(2, 1)], 5);
        assert_eq!(grid[(2, 2)], 8);
    }

    #[test]
    #[should_panic]
    fn indexing_panics_out_of_bounds_x() {
        let grid: Vec2D<u8> = Vec2D::new(3, 3).unwrap();

        let _ = grid[(3, 0)];
    }

    #[test]
    #[should_panic]
    fn indexing_panics_out_of_bounds_y() {
        let grid: Vec2D<u8> = Vec2D::new(3, 3).unwrap();

        let _ = grid[(0, 3)];
    }

    #[test]
    fn get_works() {
        let grid = Vec2D::new_with_default(3, 3, 1).unwrap();

        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(3, 0), None);
        assert_eq!(grid.get(0, 3), None);
        assert_eq!(grid.get(3, 3), None);
    }

    #[test]
    fn get_mut_works() {
        let mut grid = Vec2D::new_with_default(3, 3, 1).unwrap();

        let mutable_ref = grid.get_mut(0, 0).unwrap();
        assert_eq!(mutable_ref, &1);

        *mutable_ref = 42;
        assert_eq!(grid.get(0, 0), Some(&42))
    }

    #[test]
    fn get_row_works() {
        let grid = Vec2D::from_vec((0..6).collect(), 3).unwrap();

        assert_eq!(grid.get_row(0).unwrap(), &[0, 1, 2]);
        assert_eq!(grid.get_row(1).unwrap(), &[3, 4, 5]);
        assert_eq!(grid.get_row(2), None);
    }

    #[test]
    fn extend_works() {
        let mut grid = Vec2D::from_vec((0..6).collect(), 3).unwrap();

        let to_append: Vec<_> = (6..9).collect();
        grid.extend(to_append).unwrap();

        assert_eq!(grid.cells(), Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn insert_row_works() {
        let mut grid = Vec2D::from_vec(Vec::from([0, 1, 2, 6, 7, 8]), 3).unwrap();

        let to_insert: Vec<_> = (3..6).collect();
        grid.insert_row(1, to_insert).unwrap();

        assert_eq!(grid.cells(), Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn iter_xy_visits_all_cells() {
        let grid = Vec2D::from_vec((0..4).collect(), 2).unwrap();

        let collected: Vec<_> = grid.iter_xy().collect();
        assert_eq!(
            collected,
            vec![((0, 0), &0), ((1, 0), &1), ((0, 1), &2), ((1, 1), &3),]
        );
    }

    #[test]
    fn iter_rows_works() {
        let grid = Vec2D::from_vec((0..6).collect(), 3).unwrap();
        let rows: Vec<_> = grid.iter_rows().collect();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], &[0, 1, 2]);
        assert_eq!(rows[1], &[3, 4, 5]);
    }

    #[test]
    fn map_in_place_applies_to_all_cells() {
        let mut grid = Vec2D::new_with_default(2, 2, 1).unwrap();

        grid.map_in_place(|x| *x *= 2);
        assert_eq!(grid.cells(), &[2, 2, 2, 2]);
    }

    #[test]
    fn neighbors_von_neumann_center() {
        let grid = Vec2D::from_vec((0..9).collect(), 3).unwrap();

        let neighbors: Vec<_> = grid.neighbors_von_neumann(1, 1).collect();
        let coords: Vec<_> = neighbors.iter().map(|(c, _)| *c).collect();

        assert_eq!(coords, vec![(1, 0), (0, 1), (2, 1), (1, 2)]);
    }

    #[test]
    fn neighbors_von_neumann_corner() {
        let grid = Vec2D::from_vec((0..9).collect(), 3).unwrap();

        let neighbors: Vec<_> = grid.neighbors_von_neumann(0, 0).collect();
        let coords: Vec<_> = neighbors.iter().map(|(c, _)| *c).collect();

        assert_eq!(coords, vec![(1, 0), (0, 1)]);
    }

    #[test]
    fn neighbors_moore_center() {
        let grid = Vec2D::from_vec((0..9).collect(), 3).unwrap();

        let neighbors: Vec<_> = grid.neighbors_moore(1, 1).collect();
        let coords: Vec<_> = neighbors.iter().map(|(c, _)| *c).collect();

        assert_eq!(
            coords,
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
            ]
        );
    }

    #[test]
    fn neighbors_moore_corner() {
        let grid = Vec2D::from_vec((0..9).collect(), 3).unwrap();

        let neighbors: Vec<_> = grid.neighbors_moore(0, 0).collect();
        let coords: Vec<_> = neighbors.iter().map(|(c, _)| *c).collect();

        assert_eq!(coords, vec![(1, 0), (0, 1), (1, 1)]);
    }
}
