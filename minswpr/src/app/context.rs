use super::{BoardRef, GameState};
use crate::config::Config;
use crate::fonts::Fonts;
use crate::layout::{Layout, LayoutBase, RenderRef};
use crate::math::Point;
use crate::render::board::RenderBoard;
use crate::render::control::RenderControl;
use crate::render::{Render, RenderRect};
use std::cmp;
use std::rc::Rc;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Context<'a> {
    config: Config,
    game_state: GameState,
    board: BoardRef,
    layout: LayoutBase<'a>,
}

impl<'a> Context<'a> {
    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state
    }

    pub fn board(&self) -> &BoardRef {
        &self.board
    }

    pub fn layout(&self) -> &LayoutBase {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut LayoutBase<'a> {
        &mut self.layout
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn make_components(&mut self, fonts: &Rc<Fonts<'a>>) {
        let cc = &self.config.control;

        let board = Box::new(RenderBoard::new(
            Rc::clone(fonts),
            Rc::clone(&self.board),
            self.config.board.cells.clone(),
        ));
        let board_width = board.dimen().width();

        let v: Vec<(&'static str, RenderRef<'a>)> = vec![
            (
                "control",
                Box::new(RenderControl::new(
                    Rc::clone(&fonts),
                    cc.clone(),
                    board_width,
                )),
            ),
            (
                "spacer",
                Box::new(RenderRect::new(
                    point!(board_width, cc.spacer_height),
                    cc.spacer_color,
                )),
            ),
            ("board", board),
        ];

        self.layout.insert_all(v)
    }

    pub fn get_cell_at(&self, x: i32, y: i32, pos: Point) -> Option<Point<u32>> {
        let cell_config = &self.config.board.cells;
        let cell_dimen = &cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;
        let board = self.board.borrow();
        let screen_pos = point!(x, y);

        let mut c = (screen_pos - pos) / (*cell_dimen + (border_width, border_width));
        c.x = cmp::min(c.x, board.width() as i32 - 1);
        c.y = cmp::min(c.y, board.height() as i32 - 1);

        Some(point!(c.x as u32, c.y as u32))
    }
}
