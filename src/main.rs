mod models;
mod reader;
mod writer;

use calamine::Error;
use reader::*;
use writer::writer_function;

fn main() -> Result<(), Error> {
    // create a 9x9 array with letters blue ('B') and yellow ('Y')
    // the outer rings are blue and the middle 5x5 is yellow
    //
    // This was my first attempt to implement a Matt plan
    // kept it because it is easy to change
    /*
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

    let mut grid: Grid = match Grid::new(input_grid) {
        Ok(g) => g,
        Err(e) => {
            println!("Error: {e}");
            std::process::exit(1);
        }
    };

    */

    let mut grid = load_grid((0, 0), (20, 35))?; // full mat

    // let mut grid = load_grid((3, 3), (17, 17))?; // 1 Tatami

    println!("Sections:");

    let sections = match grid.build_diagonally(2) {
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

    println!("Deliveries:");

    let deliveries = match grid.generate_deliveries(40, 2) {
        Ok(d) => d,
        Err(e) => {
            println!("Error: {e}");
            std::process::exit(1);
        }
    };
    for delivery in deliveries {
        let mut counter = 0;

        for (i, c) in delivery {
            print!("{i} {c}, ");
            counter += i;
        }
        println!();

        println!("Size: {}", counter);

        println!();
    }

    let _ = writer_function(grid);

    Ok(())
}
