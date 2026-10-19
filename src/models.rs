use anyhow::Result;
use colored::Colorize;
use std::{collections::HashSet, fmt, hash::Hash};

type MatID = usize;

pub struct FloorPlan {
    cells: Vec<Vec<MatID>>,
    pub mats: Vec<MatTile>,
    pub height: usize,
    width: usize,
    cache: Option<(Vec<Section>, usize)>,
}

#[allow(dead_code)]
pub struct Tatami {
    cells: Vec<Vec<MatID>>,
    pub mats: Vec<MatTile>,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Color {
    Blue,
    Yellow,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Layout {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum MatPosition {
    Single(CellCoordinate),
    Double {
        first: CellCoordinate,
        second: CellCoordinate,
        layout: Layout,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatTile {
    pub position: MatPosition,
    pub color: Color,
    pub owned: bool,
    pub id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct CellCoordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub first_position: CellCoordinate,
    pub last_position: CellCoordinate,
    cells: Vec<MatID>,
    pub color: Color,
    #[allow(dead_code)]
    is_double_mat: bool,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum GridError {
    #[error("Invalid mat at ({col}, {row}): {detail}")]
    InvalidMatInput {
        row: usize,
        col: usize,
        detail: String,
    },
}

impl MatPosition {
    pub fn first(&self) -> &CellCoordinate {
        match self {
            MatPosition::Single(postition) => postition,
            MatPosition::Double { first, .. } => first,
        }
    }
}

impl fmt::Display for FloorPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, mat_id) in row.iter().enumerate() {
                let mat = &self.mats[*mat_id];
                let current = CellCoordinate::new(x, y);

                let cell_text = match &mat.position {
                    MatPosition::Single(..) => {
                        format!("[{}] ", mat.color)
                    }

                    MatPosition::Double { first, second, .. } if first.y == second.y => {
                        if &current == first {
                            format!("[{}--", mat.color)
                        } else if &current == second {
                            format!("-{}] ", mat.color)
                        } else {
                            return Err(fmt::Error);
                        }
                    }

                    MatPosition::Double { first, second, .. } if first.x == second.x => {
                        if &current == first {
                            format!("┌{}┐ ", mat.color)
                        } else if &current == second {
                            format!("└{}┘ ", mat.color)
                        } else {
                            return Err(fmt::Error);
                        }
                    }

                    MatPosition::Double { .. } => return Err(fmt::Error),
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
            self.color, self.first_position, self.last_position
        );
        write!(f, "{s}")
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

impl MatTile {
    fn write_mat(&self, f: &mut fmt::Formatter<'_>, text: &str) -> fmt::Result {
        /*
        if !self.owned {
            return write!(f, "{text}");
        }
        */

        match self.color {
            Color::Blue => write!(f, "{}", text.truecolor(0, 55, 200)),
            Color::Yellow => write!(f, "{}", text.truecolor(255, 255, 0)),
            Color::None => write!(f, "{text}"),
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

impl FloorPlan {
    pub fn new(grid_input: &[Vec<String>]) -> Result<Self, GridError> {
        let height: usize = grid_input.len();
        let width: usize = if height > 0 { grid_input[0].len() } else { 0 };

        let mut grid: Vec<Vec<MatID>> = Vec::new();
        let mut mats: Vec<MatTile> = Vec::new();

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

                    let position: MatPosition = match item.chars().nth(1).unwrap() {
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
                            MatPosition::Double {
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
                            MatPosition::Double {
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

                    let mat = MatTile {
                        position,
                        color,
                        owned: false,
                        id: mat_id,
                    };

                    mats.push(mat);
                    r.push(mat_id);
                } else {
                    let mat_id = mats.len();
                    let color = Color::from(item.chars().nth(0).unwrap());
                    let position = MatPosition::Single(CellCoordinate::new(x, y));

                    let mat = MatTile {
                        color,
                        owned: false,
                        position,
                        id: mat_id,
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

    pub fn sectioning(&mut self, section_size: usize) -> Vec<Section> {
        // preprocess the double mats
        // section the mats
        // order the sections

        let mut sections: Vec<Section> = Vec::new();

        let mats = &self.mats;

        let mut visited: HashSet<MatID> = HashSet::new();

        sections.extend(preprocess_double_mats(&mut visited, mats));

        for seed_mat in mats {
            if seed_mat.color == Color::None || !visited.insert(seed_mat.id) {
                continue;
            }

            let seed_first_position = *seed_mat.position.first();
            let seed_color = seed_mat.color;

            let start_x = seed_mat.position.first().x;
            let start_y = seed_mat.position.first().y;

            let mut expand_right = true;
            let mut expand_down = true;

            let mut x = start_x + 1;
            let mut y = start_y + 1;

            let mut section_ids: Vec<MatID> = Vec::new();

            section_ids.push(seed_mat.id);

            'outer: while expand_right || expand_down {
                if expand_right {
                    if x >= self.width {
                        expand_right = false;
                        continue;
                    }

                    let cells_to_add: Vec<MatID> =
                        self.cells[start_y..y].iter().map(|row| row[x]).collect();

                    if cells_to_add.len() + section_ids.len() > section_size {
                        expand_right = false;
                        continue;
                    }

                    for mat_id in &cells_to_add {
                        if !self.is_cell_available(seed_color, *mat_id) {
                            expand_right = false;
                            continue 'outer;
                        }
                    }
                    visited.extend(&cells_to_add);
                    section_ids.extend(cells_to_add);

                    x += 1;
                }

                if expand_down {
                    if y >= self.height {
                        expand_down = false;
                        continue;
                    }

                    let cells_to_add: Vec<MatID> = self.cells[y][start_x..x].to_vec();

                    if cells_to_add.len() + section_ids.len() > section_size {
                        expand_down = false;
                        continue;
                    }

                    for mat_id in &cells_to_add {
                        if !self.is_cell_available(seed_color, *mat_id) {
                            expand_down = false;
                            continue 'outer;
                        }
                    }
                    visited.extend(&cells_to_add);
                    section_ids.extend(cells_to_add);
                    y += 1;
                }
            }

            let last_id = section_ids.last();
            let Some(mat_id) = last_id else {
                continue;
            };

            let last_mat = &self.mats[*mat_id];
            let s = Section {
                color: seed_color,
                cells: section_ids,
                first_position: seed_first_position,
                last_position: *last_mat.position.first(),
                is_double_mat: false,
            };
            sections.push(s);
        }

        self.cache = Some((sections.clone(), section_size));
        sections
    }

    pub fn generate_pages(
        &mut self,
        delivery_size: usize,
        section_size: usize,
    ) -> Vec<Vec<MatTile>> {
        let deliveries = self.separate_by_deliveries(delivery_size, section_size);

        let mut out = Vec::new();
        for delivery in deliveries {
            let mut groups = Vec::new();
            for group in self.group_adjacent(&delivery) {
                groups.push(self.page_builder(&group));
            }
            // let _comp = self.compress_delivery(&delivery);
            out.extend(groups);
        }
        out
    }

    fn page_builder(&mut self, mats: &[MatID]) -> Vec<MatTile> {
        for &id in mats {
            self.mats[id].owned = true;
        }
        let out = self.mats.clone();
        self.reset_grid();
        out
    }

    fn is_cell_available(&self, color: Color, mat_id: MatID) -> bool {
        let mat = &self.mats[mat_id];
        mat.color == color && !mat.owned
    }

    pub fn generate_deliveries(
        &mut self,
        delivery_size: usize,
        section_size: usize,
    ) -> Vec<Vec<MatID>> {
        let mut sections = self.check_cache_or_section(section_size);
        sort_sections_diagonally(&mut sections);

        let mut deliveries: Vec<Vec<MatID>> = Vec::new();
        let mut current_delivery = Vec::new();

        for section in sections {
            if section.is_double_mat {
                continue;
            }

            for mat_id in section.cells {
                if current_delivery.len() == delivery_size {
                    deliveries.push(std::mem::take(&mut current_delivery));
                }
                current_delivery.push(mat_id);
            }
        }
        deliveries.push(current_delivery);
        deliveries
    }

    fn separate_by_deliveries(
        &mut self,
        delivery_size: usize,
        section_size: usize,
    ) -> Vec<Vec<MatID>> {
        let mut sections = self.check_cache_or_section(section_size);
        sort_sections_diagonally(&mut sections);

        let mut deliveries: Vec<Vec<MatID>> = Vec::new();
        let mut current_delivery = Vec::new();

        let mut current_size = 0;
        for section in sections {
            for mat_id in section.cells {
                if current_size == delivery_size {
                    deliveries.push(std::mem::take(&mut current_delivery));
                    current_size = 0;
                }
                current_delivery.push(mat_id);

                if section.is_double_mat {
                    continue;
                }
                current_size += 1;
            }
        }
        deliveries.push(current_delivery);
        deliveries
    }

    fn group_adjacent(&self, delivery: &[MatID]) -> Vec<Vec<MatID>> {
        let in_delivery: HashSet<MatID> = delivery.iter().copied().collect();
        let mut visited: HashSet<MatID> = HashSet::new();
        let mut groups = Vec::new();

        for &start in delivery {
            if !visited.insert(start) {
                continue;
            }

            let color = self.mats[start].color;
            let mut stack = vec![start];
            let mut group = Vec::new();

            while let Some(id) = stack.pop() {
                group.push(id);
                let positions = match self.mats[id].position {
                    MatPosition::Single(pos) => vec![pos],
                    MatPosition::Double { first, second, .. } => vec![first, second],
                };

                for pos in positions {
                    for n in self.neighbor_mats(pos.x, pos.y) {
                        if self.mats[n].color == color
                            && in_delivery.contains(&n)
                            && visited.insert(n)
                        {
                            stack.push(n);
                        }
                    }
                }
            }
            groups.push(group);
        }
        groups
    }

    fn neighbor_mats(&self, x: usize, y: usize) -> Vec<MatID> {
        let mut out = Vec::with_capacity(4);

        if let Some(next_y) = y.checked_sub(1) {
            out.push(self.cells[next_y][x]);
        }
        if y + 1 < self.height {
            out.push(self.cells[y + 1][x]);
        }
        if let Some(next_x) = x.checked_sub(1) {
            out.push(self.cells[y][next_x]);
        }
        if x + 1 < self.width {
            out.push(self.cells[y][x + 1]);
        }

        out
    }

    #[allow(dead_code)]
    fn compress_delivery(&self, delivery: &[MatID]) -> Vec<(u8, Color)> {
        let mut out = Vec::new();
        let mut color: Option<Color> = None;
        let mut count = 0;
        for id in delivery {
            let mat = &self.mats[*id];

            if color.is_none() {
                color = Some(mat.color);
            }

            if color.unwrap() != mat.color {
                out.push((count, color.unwrap()));
                color = Some(mat.color);
                count = 0;
            }
            count += 1;
        }
        out.push((count, color.unwrap()));
        out
    }

    fn reset_grid(&mut self) {
        for mat in &mut self.mats {
            mat.owned = false;
        }
    }

    fn check_cache_or_section(&mut self, section_size: usize) -> Vec<Section> {
        match &self.cache {
            None => self.sectioning(section_size),
            Some(sections) => {
                if sections.1 == section_size {
                    sections.0.clone()
                } else {
                    self.sectioning(section_size)
                }
            }
        }
    }

    pub fn print_to_terminal(&self) {
        println!("{self}");
    }
}

fn preprocess_double_mats(visited: &mut HashSet<MatID>, mats: &[MatTile]) -> Vec<Section> {
    let mut out = Vec::new();
    for mat in mats {
        if matches!(mat.position, MatPosition::Double { .. }) && visited.insert(mat.id) {
            let section = set_2x1_mat(mat);
            out.push(section);
        }
    }
    out
}

fn set_2x1_mat(mat: &MatTile) -> Section {
    let MatPosition::Double { first, second, .. } = mat.position else {
        unreachable!("Called set_2x1_mat on 1x1 mat");
    };

    Section {
        first_position: first,
        last_position: second,
        cells: vec![mat.id],
        color: mat.color,
        is_double_mat: true,
    }
}

fn sort_sections_diagonally(sections: &mut [Section]) {
    sections.sort_unstable_by_key(|item| {
        (
            item.first_position.x + item.first_position.y,
            item.first_position.y,
        )
    });
}
