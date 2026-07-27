use colored::Colorize;
use core::panic;
use std::fmt;
type MatID = usize;

pub struct Grid {
    cells: Vec<Vec<MatID>>,
    mats: Vec<Mat>,
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
enum MatPostion {
    Singe(CellCoordinate),
    Double {
        first: CellCoordinate,
        second: CellCoordinate,
    },
}

#[derive(Clone, Debug)]
struct Mat {
    id: MatID,
    position: MatPostion,
    color: Color,
    owned: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    current_load: Vec<MatID>,
    loads: Vec<Vec<MatID>>,
    max_size: usize,
}

struct ID {
    id: usize,
}

impl ID {
    fn new() -> Self {
        Self { id: 0 }
    }

    fn next(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}

impl MatPostion {
    fn first(&self) -> &CellCoordinate {
        match self {
            MatPostion::Singe(postition) => postition,
            MatPostion::Double { first, .. } => first,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, mat_id) in row.iter().enumerate() {
                let mat = self.get_mat_with_id(*mat_id);
                let current = CellCoordinate::new(x, y);

                let cell_text = match &mat.position {
                    MatPostion::Singe(..) => {
                        format!("[{}] ", mat.color)
                    }

                    MatPostion::Double { first, second } if first.y == second.y => {
                        if &current == first {
                            format!("[{}--", mat.color)
                        } else if &current == second {
                            format!("-{}] ", mat.color)
                        } else {
                            return Err(fmt::Error);
                        }
                    }

                    MatPostion::Double { first, second } if first.x == second.x => {
                        if &current == first {
                            format!("┌{}┐ ", mat.color)
                        } else if &current == second {
                            format!("└{}┘ ", mat.color)
                        } else {
                            return Err(fmt::Error);
                        }
                    }

                    MatPostion::Double { .. } => return Err(fmt::Error),
                };

                mat.write_mat(f, &cell_text)?;
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

impl Mat {
    fn write_mat(&self, f: &mut fmt::Formatter<'_>, text: &str) -> fmt::Result {
        if !self.owned {
            return write!(f, "{}", text);
        }

        match self.color {
            Color::Blue => write!(f, "{}", text.truecolor(0, 55, 200)),
            Color::Yellow => write!(f, "{}", text.truecolor(255, 255, 0)),
            Color::None => write!(f, "{}", text),
        }
    }
}

impl fmt::Display for CellCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl CellCoordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

    fn add(&mut self, mut added_content: Vec<MatID>) {
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
    pub fn new(grid_input: Vec<Vec<&str>>, max_delivery_size: Option<usize>) -> Self {
        let mut id = ID::new();

        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        let mut grid: Vec<Vec<MatID>> = Vec::new();
        let mut mats: Vec<Mat> = Vec::new();

        for (y, row) in grid_input.iter().enumerate() {
            let mut r = Vec::new();
            for (x, item) in row.iter().enumerate() {
                if item.len() > 2 {
                    panic!("Faulty mat input! :c");
                }

                if item.len() == 2 {
                    let mat_id = id.next();
                    let color = Color::from(item.chars().nth(0).unwrap());

                    let position: MatPostion = match item.chars().nth(1).unwrap() {
                        'R' => {
                            if grid_input[y][x + 1].chars().nth(0) != item.chars().nth(0) {
                                panic!("Error: Multi Color Double Mat!")
                            }
                            MatPostion::Double {
                                first: CellCoordinate::new(x, y),
                                second: CellCoordinate::new(x + 1, y),
                            }
                        }
                        'D' => {
                            if grid_input[y + 1][x].chars().nth(0) != item.chars().nth(0) {
                                panic!("Error: Multi Color Double Mat!")
                            }
                            MatPostion::Double {
                                first: CellCoordinate::new(x, y),
                                second: CellCoordinate::new(x, y + 1),
                            }
                        }
                        'U' => {
                            let double_id = grid[y - 1][x];
                            r.push(double_id);
                            continue;
                        }
                        'L' => {
                            let double_id = r[x - 1];
                            r.push(double_id);
                            continue;
                        }
                        _ => {
                            panic!("Error: Faulty Mat Input! :c")
                        }
                    };

                    let mat = Mat {
                        id: mat_id,
                        position,
                        color,
                        owned: false,
                    };

                    mats.push(mat);
                    r.push(mat_id);
                } else {
                    let mat_id = id.next();
                    let color = Color::from(item.chars().nth(0).unwrap());
                    let position = MatPostion::Singe(CellCoordinate::new(x, y));

                    let mat = Mat {
                        id: mat_id,
                        color,
                        owned: false,
                        position,
                    };
                    mats.push(mat);
                    r.push(mat_id);
                }
            }
            grid.push(r);
        }

        let delivery = Delivery::new(max_delivery_size.unwrap_or(40));

        Self {
            cells: grid,
            height,
            width,
            mats,
            delivery,
        }
    }

    fn get_mat_with_id(&self, id: MatID) -> &Mat {
        for mat in &self.mats {
            if mat.id == id {
                return mat;
            }
        }
        panic!("Mat doesnt Exist!");
    }

    fn get_mut_mat_with_id(&mut self, id: MatID) -> &mut Mat {
        for mat in &mut self.mats {
            if mat.id == id {
                return mat;
            }
        }
        panic!("Mat doesnt Exist!");
    }

    pub fn build_diagonally(&mut self) {
        let (tatami_start_x, tatami_start_y) = {
            let start = self.find_first_tatami_mat().unwrap();
            (start.x, start.y)
        };

        let mut top_left_corner_area: Vec<MatID> = Vec::new();

        let mat_ids: Vec<MatID> = self.cells[..tatami_start_y]
            .iter()
            .flat_map(|row| row[..tatami_start_x].iter().copied())
            .collect();

        for mat_id in mat_ids {
            let mat = self.get_mut_mat_with_id(mat_id);
            mat.owned = true;
            top_left_corner_area.push(mat_id);
        }

        self.delivery.add(top_left_corner_area);

        println!("{}", self);

        let mut x: usize = tatami_start_x;
        let mut y: usize = tatami_start_y;

        let mut count = 0;

        while x < self.width || y < self.height {
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
                "Current Delivery Size: {}",
                self.delivery.current_load.len()
            );
            println!("Delivery Count: {}", self.delivery.loads.len());

            if count % 2 == 1 {
                let v = self.expand_center(x, y);
                self.delivery.add(v);
            }

            count += 1;
        }

        let v = self.expand_center(self.width, self.height);
        self.delivery.add(v);

        println!("{}", self);
    }

    fn find_first_tatami_mat(&self) -> Option<&CellCoordinate> {
        for mat_id in self.cells.iter().flatten() {
            let mat = self.get_mat_with_id(*mat_id);
            if mat.color == Color::Yellow {
                return Some(mat.position.first());
            }
        }
        None
    }

    fn expand_center(&mut self, x: usize, y: usize) -> Vec<MatID> {
        let mut v: Vec<MatID> = Vec::new();

        let mat_ids: Vec<MatID> = self.cells[..y]
            .iter()
            .flat_map(|row| row[..x].iter().copied())
            .collect();

        for mat_id in mat_ids {
            let mat = self.get_mut_mat_with_id(mat_id);

            if !mat.owned {
                mat.owned = true;
                v.push(mat_id);
            }
        }
        v
    }

    fn expand_right(&mut self, x: usize, y: usize) -> Vec<MatID> {
        let mut v = Vec::new();

        let mat_ids: Vec<MatID> = self.cells[..y].iter().map(|row| row[x]).collect();
        for mat_id in mat_ids {
            let mat = self.get_mut_mat_with_id(mat_id);
            mat.owned = true;
            v.push(mat_id);
        }

        v
    }

    fn expand_down(&mut self, x: usize, y: usize) -> Vec<MatID> {
        let mut v = Vec::new();
        let mat_ids = self.cells[y][..x].to_vec();

        for mat_id in mat_ids {
            let mat = self.get_mut_mat_with_id(mat_id);
            mat.owned = true;
            v.push(mat_id);
        }
        v
    }
}
