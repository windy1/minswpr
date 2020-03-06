use crate::board::CellFlags;
use crate::math::Point;
use crate::{Context, GameState};
use sdl2::mouse::MouseButton;

use super::events::*;

/// Event handler for receiving `DrawBoard` clicks
///
/// This function has no effect if the current `GameState` is `Over`. Otherwise,
/// it retrieves the cell at the clicked position and forwards it to another
/// handler depending on which mouse button was released
///
/// If this is the first cell that was clicked on the board, the `GameState` is
/// moved from `Ready` to `Start`
///
/// # Arguments
/// * `ctx` - The game `Context`
/// * `e` - The `MouseUpEvent` to handle
pub fn on_click_board(ctx: &Context, e: MouseUpEvent) -> GameState {
    let Point { x, y } = e.mouse_pos();
    let game_state = ctx.game_state();

    // if the current game is over, freeze the board
    if let GameState::Over = game_state {
        return game_state;
    }

    match ctx.get_cell_at(x, y) {
        Some(p) => {
            // start the game when the first cell of a fresh board is clicked
            let game_state = if let GameState::Ready = game_state {
                GameState::Start
            } else {
                game_state
            };

            match &e.mouse_btn() {
                MouseButton::Left => self::on_left_click_cell(ctx, p, game_state),
                MouseButton::Middle => self::on_middle_click_cell(ctx, p, game_state),
                _ => game_state,
            }
        }
        None => game_state,
    }
}

fn on_left_click_cell(
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

fn on_middle_click_cell(
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

pub fn on_mouse_move_board(ctx: &Context, e: MouseMoveEvent) -> GameState {
    if let GameState::Over = ctx.game_state() {
        return ctx.game_state();
    }

    if !e.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
        // TODO: different mouse button have different effects here
        return ctx.game_state();
    }

    let Point { x, y } = e.mouse_pos();
    match ctx.get_cell_at(x, y) {
        Some(p) => {
            let mut board = ctx.board().borrow_mut();
            board.clear_all(CellFlags::PRESSED);
            board.cell_mut(p.x, p.y).insert(CellFlags::PRESSED);
            ctx.game_state()
        }
        None => ctx.game_state(),
    }
}

pub fn on_mouse_down_board(ctx: &Context, e: MouseDownEvent) -> GameState {
    if let GameState::Over = ctx.game_state() {
        return ctx.game_state();
    }

    let Point { x, y } = e.mouse_pos();

    match (ctx.get_cell_at(x, y), e.mouse_btn()) {
        (Some(p), MouseButton::Left) => {
            // set "pressed" state
            ctx.board()
                .borrow_mut()
                .cell_mut(p.x, p.y)
                .insert(CellFlags::PRESSED);
            ctx.game_state()
        }
        (Some(p), MouseButton::Middle) => ctx.game_state(),
        (Some(p), MouseButton::Right) => {
            // place flag
            ctx.board().borrow_mut().toggle_flag(p.x, p.y);
            ctx.game_state()
        }
        _ => ctx.game_state(),
    }
}

pub fn on_mouse_leave_board(ctx: &Context, _: MouseLeaveEvent) -> GameState {
    ctx.board().borrow_mut().clear_all(CellFlags::PRESSED);
    ctx.game_state()
}
