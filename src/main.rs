use std::io;

struct Rectangle {
    color: char,
    start_y: usize,
    start_x: usize,
    end_x: usize,
    end_y: usize,
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

    loop {
        println!("Enter starting x: ");
        let mut x_in: String = String::new();

        io::stdin()
            .read_line(&mut x_in)
            .expect("Failed to read line!");

        let starting_x: usize = match x_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid Number!");
                continue;
            }
        };

        println!("Enter starting y: ");
        let mut y_in: String = String::new();

        io::stdin()
            .read_line(&mut y_in)
            .expect("Failed to read line!");

        let starting_y: usize = match y_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid Number!");
                continue;
            }
        };

        let rect: Rectangle = create_subsection(&grid, starting_y, starting_x);
        println!(
            "start:({},{}); end:({},{}); color: {}",
            rect.start_x, rect.start_y, rect.end_x, rect.end_y, rect.color
        );
    }
}

fn create_subsection(grid: &Vec<Vec<char>>, starting_y: usize, starting_x: usize) -> Rectangle {
    // define grid dimensions
    let grid_length: usize = grid[0].len();
    let grid_width: usize = grid.len();

    let mut y: usize = starting_y;
    let mut expand_y: bool = true;

    let mut x: usize = starting_x;
    let mut expand_x: bool = true;

    // check for stating color
    let color: char = grid[y][x];

    while expand_y || expand_x {
        // goes over fields below existing rectangle towards the right
        // increases the rectangle by one line downwards

        if expand_y {
            for i in starting_x..x {
                if !is_color(y, i, color, grid) {
                    expand_y = false;
                    break;
                }
            }
        }
        //if rectangle has room below, add 1 to row counter
        if expand_y {
            y += 1;
        }

        // now go over fields on the right, same concept idea as before
        // just on the right of the existing rectangle

        if expand_x {
            for i in starting_y..y {
                if !is_color(i, x, color, grid) {
                    expand_x = false;
                    break;
                }
            }
        }
        if expand_x {
            x += 1;
        }

        if y == grid_width || x == grid_length {
            break;
        }
    }

    return Rectangle {
        color: color,
        start_y: starting_y,
        start_x: starting_x,
        end_y: y,
        end_x: x,
    };

    // later add a color to check for Matt color
    fn is_color(y: usize, x: usize, color: char, grid: &Vec<Vec<char>>) -> bool {
        grid[y][x] == color
    }
}
