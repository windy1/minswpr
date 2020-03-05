use super::board::CellFlags;
use super::math::Point;
use crate::{Context, GameState};
use sdl2::mouse::MouseButton;

#[derive(new)]
pub struct MouseUpEvent {
    mouse_btn: MouseButton,
    mouse_pos: Point,
}

impl MouseUpEvent {
    pub fn mouse_btn(&self) -> MouseButton {
        self.mouse_btn
    }

    pub fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

pub fn left_click_cell(
    ctx: &Context,
    Point { x, y }: Point<u32>,
    game_state: GameState,
) -> GameState {
    let mut board = ctx.board().borrow_mut();
    let num_revealed = board.reveal_from(x, y);
    if num_revealed > 0 && board.cell(x, y).contains(CellFlags::MINE) {
        // hit a mine :(
        GameState::Over
    } else {
        game_state
    }
}

pub fn right_click_cell(
    ctx: &Context,
    Point { x, y }: Point<u32>,
    game_state: GameState,
) -> GameState {
    ctx.board().borrow_mut().toggle_flag(x, y);
    game_state
}

pub fn middle_click_cell(
    ctx: &Context,
    Point { x, y }: Point<u32>,
    game_state: GameState,
) -> GameState {
    let mut board = ctx.board().borrow_mut();
    let mines_revealed = board
        .reveal_area(x, y)
        .iter()
        .filter(|p| board.cell(p.x, p.y).contains(CellFlags::MINE))
        .count();
    if mines_revealed > 0 {
        GameState::Over
    } else {
        game_state
    }
}
