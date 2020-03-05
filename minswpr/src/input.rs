use super::board::CellFlags;
use super::math::Point;
use crate::MsResult;
use crate::{Context, GameState};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub trait Execute {
    fn execute(&self) -> MsResult<GameState>;
}

pub trait Input {
    fn context(&self) -> &Context;
}

#[derive(new, Input)]
pub struct MouseUp<'a> {
    mouse_btn: MouseButton,
    mouse_pos: Point,
    context: &'a Context,
}

impl MouseUp<'_> {
    fn click_cell(&self, x: i32, y: i32) -> GameState {
        let ctx = self.context;
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

                match &self.mouse_btn {
                    MouseButton::Left => self.left_click_cell(p, game_state),
                    MouseButton::Right => self.right_click_cell(p, game_state),
                    MouseButton::Middle => self.middle_click_cell(p, game_state),
                    _ => game_state,
                }
            }
            None => game_state,
        }
    }

    fn left_click_cell(&self, Point { x, y }: Point<u32>, game_state: GameState) -> GameState {
        let mut board = self.context.board().borrow_mut();
        let num_revealed = board.reveal_from(x, y);
        if num_revealed > 0 && board.cell(x, y).contains(CellFlags::MINE) {
            // hit a mine :(
            GameState::Over
        } else {
            game_state
        }
    }

    fn right_click_cell(&self, Point { x, y }: Point<u32>, game_state: GameState) -> GameState {
        let ctx = self.context;
        ctx.board().borrow_mut().toggle_flag(x, y);
        game_state
    }

    fn middle_click_cell(&self, Point { x, y }: Point<u32>, game_state: GameState) -> GameState {
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
            game_state
        }
    }

    fn click_control(&self, x: i32, y: i32) -> GameState {
        self.context
            .layout()
            .get_layout("control")
            .unwrap()
            .get_at(x, y)
            .filter(|c| c.id() == "reset_button")
            .map(|_| GameState::Reset)
            .unwrap_or_else(|| self.context.game_state())
    }
}

impl Execute for MouseUp<'_> {
    fn execute(&self) -> MsResult<GameState> {
        let ctx = self.context;
        let game_state = ctx.game_state();
        let Point { x, y } = self.mouse_pos;

        println!("mouse_up = {:?}", point!(x, y));

        match ctx.layout().get_at(x, y) {
            Some(c) => match c.id() {
                "board" => Ok(self.click_cell(x, y)),
                "control" => Ok(self.click_control(x, y)),
                _ => Ok(game_state),
            },
            None => Ok(game_state),
        }
    }
}

#[derive(new, Input)]
pub struct KeyDown<'a> {
    keycode: Keycode,
    context: &'a Context,
}

impl Execute for KeyDown<'_> {
    fn execute(&self) -> MsResult<GameState> {
        match &self.keycode {
            Keycode::F2 => Ok(GameState::Reset),
            _ => Ok(self.context.game_state()),
        }
    }
}
