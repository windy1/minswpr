use crate::math::{self, Point};
use itertools::Itertools;

bitflags! {
    #[derive(Default)]
    pub struct CellFlags: u8 {
        const REVEALED = 0b00000001;
        const MINE = 0b00000010;
        const FLAG = 0b00000100;
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    num_mines: usize,
    cells: Vec<CellFlags>,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_freq: f64) -> Result<Self, &'static str> {
        if mine_freq < 0.0 || mine_freq > 1.0 {
            return Err("mine_freq must be between 0.0 and 1.0");
        }

        let num_mines = (mine_freq * (width * height) as f64).round() as usize;
        let num_cells = width * height;
        let mine_indices = math::gen_rand_unique(num_mines, 0, num_cells);
        let cells = Self::make_cells(num_cells, &mine_indices);

        Ok(Self {
            width,
            height,
            num_mines,
            cells,
        })
    }

    pub fn empty() -> Self {
        Self::new(0, 0, 0.0).unwrap()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn num_mines(&self) -> usize {
        self.num_mines
    }

    pub fn cells(&self) -> &Vec<CellFlags> {
        &self.cells
    }

    pub fn cell(&self, x: u32, y: u32) -> &CellFlags {
        &self.cells[Self::index(x, y, self.width)]
    }

    pub fn cell_mut(&mut self, x: u32, y: u32) -> &mut CellFlags {
        &mut self.cells[Self::index(x, y, self.width)]
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&CellFlags> {
        self.cells.get(Self::index(x, y, self.width))
    }

    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut CellFlags> {
        self.cells.get_mut(Self::index(x, y, self.width))
    }

    pub fn toggle_flag(&mut self, x: u32, y: u32) {
        match self.get_cell_mut(x, y) {
            Some(c) => {
                if !c.contains(CellFlags::REVEALED) {
                    c.toggle(CellFlags::FLAG);
                }
            }
            None => {}
        }
    }

    pub fn reveal_from(&mut self, x: u32, y: u32) -> u32 {
        let mut count = 0;
        self._reveal_from(x, y, &mut count);
        count
    }

    fn _reveal_from(&mut self, x: u32, y: u32, count: &mut u32) {
        // make sure the cell hasn't been previously revealed
        // or..
        // make sure the cell isn't flagged
        let cell = self
            .get_cell_mut(x, y)
            .filter(|c| !c.contains(CellFlags::REVEALED))
            .filter(|c| !c.contains(CellFlags::FLAG));

        let cell = match cell {
            Some(c) => c,
            None => return,
        };

        // reveal the current cell
        cell.insert(CellFlags::REVEALED);
        *count += 1;

        // if the revealed cell was a mine, stop revealing cells
        // or...
        // if the revealed cell is touching a mine, stop revealing cells
        if cell.contains(CellFlags::MINE) || self.count_adjacent_mines(x, y) > 0 {
            return;
        }

        // reveal all adjacent cells that are not a mine
        self.filter_neighbors(x, y, |c| {
            !c.contains(CellFlags::MINE) && !c.contains(CellFlags::REVEALED)
        })
        .iter()
        .for_each(|p| self._reveal_from(p.x, p.y, count));
    }

    pub fn reveal_unflagged(&mut self, x: u32, y: u32) -> Vec<Point<u32>> {
        let num_mines = self.count_adjacent_mines(x, y);
        let num_flags = self.count_adjacent_flags(x, y);

        // this function only accepts cells that have at least 1 adjacent mine
        // and...
        // the player must have flagged an amount of cells equal to the amount of adjacent mines
        if num_mines <= 0 || num_flags != num_mines {
            return Vec::new();
        }

        // only accept revealed cells
        let cell = self
            .get_cell(x, y)
            .filter(|c| c.contains(CellFlags::REVEALED));
        if let None = cell {
            return Vec::new();
        }

        // reveal the cells neighbors that are not revealed and not flagged
        let neighbors = self.filter_neighbors(x, y, |c| {
            !c.contains(CellFlags::REVEALED) && !c.contains(CellFlags::FLAG)
        });

        for neighbor in &neighbors {
            self.cell_mut(neighbor.x, neighbor.y)
                .insert(CellFlags::REVEALED);
        }

        neighbors
    }

    pub fn neighbors(&self, x: u32, y: u32) -> Vec<Point<u32>> {
        let x = x as i32;
        let y = y as i32;
        (x - 1..x + 2)
            .cartesian_product(y - 1..y + 2)
            .filter(|(nx, ny)| *nx >= 0 && *ny >= 0)
            .filter(|(nx, ny)| *nx < self.width as i32 && *ny < self.height as i32)
            .filter(|(nx, ny)| !(*nx == x && *ny == y))
            .map(|(nx, ny)| (nx as u32, ny as u32))
            .filter(|(nx, ny)| self.get_cell(*nx, *ny).is_some())
            .map(|(nx, ny)| Point::new(nx, ny))
            .collect()
    }

    pub fn count_adjacent_mines(&self, x: u32, y: u32) -> usize {
        self.filter_neighbors(x, y, |c| c.contains(CellFlags::MINE))
            .len()
    }

    pub fn count_adjacent_flags(&self, x: u32, y: u32) -> usize {
        self.filter_neighbors(x, y, |c| c.contains(CellFlags::FLAG))
            .len()
    }

    fn filter_neighbors<F>(&self, x: u32, y: u32, f: F) -> Vec<Point<u32>>
    where
        F: Fn(&CellFlags) -> bool,
    {
        let neighbors = self.neighbors(x, y);
        neighbors
            .iter()
            .filter(|p| f(self.cell(p.x, p.y)))
            .cloned()
            .collect()
    }

    fn index(x: u32, y: u32, w: usize) -> usize {
        y as usize * w + x as usize
    }

    fn make_cells(num_cells: usize, mine_indices: &Vec<usize>) -> Vec<CellFlags> {
        let mut cells = vec![CellFlags::default(); num_cells];
        for idx in mine_indices {
            cells[*idx].insert(CellFlags::MINE);
        }
        cells
    }
}
