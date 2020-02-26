use super::RenderBoard;
use crate::math::Dimen;
use crate::render::colors;
use crate::BoardRef;
use sdl2::pixels::Color;

pub struct RenderBoardBuilder {
    board: Option<BoardRef>,
    cell_attrs: CellAttrs,
}

impl RenderBoardBuilder {
    pub fn new() -> Self {
        Self {
            board: None,
            cell_attrs: CellAttrs::new(),
        }
    }

    pub fn board(mut self, board: BoardRef) -> Self {
        self.board = Some(board);
        self
    }

    pub fn cell_attrs(mut self, cell_attrs: CellAttrs) -> Self {
        self.cell_attrs = cell_attrs;
        self
    }

    pub fn build(self) -> Result<RenderBoard, String> {
        let board = self.board.ok_or_else(|| "missing board")?;
        Ok(RenderBoard::new(board, self.cell_attrs))
    }
}

pub struct CellAttrs {
    pub dimen: Dimen,
    pub color: Color,
    pub border_width: u32,
    pub border_color: Color,
    pub revealed_color: Color,
    pub mine_color: Color,
    pub mine_dimen: Dimen,
}

impl CellAttrs {
    pub fn new() -> Self {
        Self {
            dimen: Dimen::new(0, 0),
            color: colors::BLACK,
            border_width: 0,
            border_color: colors::BLACK,
            revealed_color: colors::BLACK,
            mine_color: colors::BLACK,
            mine_dimen: Dimen::new(0, 0),
        }
    }

    pub fn dimen(mut self, width: u32, height: u32) -> Self {
        self.dimen = Dimen::new(width, height);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = border_width;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }

    pub fn revealed_color(mut self, revealed_color: Color) -> Self {
        self.revealed_color = revealed_color;
        self
    }

    pub fn mine_color(mut self, mine_color: Color) -> Self {
        self.mine_color = mine_color;
        self
    }

    pub fn mine_dimen(mut self, width: u32, height: u32) -> Self {
        self.mine_dimen = Dimen::new(width, height);
        self
    }
}
