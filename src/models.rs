use colored::Colorize;
use std::fmt;

type MatID = usize;

pub struct Grid {
    cells: Vec<Vec<MatID>>,
    mats: Vec<Mat>,
    #[allow(dead_code)]
    delivery: Delivery,
    build_order: Vec<MatID>,
    print_intervals: usize,
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

#[allow(dead_code)]
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
    pub fn new(grid_input: Vec<Vec<&str>>, max_delivery_size: usize) -> Self {
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
                            if x + 1 >= width {
                                panic!("Error: Inconsistend Mat Input!");
                            }

                            if grid_input[y][x + 1].chars().nth(0) != item.chars().nth(0) {
                                panic!("Error: Multi Color Double Mat!");
                            } else if grid_input[y][x + 1].len() != 2 {
                                panic!("Error: Inconsistend Mat sizes!");
                            } else if grid_input[y][x + 1].chars().nth(1) != Some('L') {
                                panic!("Error: Inconsistend Mat Input!");
                            }
                            MatPostion::Double {
                                first: CellCoordinate::new(x, y),
                                second: CellCoordinate::new(x + 1, y),
                            }
                        }
                        'D' => {
                            if y + 1 >= height {
                                panic!("Error: Inconsistend Mat Input!");
                            }

                            if grid_input[y + 1][x].chars().nth(0) != item.chars().nth(0) {
                                panic!("Error: Multi Color Double Mat!");
                            } else if grid_input[y + 1][x].len() != 2 {
                                panic!("Error: Inconsistend Mat sizes!");
                            } else if grid_input[y + 1][x].chars().nth(1) != Some('U') {
                                panic!("Error: Inconsistend Mat Input!");
                            }
                            MatPostion::Double {
                                first: CellCoordinate::new(x, y),
                                second: CellCoordinate::new(x, y + 1),
                            }
                        }
                        'U' => {
                            if y == 0 {
                                panic!("Error: Inconsistend Mat sizes!");
                            }

                            let double_id = grid[y - 1][x];

                            if grid_input[y - 1][x].len() != 2 {
                                panic!("Error: Inconsistend Mat sizes!");
                            }

                            r.push(double_id);
                            continue;
                        }
                        'L' => {
                            if x == 0 {
                                panic!("Error: Inconsistend Mat sizes!");
                            }

                            let double_id = r[x - 1];

                            if grid_input[y][x - 1].len() != 2 {
                                panic!("Error: Inconsistend Mat sizes!");
                            }

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

        let delivery = Delivery::new(max_delivery_size);

        Self {
            cells: grid,
            height,
            width,
            mats,
            delivery,
            build_order: Vec::new(),
            print_intervals: 2,
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

        let top_left_corner_area: Vec<MatID> = self.cells[..tatami_start_y]
            .iter()
            .flat_map(|row| row[..tatami_start_x].iter().copied())
            .collect();

        self.add_to_build(top_left_corner_area);

        let mut x: usize = tatami_start_x;
        let mut y: usize = tatami_start_y;

        let mut count = 0;

        while x < self.width || y < self.height {
            if x < self.width {
                let add_cells = self.expand_right(x, tatami_start_y);
                self.add_to_build(add_cells);
                x += 1;
            }

            if y < self.height {
                let add_cells = self.expand_down(tatami_start_x, y);
                self.add_to_build(add_cells);
                y += 1;
            }

            if count % 2 == 1 {
                let v = self.expand_center(x, y);
                self.add_to_build(v);
            }

            count += 1;
        }

        let v = self.expand_center(self.width, self.height);
        self.add_to_build(v);

        print!("{}", self);
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

    fn expand_center(&self, x: usize, y: usize) -> Vec<MatID> {
        let mut v: Vec<MatID> = Vec::new();

        let mat_ids: Vec<MatID> = self.cells[..y]
            .iter()
            .flat_map(|row| row[..x].iter().copied())
            .collect();

        for mat_id in mat_ids {
            let mat = self.get_mat_with_id(mat_id);

            if !mat.owned {
                v.push(mat_id);
            }
        }
        v
    }

    fn expand_right(&self, x: usize, y: usize) -> Vec<MatID> {
        let mat_ids: Vec<MatID> = self.cells[..y].iter().map(|row| row[x]).collect();
        mat_ids
    }

    fn expand_down(&mut self, x: usize, y: usize) -> Vec<MatID> {
        self.cells[y][..x].to_vec()
    }

    fn add_to_build(&mut self, build_mats: Vec<MatID>) {
        for mat_id in build_mats {
            if self.build_order.contains(&mat_id) {
                continue;
            }

            let mat = self.get_mut_mat_with_id(mat_id);
            mat.owned = true;
            self.build_order.push(mat_id);

            if self.build_order.len().is_multiple_of(self.print_intervals) {
                println!("{}", self);
            }
        }
    }

    pub fn set_print_intervals(&mut self, intervals: usize) {
        self.print_intervals = intervals;
    }
}
