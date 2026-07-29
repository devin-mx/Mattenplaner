mod models;

use models::*;

fn main() {
    // create a 9x9 array with letters blue ('B') and yellow ('Y')
    // the outer rings are blue and the middle 5x5 is yellow
    //
    // This was my first attempt to implement a Matt plan
    // kept it because it is easy to change
    let input_grid: Vec<Vec<&str>> = vec![
        vec!["B", "B", "B", "B", "B", "B", "B", "B", "BR"],
        vec!["B", "B", "B", "B", "B", "B", "B", "B", "B"],
        vec!["B", "B", "Y", "Y", "Y", "Y", "Y", "B", "B"],
        vec!["B", "B", "Y", "Y", "Y", "Y", "Y", "B", "B"],
        vec!["B", "B", "Y", "Y", "YD", "Y", "Y", "B", "B"],
        vec!["B", "B", "Y", "Y", "YU", "Y", "Y", "B", "B"],
        vec!["B", "B", "Y", "Y", "Y", "YR", "YL", "B", "B"],
        vec!["BD", "B", "B", "B", "B", "B", "B", "B", "B"],
        vec!["BU", "BR", "BL", "B", "B", "B", "B", "B", "B"],
    ];

    let mut grid: Grid = Grid::new(input_grid, Some(10));

    println!("{}", grid);

    grid.set_print_intervals(2);

    grid.build_diagonally();
}
