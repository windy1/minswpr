use super::{board::CellFlags, math::Point};
use crate::{Context, GameState};
use sdl2::{keyboard::Keycode, mouse::MouseButton};

pub trait Execute {
    fn execute(&self) -> Result<GameState, String>;
}

pub trait Input {
    fn context(&self) -> &Context;
}

#[derive(new, Input)]
pub struct MouseUp<'a> {
    mouse_btn: MouseButton,
    mouse_pos: Point,
    context:   &'a Context<'a>,
}

impl MouseUp<'_> {
    fn left_click_cell(&self, Point { x, y }: Point<u32>) -> GameState {
        let mut board = self.context.board().borrow_mut();
        let num_revealed = board.reveal_from(x, y);
        if num_revealed > 0 && board.cell(x, y).contains(CellFlags::MINE) {
            GameState::Over
        } else {
            self.context.game_state()
        }
    }

    fn right_click_cell(&self, Point { x, y }: Point<u32>) -> GameState {
        let ctx = self.context;
        ctx.board().borrow_mut().toggle_flag(x, y);
        ctx.game_state()
    }

    fn middle_click_cell(&self, Point { x, y }: Point<u32>) -> GameState {
        let ctx = self.context;
        let mut board = ctx.board().borrow_mut();
        let mines_revealed = board
            .reveal_area(x, y)
            .iter()
            .filter(|p| board.cell(p.x, p.y).contains(CellFlags::MINE))
            .count();
        if mines_revealed > 0 {
            GameState::Over
        } else {
            ctx.game_state()
        }
    }
}

impl Execute for MouseUp<'_> {
    fn execute(&self) -> Result<GameState, String> {
        let ctx = self.context;
        let game_state = ctx.game_state();

        if let GameState::Over = game_state {
            return Ok(game_state);
        }

        let Point { x, y } = self.mouse_pos;
        println!("mouse_up = {:?}", point!(x, y));

        let board_pos = ctx.layout().get("board")?.pos();

        match ctx.get_cell_at(x, y, *board_pos) {
            Some(p) => match &self.mouse_btn {
                MouseButton::Left => Ok(self.left_click_cell(p)),
                MouseButton::Right => Ok(self.right_click_cell(p)),
                MouseButton::Middle => Ok(self.middle_click_cell(p)),
                _ => Ok(game_state),
            },
            None => Ok(game_state),
        }
    }
}

#[derive(new, Input)]
pub struct KeyDown<'a> {
    keycode: Keycode,
    context: &'a Context<'a>,
}

impl Execute for KeyDown<'_> {
    fn execute(&self) -> Result<GameState, String> {
        match &self.keycode {
            Keycode::F2 => Ok(GameState::Reset),
            _ => Ok(self.context.game_state()),
        }
    }
}
