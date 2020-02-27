use super::board::CellFlags;
use super::math::Point;
use super::render::board::RenderBoard;
use super::{BoardRef, GameState};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub struct Input<T> {
    meta: Option<T>,
}

impl<T> Input<T> {
    pub fn new() -> Self {
        Self { meta: None }
    }

    pub fn with_meta(meta: T) -> Self {
        Self { meta: Some(meta) }
    }
}

pub trait Execute {
    fn execute(&self) -> Result<GameState, String>;
}

pub struct MouseUp<'a> {
    mouse_btn: MouseButton,
    mouse_pos: Point,
    board: Option<BoardRef>,
    board_render: Option<&'a RenderBoard<'a>>,
    game_state: GameState,
}

impl<'a> MouseUp<'a> {
    pub fn new() -> Self {
        Self {
            mouse_btn: MouseButton::Unknown,
            mouse_pos: point!(0, 0),
            board: None,
            board_render: None,
            game_state: GameState::Unknown,
        }
    }

    pub fn mouse_btn(mut self, mouse_btn: MouseButton) -> Self {
        self.mouse_btn = mouse_btn;
        self
    }

    pub fn mouse_pos(mut self, x: i32, y: i32) -> Self {
        self.mouse_pos = point!(x, y);
        self
    }

    pub fn board(mut self, board: BoardRef) -> Self {
        self.board = Some(board);
        self
    }

    pub fn board_render(mut self, board_render: &'a RenderBoard) -> Self {
        self.board_render = Some(board_render);
        self
    }

    pub fn game_state(mut self, game_state: GameState) -> Self {
        self.game_state = game_state;
        self
    }
}

impl<'a> Execute for Input<MouseUp<'a>> {
    fn execute(&self) -> Result<GameState, String> {
        let meta = self.meta.as_ref().unwrap();
        let game_state = meta.game_state;

        if let GameState::Over = game_state {
            return Ok(game_state);
        }

        let Point { x, y } = meta.mouse_pos;
        println!("mouse_up = {:?}", point!(x, y));

        let cell = meta.board_render.unwrap().get_cell_at(x, y);
        let mut board = meta.board.as_ref().unwrap().borrow_mut();

        match cell {
            Some(p) => match meta.mouse_btn {
                MouseButton::Left => {
                    if board.reveal_from(p.x, p.y) > 0
                        && board.cell(p.x, p.y).contains(CellFlags::MINE)
                    {
                        Ok(GameState::Over)
                    } else {
                        Ok(game_state)
                    }
                }
                MouseButton::Right => {
                    board.toggle_flag(p.x, p.y);
                    Ok(game_state)
                }
                _ => Ok(game_state),
            },
            None => Ok(game_state),
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

impl Execute for Input<KeyDown> {
    fn execute(&self) -> Result<GameState, String> {
        let meta = self.meta.as_ref().unwrap();
        let game_state = meta.game_state;
        match meta.keycode {
            Keycode::F2 => Ok(GameState::Reset),
            _ => Ok(game_state),
        }
    }
}
