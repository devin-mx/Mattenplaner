mod models;

use models::*;

fn main() {
    // create a 9x9 array with letters blue ('B') and yellow ('Y')
    // the outer rings are blue and the middle 5x5 is yellow
    //
    // This was my first attempt to implement a Matt plan
    // kept it because it is easy to change
    let input_grid: Vec<Vec<&str>> = vec![
        vec!["B", "B", "B", "B", "B", "B", "B", "B", "B"],
        vec!["B", "B", "B", "B", "B", "B", "B", "B", "B"],
        vec!["B", "B", "YR", "YL", "Y", "Y", "Y", "B", "B"],
        vec!["B", "B", "YD", "Y", "Y", "Y", "Y", "B", "B"],
        vec!["B", "B", "YU", "Y", "Y", "Y", "YD", "B", "B"],
        vec!["B", "B", "Y", "Y", "Y", "Y", "YU", "B", "B"],
        vec!["B", "B", "Y", "Y", "Y", "YR", "YL", "B", "B"],
        vec!["B", "B", "B", "B", "B", "B", "B", "B", "B"],
        vec!["B", "B", "B", "B", "B", "B", "B", "B", "B"],
    ];

    let mut grid: Grid = match Grid::new(input_grid, 10) {
        Ok(g) => g,
        Err(e) => {
            println!("Error: {e}");
            std::process::exit(1);
        }
    };

    println!("{}", grid);

    let sections = match grid.build_diagonally() {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {e}");
            std::process::exit(1);
        }
    };
    for s in sections {
        println!("{s}");
    }

    println!();

    let deliveries = match grid.generate_deliveryies() {
        Ok(d) => d,
        Err(e) => {
            println!("Error: {e}");
            std::process::exit(1);
        }
    };
    for delivery in deliveries {
        for c in delivery {
            print!("{c}, ");
        }
        println!();
        println!();
    }
}
