use super::{Board, CellFlags};
use crate::math::Point;
use std::collections::HashSet;

#[test]
fn test_cell_flags() {
    let c1 = CellFlags::REVEALED | CellFlags::MINE;
    assert!(c1.contains(CellFlags::REVEALED));
    assert!(c1.contains(CellFlags::MINE));

    let c2 = CellFlags::REVEALED;
    assert!(c2.contains(CellFlags::REVEALED));
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
fn test_board_neighbors() {
    let b = Board::default();
    let neighbors: HashSet<Point<u32>> = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(0, 1),
        Point::new(2, 1),
        Point::new(0, 2),
        Point::new(1, 2),
        Point::new(2, 2),
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(neighbors, b.neighbors(1, 1).iter().cloned().collect());
}
