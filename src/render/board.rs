use super::colors;
use crate::board::Board;
use crate::math::{Dimen, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

pub struct RenderBoard {
    board: Board,
    cell_attrs: CellAttrs,
    base_dimen: Dimen,
    pos: Point,
}

impl RenderBoard {
    pub fn new(board: Board, cell_attrs: CellAttrs) -> Self {
        RenderBoard {
            board,
            cell_attrs,
            base_dimen: Dimen::new(0, 0),
            pos: Point::new(10, 10),
        }
    }

    pub fn builder() -> RenderBoardBuilder {
        RenderBoardBuilder::new()
    }

    pub fn render<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) -> Result<(), String> {
        self.draw_base(canvas)?;
        self.draw_cell_borders(canvas)?;
        self.draw_mines(canvas)?;
        Ok(())
    }

    fn draw_base<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) -> Result<(), String> {
        let cell_dimen = &self.cell_attrs.dimen;
        let cell_border_width = self.cell_attrs.border_width as u32;

        let board_cell_width = self.board.width() as u32;
        let board_px_width = cell_dimen.width() as u32 * board_cell_width
            + cell_border_width * (board_cell_width + 1);

        let board_cell_height = self.board.height() as u32;
        let board_px_height = cell_dimen.height() as u32 * board_cell_height
            + cell_border_width * (board_cell_height + 1);

        self.base_dimen.set_width(board_px_width);
        self.base_dimen.set_height(board_px_height);

        canvas.set_draw_color(self.cell_attrs.color);
        canvas.fill_rect(Rect::new(
            self.pos.x,
            self.pos.y,
            board_px_width,
            board_px_height,
        ))?;

        Ok(())
    }

    fn draw_cell_borders<T: RenderTarget>(&self, canvas: &mut Canvas<T>) -> Result<(), String> {
        canvas.set_draw_color(self.cell_attrs.border_color);

        let board_height = self.base_dimen.height();
        let board_width = self.base_dimen.width();
        let x_max = self.pos.x + board_width as i32;
        let y_max = self.pos.y + board_height as i32;
        let cell_width = self.cell_attrs.dimen.width() as usize;
        let border_width = self.cell_attrs.border_width;
        let x_step = cell_width + border_width as usize;

        for x in (self.pos.x..x_max).step_by(x_step) {
            canvas.fill_rect(Rect::new(x, self.pos.y, border_width, board_height))?;
        }

        let cell_height = self.cell_attrs.dimen.height() as usize;
        let y_step = cell_height + border_width as usize;

        for y in (self.pos.y..y_max).step_by(y_step) {
            canvas.fill_rect(Rect::new(self.pos.x, y as i32, board_width, border_width))?;
        }

        Ok(())
    }

    fn draw_mines<T: RenderTarget>(&self, canvas: &mut Canvas<T>) -> Result<(), String> {
        canvas.set_draw_color(self.cell_attrs.mine_color);

        let mine_width = self.cell_attrs.mine_dimen.width();
        let mine_height = self.cell_attrs.mine_dimen.height();
        let cell_width = self.cell_attrs.dimen.width();
        let cell_height = self.cell_attrs.dimen.height();

        for mine_pos in self.board.mine_positions() {
            let cell_pos = self.cell_pos(mine_pos.x as u32, mine_pos.y as u32);
            let mine_pos = cell_pos
                + (
                    (cell_width / 2 - (mine_width / 2)) as i32,
                    (cell_height / 2 - (mine_height / 2)) as i32,
                );
            canvas.fill_rect(Rect::new(mine_pos.x, mine_pos.y, mine_width, mine_height))?;
        }

        Ok(())
    }

    fn cell_pos(&self, x: u32, y: u32) -> Point {
        let border_width = self.cell_attrs.border_width as i32;
        let cell_dimen = self.cell_attrs.dimen;
        let step_x = (cell_dimen.width() + border_width as u32) as i32;
        let step_y = (cell_dimen.height() + border_width as u32) as i32;
        let dx = x as i32 * step_x;
        let dy = y as i32 * step_y;
        let origin = Point::new(self.pos.x + border_width, self.pos.y + border_width);
        origin + (dx, dy)
    }
}

pub struct RenderBoardBuilder {
    board: Board,
    cell_attrs: CellAttrs,
}

impl RenderBoardBuilder {
    pub fn new() -> Self {
        RenderBoardBuilder {
            board: Board::empty(),
            cell_attrs: CellAttrs::new(),
        }
    }

    pub fn board(mut self, board: Board) -> Self {
        self.board = board;
        self
    }

    pub fn cell_attrs(mut self, cell_attrs: CellAttrs) -> Self {
        self.cell_attrs = cell_attrs;
        self
    }

    pub fn build(self) -> RenderBoard {
        RenderBoard::new(self.board, self.cell_attrs)
    }
}

pub struct CellAttrs {
    dimen: Dimen,
    border_width: u32,
    color: Color,
    border_color: Color,
    mine_color: Color,
    mine_dimen: Dimen,
}

impl CellAttrs {
    pub fn new() -> Self {
        CellAttrs {
            dimen: Dimen::new(0, 0),
            border_width: 0,
            color: colors::WHITE,
            border_color: colors::BLACK,
            mine_color: colors::BLACK,
            mine_dimen: Dimen::new(0, 0),
        }
    }

    pub fn dimen(mut self, width: u32, height: u32) -> Self {
        self.dimen = Dimen::new(width, height);
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

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn mine_color(mut self, mine_color: Color) -> Self {
        self.mine_color = mine_color;
        self
    }

    pub fn mine_dimen(mut self, mine_dimen: Dimen) -> Self {
        self.mine_dimen = mine_dimen;
        self
    }
}

impl Default for CellAttrs {
    fn default() -> Self {
        CellAttrs::new()
            .dimen(50, 50)
            .border_width(1)
            .border_color(colors::WHITE)
            .color(colors::RED)
            .mine_color(colors::BLACK)
            .mine_dimen(Dimen::new(20, 20))
    }
}
