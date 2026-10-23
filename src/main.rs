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

    // CLI gets call
    //      generate documentation for building a floor plan
    //      input parameters are provided

    // program starts - initialization happens
    /*
        initialize ports & adapters
            initialize input floor plan reader
            initialize step by step instructions generator
            initialize packaging instructions generator -- configuration: available delivery size(s)
     */

    // CLI call is processed and triggers domain logic
    /*
        domain call: generate building instructions
            create tatami from input parameters based on the input floor plan
                take input parameters and use input floor plan reader to read from input excel
                take data from input excel and create the tatami struct
                calculate section list and other things necessary for the tatami to be build

        call adapter: step by step instructions generator
            input: groupings
            input: output folder
            input: filename

        call adapter: packaging instructions generator
            input: sections list
            input: output folder
            input: filename
     */

    // CLI call ends program execution

    // let mut floor_plan = load_floor_plan_excel((0, 0), (20, 35))?; // full mat
    //

    let mut floor_plan = load_floor_plan_excel((3, 3), (17, 17))?; // 1 Tatami
    floor_plan.print_to_terminal();

    // TODO: separate floor plan into individual Tatami
    // floor_plan would then be replaced with each tatami

    let mut sections = floor_plan.sectioning(SECTIONING_SIZE);
    sort_sections_diagonally(&mut sections);

    // Grouping = sections that are connected are grouped into one "step" in the final excel
    // Deliveries = in which chunks are tatamis delivered?
    // FIX: Grouping and dividing into deliveries are independant; causes deliveries to be
    //      wrong on the output EXCEL

    let deliveries = floor_plan.generate_deliveries(&sections, DELIVERY_SIZE);

    let pages = floor_plan.generate_pages(&deliveries);
    // Grouping happens here                                 ^^^^^^^^^^^^^^
    // probably also not smart (comprehension)

    write_output_excel(&pages, OUTPUT_FILE_NAME)?;

    Ok(())
}
