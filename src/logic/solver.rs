use super::{puzzle::Coord, Grid};

fn get_min_candidates_cell(g: &Grid) -> Coord {
    let mut coords = Coord { row: 0, col: 0 };
    let mut min = usize::MAX;

    for (i, row) in g.candidate_matrix().iter().enumerate() {
        for (j, candidates) in row.iter().enumerate() {
            if candidates.len() < min {
                min = candidates.len();
                coords = Coord { row: i, col: j };
            }
        }
    }

    coords
}

pub fn solve_backtracking_heuristics(g: Grid) -> Grid {
    while g.empty_cell_count > 0 {}
    let target_cell = get_min_candidates_cell(&g);
    g
}
