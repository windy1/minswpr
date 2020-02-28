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

fn make_board() -> Board {
    Board::new(8, 8, 0.078).unwrap()
}

#[test]
fn test_board_new() -> Result<(), &'static str> {
    let b = make_board();
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
    let b = make_board();
    let neighbors: HashSet<Point<u32>> = vec![
        point!(0, 0),
        point!(1, 0),
        point!(2, 0),
        point!(0, 1),
        point!(2, 1),
        point!(0, 2),
        point!(1, 2),
        point!(2, 2),
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(neighbors, b.neighbors(1, 1).iter().cloned().collect());
}

#[test]
fn test_reveal_from() {
    let mut b = Board::new(9, 9, 0.0).unwrap();
    assert_eq!(81, b.reveal_from(0, 0));
}

#[test]
fn test_reveal_unflagged() {
    let mut b = Board::new(9, 9, 0.0).unwrap();
    assert_eq!(0, b.reveal_unflagged(0, 0));

    b.cell_mut(1, 0).insert(CellFlags::MINE);
    assert_eq!(0, b.reveal_unflagged(0, 0));

    b.cell_mut(1, 0).insert(CellFlags::FLAG);
    assert_eq!(0, b.reveal_unflagged(0, 0));

    b.cell_mut(0, 0).insert(CellFlags::REVEALED);
    assert_eq!(2, b.reveal_unflagged(0, 0));

    b = Board::new(9, 9, 0.0).unwrap();
    b.cell_mut(0, 0).insert(CellFlags::MINE | CellFlags::FLAG);
    b.cell_mut(1, 0).insert(CellFlags::MINE | CellFlags::FLAG);
    b.cell_mut(1, 1).insert(CellFlags::REVEALED);

    let mut b2 = b.clone();

    assert_eq!(6, b.reveal_unflagged(1, 1));

    b2.cell_mut(0, 0).remove(CellFlags::FLAG);
    assert_eq!(0, b2.reveal_unflagged(1, 1));

    b2.cell_mut(0, 1).insert(CellFlags::FLAG);
    assert_eq!(6, b2.reveal_unflagged(1, 1));
}
