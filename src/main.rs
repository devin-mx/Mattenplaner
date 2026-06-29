// io for user input to detect starting point
use std::io;

struct Grid {
    content: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    next_subsection_id: u32,
}

impl Grid {
    fn new(grid_input: Vec<Vec<char>>) -> Self {
        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for row in grid_input {
            let mut r = Vec::new();
            for item in row {
                let color = Color::from(item);
                let cell = Cell {
                    color,
                    owned_by: None,
                };
                r.push(cell);
            }
            grid.push(r);
        }

        Self {
            content: grid,
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
        let color: &Color = &self.content[y][x].color;

        if color == &Color::None {
            return None;
        }

        while expand_y || expand_x {
            // check for boundries (-1 because dimensions are 1 based and y are 0 based [indexes])
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

            if y == self.height && x == self.width {
                break;
            }

            if y == self.height {
                expand_y = false;
            }

            if x == self.width {
                expand_x = false;
            }
        }

        let sub: Subsection = Subsection {
            _id: self.next_subsection_id,
            color: color.clone(),
            start_y: starting_y,
            start_x: starting_x,
            end_y: y,
            end_x: x,
        };
        self.next_subsection_id += 1;
        Some(sub)
    }

    fn is_color(&self, y: usize, x: usize, color: &Color) -> bool {
        &self.content[y][x].color == color
    }
}

// output format for the sub sections of the matt
#[derive(Debug)]
struct Subsection {
    _id: u32,
    color: Color,
    start_y: usize,
    start_x: usize,
    // The end coordinates are exclusive
    // dont panic over an end_x being 9 for an 9x9 grid
    end_x: usize,
    end_y: usize,
}
#[derive(Debug, PartialEq, Clone)]
enum Color {
    Blue,
    Yellow,
    None,
}

impl From<char> for Color {
    fn from(item: char) -> Self {
        match item {
            'B' => Color::Blue,
            'Y' => Color::Yellow,
            _ => Color::None,
        }
    }
}

struct Cell {
    color: Color,
    owned_by: Option<usize>,
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
            "start:({},{}); end:({},{}); color: {:?}",
            sub.start_x, sub.start_y, sub.end_x, sub.end_y, sub.color
        );
        subsections.push(sub);
        println!("current list of Subsections: {:#?}", subsections);
    }
}
