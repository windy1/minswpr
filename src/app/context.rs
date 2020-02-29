use super::{BoardRef, Config, GameState};
use crate::layout::Layout;
use crate::math::Point;
use std::cmp;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Context<'a> {
    config: Config,
    game_state: GameState,
    board: BoardRef,
    layout: Layout<'a>,
}

impl<'a> Context<'a> {
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn board(&self) -> &BoardRef {
        &self.board
    }

    pub fn layout(&self) -> &Layout<'a> {
        &self.layout
    }

    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state
    }

    pub fn get_cell_at(&self, x: i32, y: i32, pos: &Point) -> Option<Point<u32>> {
        let base_dimen = &self.layout.get("board").unwrap().dimen();
        let min_x = pos.x;
        let min_y = pos.y;
        let max_x = min_x + base_dimen.width() as i32;
        let max_y = min_y + base_dimen.height() as i32;

        if x < min_x || x > max_x || y < min_y || y > max_y {
            return None;
        }

        let cell_config = &self.config.board.cells;
        let cell_dimen = &cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;
        let board = self.board.borrow();
        let screen_pos = point!(x, y);

        let mut c = (screen_pos - *pos) / (*cell_dimen + (border_width, border_width));
        c.x = cmp::min(c.x, board.width() as i32 - 1);
        c.y = cmp::min(c.y, board.height() as i32 - 1);

        Some(point!(c.x as u32, c.y as u32))
    }
}
