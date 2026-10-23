mod models;
mod reader;
mod writer;

use anyhow::{Ok, Result};
use reader::load_floor_plan_excel;
use writer::write_output_excel;

use crate::models::*;

fn main() -> Result<()> {
    const DELIVERY_SIZE: usize = 40;
    const SECTIONING_SIZE: usize = 2;
    const OUTPUT_FILE_NAME: &str = "someName";

    // let mut floor_plan = load_floor_plan_excel((0, 0), (20, 35))?; // full mat
    //

    let mut floor_plan = load_floor_plan_excel((3, 3), (17, 17))?; // 1 Tatami
    floor_plan.print_to_terminal();

    // TODO: separate floor plan into individual Tatami
    // floor_plan would then be replaced with each tatami

    let mut sections = floor_plan.sectioning(SECTIONING_SIZE);
    sort_sections_diagonally(&mut sections);

    // FIX: Grouping and deviding into deliveries are independant; casues deliveries to be
    //      wrong on the output EXCEL

    let deliveries = floor_plan.generate_deliveries(&sections, DELIVERY_SIZE);

    let pages = floor_plan.generate_pages(&deliveries);
    // Grouping happens here                                 ^^^^^^^^^^^^^^
    // probably also not smart (comprehension)

    write_output_excel(&pages, OUTPUT_FILE_NAME)?;

    Ok(())
}
