struct Rectangle {
    rows: usize,
    columns: usize,
}

fn main() {
    // create a 9x9 array with letters blue ('B') and yellow ('Y')
    // the outer rings are blue and the middle 5x5 is yellow
    let grid: Vec<Vec<char>> = vec![
        vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
        vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
        vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
    ];

    let rect: Rectangle = create_box(&grid, 8, 5);
    println!("{} {}", rect.rows, rect.columns);
}

fn create_box(grid: &Vec<Vec<char>>, starting_row: usize, starting_column: usize) -> Rectangle {
    // define grid dimensions
    let grid_length: usize = grid[0].len();
    let grid_width: usize = grid.len();

    println!(
        "Grid length: {}.    Grid width: {}.",
        grid_length, grid_width
    );

    let mut row: usize = starting_row;
    let mut continue_rows: bool = true;

    let mut column: usize = starting_column;
    let mut continue_columns: bool = true;

    while continue_rows || continue_columns {
        // goes over fields below existing rectangle towards the right
        // increases the rectangle by one line downwards

        if continue_rows {
            for i in starting_column..column {
                if !is_blue(row, i, grid) {
                    continue_rows = false;
                    break;
                }
            }
        }
        //if rectangle has room below, add 1 to row counter
        if continue_rows {
            row += 1;
            println!("Row added! Rows: {}", row);
        }

        // now go over fields on the right, same concept idea as before
        // just on the right of the existing rectangle

        if continue_columns {
            for i in starting_row..row {
                if !is_blue(i, column, grid) {
                    continue_columns = false;
                    break;
                }
            }
        }
        if continue_columns {
            column += 1;
            println!("Column added! Columns: {}", column);
        }

        if row == grid_width || column == grid_length {
            break;
        }
    }

    return Rectangle {
        rows: row,
        columns: column,
    };

    // later add a color to check for Matt color
    fn is_blue(row: usize, column: usize, grid: &Vec<Vec<char>>) -> bool {
        grid[row][column] == 'B'
    }
}
