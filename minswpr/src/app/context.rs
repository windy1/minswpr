use super::{BoardRef, GameState, StopwatchRef};
use crate::config::Config;
use crate::layout::Layout;
use crate::math::Point;
use std::cmp;

/// The main game context
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Context {
    config: Config,
    game_state: GameState,
    board: BoardRef,
    stopwatch: StopwatchRef,
    layout: Layout,
}

impl Context {
    /// Returns the current `GameState`
    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    /// Sets the current `GameState`
    ///
    /// # Arguments
    ///
    /// * `game_state` - The new `GameState`
    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state
    }

    /// Returns a `RefCell` of the `Board`
    pub fn board(&self) -> &BoardRef {
        &self.board
    }

    /// Returns a `RefCell` of the `Stopwatch`
    pub fn stopwatch(&self) -> &StopwatchRef {
        &self.stopwatch
    }

    /// Returns the base `Layout`
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Returns the base `layout`
    pub fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }

    /// Returns the application `Config`
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Return `Some(Point<u32>)` with the board position of the cell that
    /// occupies the point on the screen specified
    ///
    /// # Arguments
    ///
    /// * `x` - x position on the screen
    /// * `y` - y position on the screen
    pub fn get_cell_at(&self, x: i32, y: i32, board_pos: Point) -> Option<Point<u32>> {
        let cell_config = &self.config.board.cells;
        let cell_dimen = &cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;
        let board = self.board.borrow();
        let screen_pos = point!(x, y);
        let board_pos = self.layout.get("board").unwrap().pos();

        let mut c = (screen_pos - board_pos) / (*cell_dimen + (border_width, border_width));
        c.x = cmp::min(c.x, board.width() as i32 - 1);
        c.y = cmp::min(c.y, board.height() as i32 - 1);

        Some(point!(c.x as u32, c.y as u32))
    }
}
