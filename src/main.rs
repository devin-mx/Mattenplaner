// io for user input to detect starting point
use std::io;

struct Grid {
    content: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(grid_input: Vec<Vec<char>>) -> Self {
        let width: usize = grid_input.len();
        let height: usize = if width > 0 { grid_input[0].len() } else { 0 };

        Self {
            content: grid_input,
            height,
            width,
        }
    }

    // function that creates a rectangular area within the grid
    fn create_subsection(&self, starting_y: usize, starting_x: usize) -> Rectangle {
        let mut y: usize = starting_y;
        let mut expand_y: bool = true;

        let mut x: usize = starting_x;
        let mut expand_x: bool = true;

        // check for stating color
        let color: char = self.content[y][x];

        while expand_y || expand_x {
            // check for boundries (-1 because dimensions are 1 based and y are 0 based [indexes])
            if y == (self.height - 1) && x == (self.width - 1) {
                break;
            }

            if y == (self.height - 1) {
                expand_y = false;
            }

            if x == (self.width - 1) {
                expand_x = false;
            }

            // goes over fields below existing rectangle towards the right
            // increases the rectangle by one line downwards
            if expand_y {
                for i in starting_x..x {
                    if !self.is_color(y, i, color) {
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
                    if !self.is_color(i, x, color) {
                        expand_x = false;
                        break;
                    }
                }
            }
            if expand_x {
                x += 1;
            }
        }

        Rectangle {
            color,
            start_y: starting_y,
            start_x: starting_x,
            end_y: y,
            end_x: x,
        }
    }

    fn is_color(&self, y: usize, x: usize, color: char) -> bool {
        self.content[y][x] == color
    }
}

// output format for the sub sections of the matt
#[derive(Debug)]
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
    let input_grid: Vec<Vec<char>> = vec![
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

    let grid: Grid = Grid::new(input_grid);
    let mut subsections: Vec<Rectangle> = Vec::new();

    // input handling; probably temporary but good to know for me
    // input numbers are 0 based for indexing
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

        // creating subsection and printing starting and ending coordinates
        let rect: Rectangle = grid.create_subsection(starting_y, starting_x);
        println!(
            "start:({},{}); end:({},{}); color: {}",
            rect.start_x, rect.start_y, rect.end_x, rect.end_y, rect.color
        );

        subsections.push(rect);
        println!("current list of Subsections: {:#?}", subsections);
    }
}
