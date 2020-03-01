use super::board::CellFlags;
use super::math::Point;
use super::{Context, GameState};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub trait Execute {
    fn execute(&self) -> Result<GameState, String>;
}

#[derive(Builder)]
pub struct MouseUp<'a> {
    mouse_btn: MouseButton,
    mouse_pos: Point,
    context: Option<&'a Context<'a>>,
}

impl<'a> MouseUp<'a> {
    fn left_click_cell(&self, Point { x, y }: Point<u32>) -> GameState {
        let ctx = self.context.unwrap();
        let mut board = ctx.board().borrow_mut();
        let num_revealed = board.reveal_from(x, y);
        if num_revealed > 0 && board.cell(x, y).contains(CellFlags::MINE) {
            GameState::Over
        } else {
            *ctx.game_state()
        }
    }

    fn right_click_cell(&self, Point { x, y }: Point<u32>) -> GameState {
        let ctx = self.context.unwrap();
        ctx.board().borrow_mut().toggle_flag(x, y);
        *ctx.game_state()
    }

    fn middle_click_cell(&self, Point { x, y }: Point<u32>) -> GameState {
        let ctx = self.context.unwrap();
        let mut board = ctx.board().borrow_mut();
        let mines_revealed = board
            .reveal_unflagged(x, y)
            .iter()
            .filter(|p| board.cell(p.x, p.y).contains(CellFlags::MINE))
            .count();
        if mines_revealed > 0 {
            GameState::Over
        } else {
            *ctx.game_state()
        }
    }
}

impl<'a> Execute for MouseUp<'a> {
    fn execute(&self) -> Result<GameState, String> {
        let ctx = self.context.unwrap();
        let game_state = ctx.game_state();

        if let GameState::Over = game_state {
            return Ok(*game_state);
        }

        let Point { x, y } = self.mouse_pos;
        println!("mouse_up = {:?}", point!(x, y));

        let board_pos = ctx.layout().get("board")?.pos();

        match ctx.get_cell_at(x, y, &board_pos) {
            Some(p) => match &self.mouse_btn {
                MouseButton::Left => Ok(self.left_click_cell(p)),
                MouseButton::Right => Ok(self.right_click_cell(p)),
                MouseButton::Middle => Ok(self.middle_click_cell(p)),
                _ => Ok(*game_state),
            },
            None => Ok(*game_state),
        }
    }
}

impl<'a> Default for MouseUp<'a> {
    fn default() -> Self {
        Self {
            mouse_btn: MouseButton::Unknown,
            mouse_pos: point!(0, 0),
            context: None,
        }
    }
}

#[derive(new)]
pub struct KeyDown {
    keycode: Keycode,
    game_state: GameState,
}

impl Execute for KeyDown {
    fn execute(&self) -> Result<GameState, String> {
        let game_state = &self.game_state;
        match &self.keycode {
            Keycode::F2 => Ok(GameState::Reset),
            _ => Ok(*game_state),
        }
    }
}
