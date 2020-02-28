use super::board::CellFlags;
use super::math::Point;
use super::render::board::RenderBoard;
use super::{BoardRef, GameState};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub trait Execute {
    fn execute(&self) -> Result<GameState, String>;
}

#[derive(Builder)]
pub struct MouseUp<'a> {
    mouse_btn: MouseButton,
    mouse_pos: Point,
    board: Option<BoardRef>,
    board_render: Option<&'a RenderBoard<'a>>,
    game_state: GameState,
}

impl<'a> Default for MouseUp<'a> {
    fn default() -> Self {
        Self {
            mouse_btn: MouseButton::Unknown,
            mouse_pos: point!(0, 0),
            board: None,
            board_render: None,
            game_state: GameState::Unknown,
        }
    }
}

impl<'a> Execute for MouseUp<'a> {
    fn execute(&self) -> Result<GameState, String> {
        let game_state = &self.game_state;

        if let GameState::Over = game_state {
            return Ok(*game_state);
        }

        let Point { x, y } = self.mouse_pos;
        println!("mouse_up = {:?}", point!(x, y));

        let cell = &self.board_render.unwrap().get_cell_at(x, y);
        let mut board = self.board.as_ref().unwrap().borrow_mut();

        match cell {
            Some(p) => match &self.mouse_btn {
                MouseButton::Left => {
                    if board.reveal_from(p.x, p.y) > 0
                        && board.cell(p.x, p.y).contains(CellFlags::MINE)
                    {
                        Ok(GameState::Over)
                    } else {
                        Ok(*game_state)
                    }
                }
                MouseButton::Right => {
                    board.toggle_flag(p.x, p.y);
                    Ok(*game_state)
                }
                _ => Ok(*game_state),
            },
            None => Ok(*game_state),
        }
    }
}

pub struct KeyDown {
    keycode: Keycode,
    game_state: GameState,
}

impl KeyDown {
    pub fn new(keycode: Keycode, game_state: GameState) -> Self {
        Self {
            keycode,
            game_state,
        }
    }
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
