use super::board::CellFlags;
use super::math::Point;
use crate::layout::Element;
use crate::layout::OnMouse;
use crate::{Context, GameState};
use sdl2::mouse::{MouseButton, MouseState};

pub trait MouseEvent {
    /// Returns the `Point` position of the mouse
    fn mouse_pos(&self) -> Point;
}

/// Event created when a `MouseButton`, is released on the screen
#[derive(new)]
pub struct MouseUpEvent {
    mouse_btn: MouseButton,
    mouse_pos: Point,
}

impl MouseUpEvent {
    /// Returns the `MouseButton` that was released
    pub fn mouse_btn(&self) -> MouseButton {
        self.mouse_btn
    }
}

impl MouseEvent for MouseUpEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

#[derive(new)]
pub struct MouseMoveEvent {
    mouse_state: MouseState,
    mouse_pos: Point,
}

impl MouseMoveEvent {
    pub fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }
}

impl MouseEvent for MouseMoveEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

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
                MouseButton::Right => self::on_right_click_cell(ctx, p, game_state),
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

fn on_right_click_cell(
    ctx: &Context,
    Point { x, y }: Point<u32>,
    game_state: GameState,
) -> GameState {
    ctx.board().borrow_mut().toggle_flag(x, y);
    game_state
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

/// Returns an `OnMouse<E: MouseEvnet>` handler that will defer `MouseUpEvent`s
/// to the specified `Layout`'s child elements. Panics if the `Node` with
/// `layout_id` is not a `Layout`
pub fn defer_mouse<E, F>(layout_id: &'static str, f: &'static F) -> Box<OnMouse<E>>
where
    E: MouseEvent,
    F: Fn(&Element) -> Option<&OnMouse<E>>,
{
    Box::new(move |ctx, e| {
        ctx.layout()
            .get_layout(layout_id)
            .unwrap()
            .defer_mouse_event(ctx, e, f)
    })
}

pub fn mouse_up(elem: &Element) -> Option<&OnMouse<MouseUpEvent>> {
    elem.mouse_up()
}

pub fn mouse_move(elem: &Element) -> Option<&OnMouse<MouseMoveEvent>> {
    elem.mouse_move()
}
