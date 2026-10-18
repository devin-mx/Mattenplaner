mod models;
mod reader;
mod writer;

use anyhow::Result;
use reader::load_floor_plan_excel;
use writer::write_output_excel;

fn main() -> Result<()> {
    const DELIVERY_SIZE: usize = 40;
    const SECTIONING_SIZE: usize = 2;
    const OUTPUT_FILE_NAME: &str = "Bob";

    // let mut floor_plan = load_floor_plan_excel((0, 0), (20, 35))?; // full mat
    // Do not use! Currently too big for excel

    let mut floor_plan = load_floor_plan_excel((3, 3), (17, 17))?; // 1 Tatami
    floor_plan.print_to_terminal();

    // TODO: separate floor plan into individual Tatami

    // floor_plan would be replaced with tatami

    let _deliveries = floor_plan.generate_deliveries(DELIVERY_SIZE, SECTIONING_SIZE);

    let pages = floor_plan.generate_pages(DELIVERY_SIZE, SECTIONING_SIZE);

    write_output_excel(&pages, OUTPUT_FILE_NAME)?;
    Ok(())
}
