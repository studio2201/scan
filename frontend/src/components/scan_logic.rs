//! Scan game logic and grid state management.

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum Sector {
    Alpha,
    Beta,
    Gamma,
}

#[allow(dead_code)]
impl Sector {
    pub fn config(self) -> (usize, usize, usize) {
        match self {
            Self::Alpha => (9, 9, 10),
            Self::Beta => (16, 16, 40),
            Self::Gamma => (30, 16, 99),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Alpha => "Alpha",
            Self::Beta => "Beta",
            Self::Gamma => "Gamma",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub adjacent_mines: u8,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            adjacent_mines: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameStatus {
    NotStarted,
    Playing,
    Won,
    Lost,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BoardState {
    pub sector: Sector,
    pub rows: usize,
    pub cols: usize,
    pub mines: usize,
    pub grid: Vec<Vec<Cell>>,
    pub status: GameStatus,
    pub first_click: bool,
}

impl BoardState {
    pub fn new(sector: Sector) -> Self {
        let (rows, cols, mines) = sector.config();
        let grid = vec![vec![Cell::new(); cols]; rows];
        Self {
            sector,
            rows,
            cols,
            mines,
            grid,
            status: GameStatus::NotStarted,
            first_click: true,
        }
    }

    /// Place mines safely (guaranteeing first click and its neighbors are clear)
    pub fn generate_mines(&mut self, start_r: usize, start_c: usize) {
        let mut mine_positions = Vec::new();

        // Build all possible positions except start position and its neighbors
        for r in 0..self.rows {
            for c in 0..self.cols {
                let is_near_start = (r as isize - start_r as isize).abs() <= 1
                    && (c as isize - start_c as isize).abs() <= 1;
                if !is_near_start {
                    mine_positions.push((r, c));
                }
            }
        }

        // Shuffle using JS Math.random (Fisher-Yates)
        let len = mine_positions.len();
        for i in (1..len).rev() {
            let j = (js_sys::Math::random() * (i + 1) as f64).floor() as usize;
            mine_positions.swap(i, j);
        }

        let mines_to_place = self.mines.min(mine_positions.len());
        for &(r, c) in &mine_positions[..mines_to_place] {
            self.grid[r][c].is_mine = true;
        }

        // Calculate adjacency numbers
        for r in 0..self.rows {
            for c in 0..self.cols {
                if !self.grid[r][c].is_mine {
                    self.grid[r][c].adjacent_mines = self.count_adjacent_mines(r, c);
                }
            }
        }
        self.first_click = false;
        self.status = GameStatus::Playing;
    }

    fn count_adjacent_mines(&self, r: usize, c: usize) -> u8 {
        let mut count = 0;
        for dr in -1..=1 {
            for dc in -1..=1 {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0
                    && nr < self.rows as isize
                    && nc >= 0
                    && nc < self.cols as isize
                    && self.grid[nr as usize][nc as usize].is_mine
                {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn reveal_cell(&mut self, r: usize, c: usize) {
        if self.status == GameStatus::Won || self.status == GameStatus::Lost {
            return;
        }
        if self.grid[r][c].is_revealed || self.grid[r][c].is_flagged {
            return;
        }

        if self.first_click {
            self.generate_mines(r, c);
        }

        if self.grid[r][c].is_mine {
            self.grid[r][c].is_revealed = true;
            self.status = GameStatus::Lost;
            // Reveal all other mines
            for row in 0..self.rows {
                for col in 0..self.cols {
                    if self.grid[row][col].is_mine {
                        self.grid[row][col].is_revealed = true;
                    }
                }
            }
            return;
        }

        self.reveal_recursive(r, c);

        if self.check_win() {
            self.status = GameStatus::Won;
        }
    }

    fn reveal_recursive(&mut self, r: usize, c: usize) {
        if r >= self.rows
            || c >= self.cols
            || self.grid[r][c].is_revealed
            || self.grid[r][c].is_flagged
        {
            return;
        }

        self.grid[r][c].is_revealed = true;

        if self.grid[r][c].adjacent_mines == 0 {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                        self.reveal_recursive(nr as usize, nc as usize);
                    }
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, r: usize, c: usize) {
        if self.status != GameStatus::Playing && self.status != GameStatus::NotStarted {
            return;
        }
        if self.grid[r][c].is_revealed {
            return;
        }
        self.grid[r][c].is_flagged = !self.grid[r][c].is_flagged;
    }

    pub fn count_flagged(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|cell| cell.is_flagged).count())
            .sum()
    }

    fn check_win(&self) -> bool {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let cell = &self.grid[r][c];
                if !cell.is_mine && !cell.is_revealed {
                    return false;
                }
            }
        }
        true
    }
}
