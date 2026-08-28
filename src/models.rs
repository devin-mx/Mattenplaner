use colored::Colorize;
use std::{fmt, mem::take};

type MatID = usize;

pub struct Grid {
    cells: Vec<Vec<MatID>>,
    mats: Vec<Mat>,
    delivery: Delivery,
    build_order: Vec<MatID>,
    print_intervals: usize,
    section_size: usize,
    height: usize,
    width: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Blue,
    Yellow,
    None,
}

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct CellCoordinate {
    y: usize,
    x: usize,
}

#[derive(Debug)]
pub struct Section {
    pub first_postion: CellCoordinate,
    pub last_position: CellCoordinate,
    cells: Vec<MatID>,
    pub color: Color,
}

struct Delivery {
    current_load: Vec<Color>,
    loads: Vec<Vec<Color>>,
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

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "Color: {}; from {}, to {}",
            self.color, self.first_postion, self.last_position
        );
        write!(f, "{}", s)
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
        write!(f, "({}, {})", self.x + 1, self.y + 1)
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

    fn add(&mut self, mut added_content: Vec<Color>) {
        while added_content.len() + self.current_load.len() >= self.max_size {
            let remaining_space = self.max_size - self.current_load.len();
            self.current_load
                .extend(added_content.drain(..remaining_space));

            self.loads.push(take(&mut self.current_load));
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
                            panic!("Error: Faulty Mat Input!")
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
            section_size: 2,
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

    pub fn build_diagonally(&mut self) -> Vec<Section> {
        // preprocess the double mats
        // section the mats
        // order the sections

        let mut sections: Vec<Section> = Vec::new();

        let mat_ids: Vec<MatID> = self.cells.iter().flatten().copied().collect();

        for mat_id in &mat_ids {
            let mat = self.get_mat_with_id(*mat_id);
            if mat.owned {
                continue;
            }
            if matches!(mat.position, MatPostion::Double { .. }) {
                let section = self.set_2x1_mat(*mat_id);
                sections.push(section);
            }
        }

        for mat_id in &mat_ids {
            let seed_mat = self.get_mut_mat_with_id(*mat_id);
            println!("{}", seed_mat.position.first());

            if seed_mat.owned {
                continue;
            }

            let seed_first_position = *seed_mat.position.first();
            let seed_color = seed_mat.color;

            let seed_start_x = seed_mat.position.first().x;
            let seed_start_y = seed_mat.position.first().y;

            let mut expand_right = true;
            let mut expand_down = true;

            let mut x = seed_start_x + 1;
            let mut y = seed_start_y + 1;

            let mut section_ids: Vec<MatID> = Vec::new();

            seed_mat.owned = true;
            section_ids.push(seed_mat.id);

            'outer: while expand_right || expand_down {
                if expand_right {
                    if x >= self.width {
                        expand_right = false;
                        continue;
                    }

                    let cells_to_add: Vec<MatID> = self.cells[seed_start_y..y]
                        .iter()
                        .map(|row| row[x])
                        .collect();

                    if self.section_size < cells_to_add.len() + section_ids.len() {
                        expand_right = false;
                        continue;
                    }

                    for mat_id in &cells_to_add {
                        if !self.is_cell_available(&seed_color, *mat_id) {
                            expand_right = false;
                            continue 'outer;
                        }
                    }

                    for mat_id in &cells_to_add {
                        let mat = self.get_mut_mat_with_id(*mat_id);
                        mat.owned = true;
                    }

                    section_ids.extend(cells_to_add);

                    x += 1;
                }

                if expand_down {
                    if y >= self.height {
                        expand_down = false;
                        continue;
                    }

                    let cells_to_add: Vec<MatID> = self.cells[y][seed_start_x..x].to_vec();

                    if self.section_size < cells_to_add.len() + section_ids.len() {
                        expand_down = false;
                        continue;
                    }

                    for mat_id in &cells_to_add {
                        if !self.is_cell_available(&seed_color, *mat_id) {
                            expand_down = false;
                            continue 'outer;
                        }
                    }

                    for mat_id in &cells_to_add {
                        let mat = self.get_mut_mat_with_id(*mat_id);
                        mat.owned = true;
                    }

                    section_ids.extend(cells_to_add);

                    y += 1;
                }
            }

            let last_id = section_ids.last();
            let mat_id = match last_id {
                None => continue,
                Some(id) => *id,
            };

            let last_mat = self.get_mat_with_id(mat_id);
            let s = Section {
                color: seed_color,
                cells: section_ids,
                first_postion: seed_first_position,
                last_position: *last_mat.position.first(),
            };
            sections.push(s);
            println!("{}", self);
        }
        sections.sort_unstable_by_key(|item| {
            (
                item.first_postion.x + item.first_postion.y,
                item.first_postion.y,
            )
        });
        self.reset_grid();
        sections
    }

    fn set_2x1_mat(&mut self, mat_id: MatID) -> Section {
        let mat = self.get_mut_mat_with_id(mat_id);
        let s = match mat.position {
            MatPostion::Double { first, second } => Section {
                first_postion: first,
                last_position: second,
                cells: vec![mat.id],
                color: mat.color,
            },
            _ => panic!("Smth went wrong"),
        };

        mat.owned = true;
        s
    }

    fn is_cell_available(&self, color: &Color, mat_id: MatID) -> bool {
        let mat = self.get_mat_with_id(mat_id);
        &mat.color == color && !mat.owned
    }

    fn add_to_build(&mut self, build_mats: Vec<MatID>) {
        for mat_id in &build_mats {
            if self.build_order.contains(mat_id) {
                continue;
            }

            let mat = self.get_mut_mat_with_id(*mat_id);
            mat.owned = true;
            self.build_order.push(*mat_id);

            if self.build_order.len().is_multiple_of(self.print_intervals) {
                println!("{}", self);
            }
        }

        let delivery_mats: Vec<Color> = build_mats
            .into_iter()
            .map(|mat_id| {
                let mat = self.get_mat_with_id(mat_id);
                mat.color
            })
            .collect();

        self.delivery.add(delivery_mats);
    }

    pub fn set_print_intervals(&mut self, intervals: usize) {
        self.print_intervals = intervals;
    }

    fn reset_grid(&mut self) {
        let mat_ids: Vec<MatID> = self.cells.iter().flatten().copied().collect();
        for mat_id in mat_ids {
            self.get_mut_mat_with_id(mat_id).owned = false;
        }
    }
}
