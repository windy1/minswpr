use super::math::{self, Point};

bitflags! {
    #[derive(Default)]
    pub struct CellFlags: u8 {
        const EXCAVATED = 0b00000001;
        const MINE = 0b00000010;
    }
}

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    num_mines: usize,
    mine_positions: Vec<Point>,
    cells: Vec<CellFlags>,
}

impl Board {
    const DEF_WIDTH: usize = 8;
    const DEF_HEIGHT: usize = 8;
    const DEF_MINE_FREQ: f64 = 0.078;

    pub fn new(width: usize, height: usize, mine_freq: f64) -> Result<Board, &'static str> {
        if mine_freq < 0.0 || mine_freq > 1.0 {
            return Err("mine_freq must be between 0.0 and 1.0");
        }

        let num_mines = (mine_freq * (width * height) as f64).round() as usize;
        let num_cells = width * height;
        let mine_indices = math::gen_rand_unique(num_mines, 0, num_cells);
        let cells = Self::make_cells(num_cells, &mine_indices);
        let mine_positions = mine_indices
            .iter()
            .map(|i| Self::point_from_index(width, height, *i))
            .collect();

        Ok(Board {
            width,
            height,
            num_mines,
            mine_positions,
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

    pub fn mine_positions(&self) -> &Vec<Point> {
        &self.mine_positions
    }

    pub fn index(&self, x: u32, y: u32) -> usize {
        y as usize * self.width + x as usize
    }

    fn point_from_index(width: usize, height: usize, idx: usize) -> Point {
        Point::new((idx % width) as i32, (idx / height) as i32)
    }

    fn make_cells(num_cells: usize, mine_indices: &Vec<usize>) -> Vec<CellFlags> {
        let mut cells = vec![CellFlags::default(); num_cells];
        for idx in mine_indices {
            cells[*idx].insert(CellFlags::MINE);
        }
        cells
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Board::DEF_WIDTH, Board::DEF_HEIGHT, Board::DEF_MINE_FREQ).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, CellFlags};
    use crate::math::Point;

    #[test]
    fn test_cell_flags() {
        let c1 = CellFlags::EXCAVATED | CellFlags::MINE;
        assert!(c1.contains(CellFlags::EXCAVATED));
        assert!(c1.contains(CellFlags::MINE));

        let c2 = CellFlags::EXCAVATED;
        assert!(c2.contains(CellFlags::EXCAVATED));
        assert!(!c2.contains(CellFlags::MINE));

        let c3: CellFlags = Default::default();
        assert!(c3.is_empty());
    }

    #[test]
    fn test_board_new() -> Result<(), &'static str> {
        let b = Board::default();
        let num_mines = b
            .cells()
            .iter()
            .filter(|c| c.contains(CellFlags::MINE))
            .count();
        assert_eq!(num_mines, b.num_mines());
        assert_eq!(b.width() * b.height(), b.cells().len());
        Ok(())
    }

    #[test]
    fn test_board_indexing() {
        let b = Board::default();
        assert_eq!(10, b.index(2, 1));
        assert_eq!(11, b.index(3, 1));
        assert_eq!(
            Point::new(2, 1),
            Board::point_from_index(b.width(), b.height(), 10)
        );
        assert_eq!(
            Point::new(3, 1),
            Board::point_from_index(b.width(), b.height(), 11)
        );
    }
}
