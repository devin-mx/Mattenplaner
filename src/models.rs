use colored::Colorize;
use std::fmt;

// Grid represents the matt plan;
pub struct Grid {
    content: Vec<Vec<Cell>>,
    delivery: Delivery,
    height: usize,
    width: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Blue,
    Yellow,
    None,
}

#[derive(Clone, Debug)]
struct Cell {
    position: CellCoordinate,
    color: Color,
    owned: bool,
}

#[derive(Clone, Debug)]
struct CellCoordinate {
    y: usize,
    x: usize,
}

// todo: learn Lifetime annotation
//          - this would allow to store references in delivery
//          - thus allowing to change Cell.owned within add whch would be more logical (I think)
//          - do not be confused; this is mostly a concern for readability and display logic
//
struct Delivery {
    current_load: Vec<Cell>,
    loads: Vec<Vec<Cell>>,
    max_size: usize,
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.owned {
            false => {
                write!(f, "[{}] ", self.color)
            }
            true => {
                let s = format!("[{}] ", self.color);
                match self.color {
                    Color::Blue => {
                        write!(f, "{}", s.truecolor(0, 55, 200))
                    }
                    Color::Yellow => {
                        write!(f, "{}", s.truecolor(255, 255, 0))
                    }
                    Color::None => {
                        write!(f, "{}", s)
                    }
                }
            }
        }
    }
}

impl fmt::Display for CellCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Delivery {
    pub fn new(max_size: usize) -> Self {
        Self {
            current_load: Vec::new(),
            loads: Vec::new(),
            max_size,
        }
    }

    fn add(&mut self, mut added_content: Vec<Cell>) {
        while added_content.len() + self.current_load.len() >= self.max_size {
            let remaining_space = self.max_size - self.current_load.len();
            self.current_load
                .extend(added_content.drain(..remaining_space));

            self.loads.push(self.current_load.clone());
            self.current_load.clear();
        }

        self.current_load.extend(added_content);
    }
}

impl Grid {
    // common theme; creating a new method for instanciating a new struct
    pub fn new(grid_input: Vec<Vec<char>>, max_delivery_size: Option<usize>) -> Self {
        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        let mut grid: Vec<Vec<Cell>> = Vec::new();

        for (y, row) in grid_input.iter().enumerate() {
            let mut r = Vec::new();
            for (x, item) in row.iter().enumerate() {
                let color = Color::from(*item);
                let cell = Cell {
                    position: CellCoordinate { x, y },
                    color,
                    owned: false,
                };
                r.push(cell);
            }
            grid.push(r);
        }

        let delivery = Delivery::new(max_delivery_size.unwrap_or(40));

        Self {
            content: grid,
            delivery,
            height,
            width,
        }
    }

    pub fn build_diagonally(&mut self) {
        let (tatami_start_x, tatami_start_y) = {
            let start = self.find_first_tatami_matt().unwrap();
            (start.x, start.y)
        };

        let mut top_left_corner_area: Vec<Cell> = Vec::new();
        for row in self.content[..tatami_start_y].iter_mut() {
            for cell in row[..tatami_start_x].iter_mut() {
                cell.owned = true;
                top_left_corner_area.push(cell.clone());
            }
        }
        self.delivery.add(top_left_corner_area);

        let mut x: usize = tatami_start_x;
        let mut y: usize = tatami_start_y;

        while x < self.width && y < self.height {
            if x < self.width {
                let add_cells = self.expand_right(x, tatami_start_y);
                self.delivery.add(add_cells);
                x += 1;
            }

            if y < self.height {
                let add_cells = self.expand_down(tatami_start_x, y);
                self.delivery.add(add_cells);
                y += 1;
            }

            println!("{}", self);

            println!(
                "Curreny Delivery Size: {}",
                self.delivery.current_load.len()
            );
            println!("Delivery Count: {}", self.delivery.loads.len());
        }
    }

    fn find_first_tatami_matt(&self) -> Option<&CellCoordinate> {
        for cell in self.content.iter().flatten() {
            if cell.color == Color::Yellow {
                return Some(&cell.position);
            }
        }
        None
    }

    fn expand_right(&mut self, x: usize, y: usize) -> Vec<Cell> {
        let mut v = Vec::new();
        for row in self.content[..y].iter_mut() {
            row[x].owned = true;
            v.push(row[x].clone());
        }
        // let v: Vec<Cell> = self.content[..y].iter().map(|row| row[x].clone()).collect();
        v
    }

    fn expand_down(&mut self, x: usize, y: usize) -> Vec<Cell> {
        let mut v = Vec::new();
        for cell in self.content[y][..x].iter_mut() {
            cell.owned = true;
            v.push(cell.clone());
        }
        // let v: Vec<Cell> = self.content[y][..x].to_vec();
        v
    }
}
