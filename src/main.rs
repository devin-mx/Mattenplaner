use std::collections::VecDeque;
use std::fmt;

// Grid represents the matt plan;
struct Grid {
    content: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    next_subsection_id: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.content {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    // common theme; creating a new method for instanciating a new struct
    fn new(grid_input: Vec<Vec<char>>) -> Self {
        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for (y, row) in grid_input.iter().enumerate() {
            let mut r = Vec::new();
            for (x, item) in row.iter().enumerate() {
                let color = Color::from(*item);
                let cell = Cell {
                    location: CellCoordinate { x, y },
                    color,
                    allocated_by: None,
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
    fn _custom_subsectioning(&mut self, seed_cell: CellCoordinate) -> Option<Subsection> {
        /*
         * Starts at seed
         * Checks if right of the already allocated area is available
         *   if yes: allocates it; if no: stops trying to expand in that direction
         * Same idea, but checking down
         * repeat
         */

        let mut y: usize = seed_cell.y;
        let mut expand_y: bool = true;

        let mut x: usize = seed_cell.x;
        let mut expand_x: bool = true;

        // check for stating color
        let color = self.content[y][x].color;

        if !self.is_available(y, x, color) {
            return None;
        }

        while expand_y || expand_x {
            // goes over fields below existing subsection towards the right
            // increases the subsection by one line downwards
            if expand_y {
                for i in seed_cell.x..x {
                    if !self.is_available(y, i, color) {
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
                for i in seed_cell.y..y {
                    if !self.is_available(i, x, color) {
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

        let mut cells: Vec<CellCoordinate> = Vec::new();
        for i in seed_cell.y..y {
            for j in seed_cell.x..x {
                self.content[i][j].allocated_by = Some(self.next_subsection_id);
                cells.push(self.content[i][j].location.clone());
            }
        }

        let sub = Subsection {
            color,
            _id: self.next_subsection_id,
            content: cells,
        };

        Some(sub)
    }

    fn _flood_fill(
        &mut self,
        location: CellCoordinate,
        color: Color,
        max_size: usize,
    ) -> Subsection {
        let mut cells_in_subsection: Vec<CellCoordinate> = Vec::new();
        let mut queue: VecDeque<CellCoordinate> = VecDeque::new();

        queue.push_back(location);

        while let Some(cell_location) = queue.pop_front() {
            let cell: Cell = self.content[cell_location.y][cell_location.x].clone();

            if self.is_available(cell_location.y, cell_location.x, color) {
                self.content[cell_location.y][cell_location.x].allocated_by =
                    Some(self.next_subsection_id);

                cells_in_subsection.push(cell.location);

                if cells_in_subsection.len() >= max_size {
                    break;
                }

                if cell_location.x + 1 < self.width {
                    queue.push_back(CellCoordinate {
                        x: cell_location.x + 1,
                        y: cell_location.y,
                    });
                }

                if cell_location.x != 0 {
                    queue.push_back(CellCoordinate {
                        x: cell_location.x - 1,
                        y: cell_location.y,
                    });
                }

                if cell_location.y + 1 < self.height {
                    queue.push_back(CellCoordinate {
                        x: cell_location.x,
                        y: cell_location.y + 1,
                    });
                }

                if cell_location.y != 0 {
                    queue.push_back(CellCoordinate {
                        x: cell_location.x,
                        y: cell_location.y - 1,
                    });
                }
            }
        }

        Subsection {
            _id: self.next_subsection_id,
            color,
            content: cells_in_subsection,
        }
    }

    fn is_available(&self, y: usize, x: usize, color: Color) -> bool {
        if color == Color::None {
            return false;
        }
        if x >= self.width || y >= self.height {
            return false;
        }
        self.content[y][x].color == color && self.content[y][x].allocated_by.is_none()
    }
}

// output format for the sub sections of the matt
#[derive(Debug)]
struct Subsection {
    _id: usize,
    color: Color,
    content: Vec<CellCoordinate>,
}

impl fmt::Display for Subsection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in &self.content {
            write!(f, "{}, ", cell)?;
        }
        writeln!(f)?;
        writeln!(f, "ID: {}; Color: {}", self._id, self.color)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Blue,
    Yellow,
    None,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blue => write!(f, "B"),
            Self::Yellow => write!(f, "Y"),
            Self::None => write!(f, "N"),
        }
    }
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

#[derive(Clone)]
struct Cell {
    location: CellCoordinate,
    color: Color,
    allocated_by: Option<usize>,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:?}) ", self.color, self.allocated_by)
    }
}

#[derive(Clone, Debug)]
struct CellCoordinate {
    y: usize,
    x: usize,
}

impl fmt::Display for CellCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    // create a 9x9 array with letters blue ('B') and yellow ('Y')
    // the outer rings are blue and the middle 5x5 is yellow
    //
    // This was my first attempt to implement a Matt plan
    // kept it because it is easy to cahnge
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

    let mut grid: Grid = Grid::new(input_grid);
    let mut subsections: Vec<Subsection> = Vec::new();

    // loop over every Cell and use it as seed if available
    println!("{}", grid);

    for i in 0..grid.height {
        for j in 0..grid.width {
            let cell = grid.content[i][j].clone();
            if !grid.is_available(cell.location.y, cell.location.x, cell.color) {
                continue;
            }
            let sub = grid._flood_fill(cell.location, cell.color, 13);
            //let sub = grid._custom_subsectioning(cell.location).unwrap();
            subsections.push(sub);
            grid.next_subsection_id += 1;
        }
    }

    for s in subsections {
        println!("{}", s);
    }
    println!("{}", grid);
}
