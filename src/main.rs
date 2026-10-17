mod models;
mod reader;
mod writer;

use std::process::exit;

use calamine::Error;
use reader::*;
use writer::writer_function;

fn main() -> Result<(), Error> {
    let mut grid = load_grid((0, 0), (20, 35))?; // full mat

    // let mut grid = load_grid((3, 3), (17, 17))?; // 1 Tatami

    let pages = match grid.print_pages(40, 2) {
        Ok(pages) => pages,
        Err(_) => exit(1),
    };

    let _ = writer_function(&pages, "David");
    Ok(())
}
