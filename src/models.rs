use colored::Colorize;
use std::fmt;

type MatID = usize;

pub struct Grid {
    cells: Vec<Vec<MatID>>,
    pub mats: Vec<Mat>,
    height: usize,
    pub width: usize,
    cache: Option<(Vec<Section>, usize)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Blue,
    Yellow,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Layout {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum MatPostion {
    Singe(CellCoordinate),
    Double {
        first: CellCoordinate,
        second: CellCoordinate,
        layout: Layout,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mat {
    pub position: MatPostion,
    pub color: Color,
    owned: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct CellCoordinate {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub first_postion: CellCoordinate,
    pub last_position: CellCoordinate,
    cells: Vec<MatID>,
    pub color: Color,
    is_double_mat: bool,
}

#[derive(Debug, Clone)]
pub enum GridError {
    InvalidMatInput {
        row: usize,
        col: usize,
        detail: String,
    },
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMatInput { row, col, detail } => {
                write!(f, "Invalid mat at ({col}, {row}): {detail} ")
            }
        }
    }
}

impl MatPostion {
    pub fn first(&self) -> &CellCoordinate {
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
                let mat = &self.mats[*mat_id];
                let current = CellCoordinate::new(x, y);

                let cell_text = match &mat.position {
                    MatPostion::Singe(..) => {
                        format!("[{}] ", mat.color)
                    }

                    MatPostion::Double { first, second, .. } if first.y == second.y => {
                        if &current == first {
                            format!("[{}--", mat.color)
                        } else if &current == second {
                            format!("-{}] ", mat.color)
                        } else {
                            return Err(fmt::Error);
                        }
                    }

                    MatPostion::Double { first, second, .. } if first.x == second.x => {
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

impl Grid {
    pub fn new(grid_input: Vec<Vec<String>>) -> Result<Self, GridError> {
        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        let mut grid: Vec<Vec<MatID>> = Vec::new();
        let mut mats: Vec<Mat> = Vec::new();

        for (y, row) in grid_input.iter().enumerate() {
            let mut r = Vec::new();
            for (x, item) in row.iter().enumerate() {
                if item.len() > 2 {
                    return Err(GridError::InvalidMatInput {
                        row: y,
                        col: x,
                        detail: format!("mat descriptor '{item}' too long (max 2 chars)"),
                    });
                }

                if item.len() == 2 {
                    let color = Color::from(item.chars().nth(0).unwrap());

                    let position: MatPostion = match item.chars().nth(1).unwrap() {
                        'R' => {
                            if x + 1 >= width {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} points out of bounds"
                                    ),
                                });
                            }

                            if grid_input[y][x + 1].chars().nth(0) != item.chars().nth(0) {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend colors"
                                    ),
                                });
                            } else if grid_input[y][x + 1].len() != 2 {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend neighbors (length)"
                                    ),
                                });
                            } else if grid_input[y][x + 1].chars().nth(1) != Some('L') {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend neighbors (symbol)"
                                    ),
                                });
                            }
                            MatPostion::Double {
                                first: CellCoordinate::new(x, y),
                                second: CellCoordinate::new(x + 1, y),
                                layout: Layout::Horizontal,
                            }
                        }
                        'D' => {
                            if y + 1 >= height {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} points out of bounds"
                                    ),
                                });
                            }

                            if grid_input[y + 1][x].chars().nth(0) != item.chars().nth(0) {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend colors"
                                    ),
                                });
                            } else if grid_input[y + 1][x].len() != 2 {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend neighbors (length)"
                                    ),
                                });
                            } else if grid_input[y + 1][x].chars().nth(1) != Some('U') {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend neighbors (symbol)"
                                    ),
                                });
                            }
                            MatPostion::Double {
                                first: CellCoordinate::new(x, y),
                                second: CellCoordinate::new(x, y + 1),
                                layout: Layout::Vertical,
                            }
                        }
                        'U' => {
                            if y == 0 {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} points out of bounds"
                                    ),
                                });
                            }

                            let double_id = grid[y - 1][x];

                            if grid_input[y - 1][x].len() != 2 {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend neighbors"
                                    ),
                                });
                            }

                            r.push(double_id);
                            continue;
                        }
                        'L' => {
                            if x == 0 {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} points out of bounds"
                                    ),
                                });
                            }

                            let double_id = r[x - 1];

                            if grid_input[y][x - 1].len() != 2 {
                                return Err(GridError::InvalidMatInput {
                                    row: y,
                                    col: x,
                                    detail: format!(
                                        "Double Mat descriptor {item} has inconsistend neighbors"
                                    ),
                                });
                            }

                            r.push(double_id);
                            continue;
                        }
                        _ => {
                            return Err(GridError::InvalidMatInput {
                                row: y,
                                col: x,
                                detail: format!("Invalid Mat descriptor: {item}"),
                            });
                        }
                    };

                    let mat_id = mats.len();

                    let mat = Mat {
                        position,
                        color,
                        owned: false,
                    };

                    mats.push(mat);
                    r.push(mat_id);
                } else {
                    let mat_id = mats.len();
                    let color = Color::from(item.chars().nth(0).unwrap());
                    let position = MatPostion::Singe(CellCoordinate::new(x, y));

                    let mat = Mat {
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

        Ok(Self {
            cells: grid,
            height,
            width,
            mats,
            cache: None,
        })
    }

    pub fn build_diagonally(&mut self, section_size: usize) -> Result<Vec<Section>, GridError> {
        // preprocess the double mats
        // section the mats
        // order the sections

        let mut sections: Vec<Section> = Vec::new();

        let mat_ids: Vec<MatID> = self.cells.iter().flatten().copied().collect();

        for mat_id in &mat_ids {
            let mat = &self.mats[*mat_id];
            if mat.owned {
                continue;
            }
            if matches!(mat.position, MatPostion::Double { .. }) {
                let section = self.set_2x1_mat(*mat_id);
                sections.push(section);
            }
        }

        for mat_id in &mat_ids {
            let seed_mat = &mut self.mats[*mat_id];

            if seed_mat.owned || seed_mat.color == Color::None {
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
            section_ids.push(*mat_id);

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

                    if section_size < cells_to_add.len() + section_ids.len() {
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
                        let mat = &mut self.mats[*mat_id];
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

                    if section_size < cells_to_add.len() + section_ids.len() {
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
                        let mat = &mut self.mats[*mat_id];
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

            let last_mat = &self.mats[mat_id];
            let s = Section {
                color: seed_color,
                cells: section_ids,
                first_postion: seed_first_position,
                last_position: *last_mat.position.first(),
                is_double_mat: false,
            };
            sections.push(s);
        }
        sections.sort_unstable_by_key(|item| {
            (
                item.first_postion.x + item.first_postion.y,
                item.first_postion.y,
            )
        });
        println!("{}", self);
        self.reset_grid();
        self.cache = Some((sections.clone(), section_size));

        Ok(sections)
    }

    fn set_2x1_mat(&mut self, mat_id: MatID) -> Section {
        let mat = &mut self.mats[mat_id];
        let s = match mat.position {
            MatPostion::Double { first, second, .. } => Section {
                first_postion: first,
                last_position: second,
                cells: vec![mat_id],
                color: mat.color,
                is_double_mat: true,
            },
            _ => panic!("Called set_2x1_mat on 1x1 mat"),
        };

        mat.owned = true;
        s
    }

    fn is_cell_available(&self, color: &Color, mat_id: MatID) -> bool {
        let mat = &self.mats[mat_id];
        &mat.color == color && !mat.owned
    }

    pub fn generate_deliveries(
        &mut self,
        delivery_size: usize,
        section_size: usize,
    ) -> Result<Vec<Vec<(i32, Color)>>, GridError> {
        let sections = match &self.cache {
            None => self.build_diagonally(section_size)?,
            Some(sections) => {
                if sections.1 == section_size {
                    sections.0.clone()
                } else {
                    self.build_diagonally(section_size)?
                }
            }
        };

        let mut deliveries: Vec<Vec<Color>> = Vec::new();
        let mut current_delivery: Vec<Color> = Vec::new();

        for section in sections {
            if section.is_double_mat {
                continue;
            }

            for _ in section.cells {
                if current_delivery.len() == delivery_size {
                    deliveries.push(current_delivery.clone());
                    current_delivery.clear();
                }

                current_delivery.push(section.color);
            }
        }
        deliveries.push(current_delivery);

        let mut compressed_deliveries: Vec<Vec<(i32, Color)>> = Vec::new();

        for delivery in &deliveries {
            let mut compressed_delivery = Vec::new();
            let mut counter = 0;
            let mut color: &Color = &delivery[0];

            for m in delivery {
                if m == color {
                    counter += 1;
                } else {
                    compressed_delivery.push((counter, *color));
                    counter = 1;
                    color = m;
                }
            }
            compressed_delivery.push((counter, *color));
            compressed_deliveries.push(compressed_delivery);
        }

        Ok(compressed_deliveries)
    }

    fn reset_grid(&mut self) {
        for mat in &mut self.mats {
            mat.owned = false;
        }
    }
}
