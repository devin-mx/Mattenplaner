use crate::models::Grid;
use calamine::{Data, Error, Reader, Xlsx, open_workbook};

pub fn load_grid(start: (usize, usize), end: (usize, usize)) -> Result<Grid, Error> {
    let base: (usize, usize) = (11, 3);

    let mut wb: Xlsx<_> = open_workbook("excel_data/input.xlsx").unwrap();
    let range = wb.worksheet_range("Sheet 1")?;

    let mut input_grid: Vec<Vec<String>> = Vec::new();

    for i in base.0 + start.0..base.0 + end.0 {
        let mut r: Vec<String> = Vec::new();
        for j in base.1 + start.1..base.1 + end.1 {
            r.push(
                range
                    .get((i, j))
                    .unwrap_or(&Data::String("X".to_string()))
                    .to_string(),
            );
        }
        input_grid.push(r);
    }

    let grid = Grid::new(input_grid).expect("Error while creating Grid!");

    Ok(grid)
}
