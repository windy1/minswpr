use super::{BoardRef, ButtonRef, GameState, StopwatchRef};
use crate::config::Config;
use crate::control::Button;
use crate::layout::Layout;
use crate::math::Point;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

type ButtonMap = HashMap<&'static str, ButtonRef>;

/// The main game context
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Context {
    config: Config,
    game_state: GameState,
    #[builder(default)]
    layout: Layout,
    board: BoardRef,
    stopwatch: StopwatchRef,
    #[builder(default)]
    buttons: ButtonMap,
}

impl Context {
    /// Returns the application `Config`
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns the current `GameState`
    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    /// Sets the current `GameState` to `game_state`
    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state
    }

    /// Returns the base `Layout`
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Returns the base `layout`
    pub fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }

    /// Sets the root `Layout`
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout
    }

    /// Returns a `RefCell` of the `Board`
    pub fn board(&self) -> &BoardRef {
        &self.board
    }

    /// Returns a `RefCell` of the `Stopwatch`
    pub fn stopwatch(&self) -> &StopwatchRef {
        &self.stopwatch
    }

    pub fn buttons(&self) -> Vec<&ButtonRef> {
        self.buttons.values().collect()
    }

    pub fn button(&self, id: &'static str) -> &ButtonRef {
        &self
            .buttons
            .get(id)
            .unwrap_or_else(|| panic!("missing required Button `{}`", id))
    }

    pub fn insert_button(&mut self, id: &'static str, button: Button) {
        self.buttons.insert(id, Rc::new(RefCell::new(button)));
    }

    /// Return `Some(Point<u32>)` with the board position of the cell that
    /// occupies the point on the screen specified. Returns `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `x` - x position on the screen
    /// * `y` - y position on the screen
    pub fn get_cell_at(&self, x: i32, y: i32) -> Option<Point<u32>> {
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
