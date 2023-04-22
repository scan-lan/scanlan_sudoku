use std::collections::HashSet;

use super::{get_base_solution, get_box_coords_containing, Cell, Grid, Group};
use crate::logic::{grid_trait::GridTrait, puzzle::Coord, SIZE};

#[test]
fn get_box_containing_returns_correct_coords() {
    let c: Coord = (0, 0).into();
    let expected: [Coord; SIZE] = [
        (0, 0).into(),
        (0, 1).into(),
        (0, 2).into(),
        (1, 0).into(),
        (1, 1).into(),
        (1, 2).into(),
        (2, 0).into(),
        (2, 1).into(),
        (2, 2).into(),
    ];

    assert_eq!(get_box_coords_containing(c), expected);
}

#[test]
fn update_cell() {
    let mut expected = [[Cell::Empty; SIZE]; SIZE];
    expected[4][4] = Cell::Filled(1);
    expected[4][5] = Cell::Filled(1);
    let mut g = Grid::new();

    g.update(Coord { row: 4, col: 4 }, 1).unwrap();
    g.update(Coord { row: 4, col: 5 }, 1).unwrap();

    assert_eq!(g.rows(), &expected);
}

#[test]
fn update_keeps_all_groups_synced() {
    let mut g = Grid::new();
    g.update(Coord { row: 1, col: 1 }, 1).unwrap();
    g.update(Coord { row: 2, col: 3 }, 9).unwrap();

    assert_eq!(g.rows[1][1], g.cols[1][1]);
    assert_eq!(g.cols[1][1], g.boxes[0][4]);
    assert_eq!(g.rows[2][3], g.cols[3][2]);
    assert_eq!(g.cols[3][2], g.boxes[1][6]);
}

#[test]
fn candidate_matrix_correct_for_from() {
    let expected = get_base_solution().map(|row| {
        row.map(|cell| match cell {
            Cell::Clue(n) => HashSet::from([n]),
            _ => HashSet::with_capacity(SIZE),
        })
    });
    let g = Grid::from(get_base_solution());

    assert_eq!(expected, g.candidate_matrix);
}

#[test]
fn update_gives_correct_candidate_matrix() {
    let mut g = Grid::new();
    g.update(Coord { row: 4, col: 5 }, 9).unwrap();

    assert!(g.candidate_matrix[4]
        .iter()
        .all(|candidates| !candidates.contains(&9)));
    assert!(g.candidate_matrix.iter().all(|row| !row[5].contains(&9)));
    assert!(g.candidate_matrix.boxes()[4]
        .iter()
        .all(|candidates| !candidates.contains(&9)))
}

#[test]
fn empty_cell_count_correct_after_from() {
    let mut rows = get_base_solution();
    for cell in rows.iter_mut().flatten().take(5) {
        *cell = Cell::Empty;
    }
    let g = Grid::from(rows);

    assert_eq!(g.empty_cell_count, 5);
}

#[test]
fn empty_cell_count_correct_after_update() {
    let mut g = Grid::new();
    g.update((0, 0).into(), 9).unwrap();

    assert_eq!(g.empty_cell_count, 80);
}

#[test]
fn from_returns_correct_vals() {
    let expected_rows = [
        [
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
        ],
        [
            Cell::Clue(4),
            Cell::Empty,
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
        ],
        [
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
            Cell::Clue(6),
        ],
        [
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
        ],
        [
            Cell::Clue(5),
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
        ],
        [
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
            Cell::Clue(6),
            Cell::Clue(7),
        ],
        [
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
        ],
        [
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
        ],
        [
            Cell::Clue(9),
            Cell::Clue(1),
            Cell::Clue(2),
            Cell::Clue(3),
            Cell::Clue(4),
            Cell::Clue(5),
            Cell::Clue(6),
            Cell::Clue(7),
            Cell::Clue(8),
        ],
    ];
    let expected_cols = [
        [
            Cell::Clue(1),
            Cell::Clue(4),
            Cell::Clue(7),
            Cell::Clue(2),
            Cell::Clue(5),
            Cell::Clue(8),
            Cell::Clue(3),
            Cell::Clue(6),
            Cell::Clue(9),
        ],
        [
            Cell::Clue(2),
            Cell::Empty,
            Cell::Clue(8),
            Cell::Clue(3),
            Cell::Clue(6),
            Cell::Clue(9),
            Cell::Clue(4),
            Cell::Clue(7),
            Cell::Clue(1),
        ],
        [
            Cell::Clue(3),
            Cell::Clue(6),
            Cell::Clue(9),
            Cell::Clue(4),
            Cell::Clue(7),
            Cell::Clue(1),
            Cell::Clue(5),
            Cell::Clue(8),
            Cell::Clue(2),
        ],
        [
            Cell::Clue(4),
            Cell::Clue(7),
            Cell::Clue(1),
            Cell::Clue(5),
            Cell::Clue(8),
            Cell::Clue(2),
            Cell::Clue(6),
            Cell::Clue(9),
            Cell::Clue(3),
        ],
        [
            Cell::Clue(5),
            Cell::Clue(8),
            Cell::Clue(2),
            Cell::Clue(6),
            Cell::Clue(9),
            Cell::Clue(3),
            Cell::Clue(7),
            Cell::Clue(1),
            Cell::Clue(4),
        ],
        [
            Cell::Clue(6),
            Cell::Clue(9),
            Cell::Clue(3),
            Cell::Clue(7),
            Cell::Clue(1),
            Cell::Clue(4),
            Cell::Clue(8),
            Cell::Clue(2),
            Cell::Clue(5),
        ],
        [
            Cell::Clue(7),
            Cell::Clue(1),
            Cell::Clue(4),
            Cell::Clue(8),
            Cell::Clue(2),
            Cell::Clue(5),
            Cell::Clue(9),
            Cell::Clue(3),
            Cell::Clue(6),
        ],
        [
            Cell::Clue(8),
            Cell::Clue(2),
            Cell::Clue(5),
            Cell::Clue(9),
            Cell::Clue(3),
            Cell::Clue(6),
            Cell::Clue(1),
            Cell::Clue(4),
            Cell::Clue(7),
        ],
        [
            Cell::Clue(9),
            Cell::Clue(3),
            Cell::Clue(6),
            Cell::Clue(1),
            Cell::Clue(4),
            Cell::Clue(7),
            Cell::Clue(2),
            Cell::Clue(5),
            Cell::Clue(8),
        ],
    ];
    let mut expected_boxes = expected_rows.to_owned();
    expected_boxes[0][4] = Cell::Empty;
    expected_boxes[1][1] = Cell::Clue(5);
    let mut rows = get_base_solution();
    rows[1][1] = Cell::Empty;
    let g = Grid::from(rows);

    assert_eq!(g.rows, expected_rows);
    assert_eq!(g.cols, expected_cols);
    assert_eq!(g.boxes, expected_boxes);
}
