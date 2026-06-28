// io for user input to detect starting point
use std::io;

struct Grid {
    content: Vec<Vec<char>>,
    height: usize,
    width: usize,
    next_subsection_id: u32,
}

impl Grid {
    fn new(grid_input: Vec<Vec<char>>) -> Self {
        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        Self {
            content: grid_input,
            height,
            width,
            next_subsection_id: 0,
        }
    }

    // function that creates a rectangular area within the grid
    fn create_subsection(&mut self, starting_y: usize, starting_x: usize) -> Option<Subsection> {
        let mut y: usize = starting_y;
        let mut expand_y: bool = true;

        let mut x: usize = starting_x;
        let mut expand_x: bool = true;

        // check for stating color
        let color: char = self.content[y][x];

        if color == 'X' {
            return None;
        }

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

            // goes over fields below existing subsection towards the right
            // increases the subsection by one line downwards
            if expand_y {
                for i in starting_x..x {
                    if !self.is_color(y, i, color) {
                        expand_y = false;
                        break;
                    }
                }
            }
            //if subsection has room below, add 1 to row counter
            if expand_y {
                y += 1;
            }

            // now go over fields on the right, same concept idea as before
            // just on the right of the existing subsection
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

        let sub: Subsection = Subsection {
            _id: self.next_subsection_id,
            color,
            start_y: starting_y,
            start_x: starting_x,
            end_y: y,
            end_x: x,
        };
        self.next_subsection_id += 1;
        Some(sub)
    }

    fn is_color(&self, y: usize, x: usize, color: char) -> bool {
        self.content[y][x] == color
    }
}

// output format for the sub sections of the matt
#[derive(Debug)]
struct Subsection {
    _id: u32,
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
        vec!['X', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'Y', 'Y', 'Y', 'Y', 'Y', 'B', 'B'],
        vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
        vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'],
    ];

    let mut grid: Grid = Grid::new(input_grid);
    let mut subsections: Vec<Subsection> = Vec::new();

    // input handling; probably temporary but good to know for me
    // input numbers are 0 based for indexing

    let mut input_buffer = String::new();

    loop {
        println!("Enter starting x: ");

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line!");

        let starting_x: usize = match input_buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid Number!");
                continue;
            }
        };
        input_buffer.clear();

        println!("Enter starting y: ");

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line!");

        let starting_y: usize = match input_buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid Number!");
                continue;
            }
        };
        input_buffer.clear();

        // creating subsection and printing starting and ending coordinates
        // also handles None in case starting cell is marked "X"(invalid)
        let Some(sub) = grid.create_subsection(starting_y, starting_x) else {
            println!("Starting Cell is invalid!");
            continue;
        };
        println!(
            "start:({},{}); end:({},{}); color: {}",
            sub.start_x, sub.start_y, sub.end_x, sub.end_y, sub.color
        );
        subsections.push(sub);
        println!("current list of Subsections: {:#?}", subsections);
    }
}
