use crate::board::CellFlags;
use crate::math::{Dimen, Point};
use crate::BoardRef;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use std::cmp;

mod builder;

pub use self::builder::*;

pub struct RenderBoard {
    board: BoardRef,
    cell_attrs: CellAttrs,
    base_dimen: Dimen,
    pos: Point,
}

impl RenderBoard {
    pub fn new(board: BoardRef, cell_attrs: CellAttrs) -> Self {
        Self {
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
        self.draw_cells(canvas)?;
        Ok(())
    }

    fn draw_base<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) -> Result<(), String> {
        let cell_dimen = &self.cell_attrs.dimen;
        let cell_border_width = self.cell_attrs.border_width as u32;

        let board = self.board.borrow();

        let board_cell_width = board.width() as u32;
        let board_px_width = cell_dimen.width() as u32 * board_cell_width
            + cell_border_width * (board_cell_width + 1);

        let board_cell_height = board.height() as u32;
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

    fn draw_cells<T: RenderTarget>(&self, canvas: &mut Canvas<T>) -> Result<(), String> {
        let board = self.board.borrow();
        for x in 0..board.width() as u32 {
            for y in 0..board.height() as u32 {
                self.draw_cell(canvas, x, y)?;
            }
        }
        Ok(())
    }

    fn draw_cell<T>(&self, canvas: &mut Canvas<T>, x: u32, y: u32) -> Result<(), String>
    where
        T: RenderTarget,
    {
        let board = self.board.borrow();
        let cell = board.get_cell(x, y).unwrap();
        let cell_pos = self.cell_pos(x as u32, y as u32);
        if cell.contains(CellFlags::REVEALED) {
            self.fill_cell(canvas, &cell_pos, &self.cell_attrs.revealed_color)?;

            if cell.contains(CellFlags::MINE) {
                self.draw_mine(canvas, &cell_pos)?;
            }
        }

        Ok(())
    }

    fn fill_cell<T>(
        &self,
        canvas: &mut Canvas<T>,
        cell_pos: &Point,
        color: &Color,
    ) -> Result<(), String>
    where
        T: RenderTarget,
    {
        let cell_dimen = &self.cell_attrs.dimen;
        canvas.set_draw_color(*color);
        canvas.fill_rect(Rect::new(
            cell_pos.x,
            cell_pos.y,
            cell_dimen.width(),
            cell_dimen.height(),
        ))?;
        Ok(())
    }

    fn draw_mine<T>(&self, canvas: &mut Canvas<T>, cell_pos: &Point) -> Result<(), String>
    where
        T: RenderTarget,
    {
        let cell_attrs = &self.cell_attrs;

        let cell_dimen = &cell_attrs.dimen;
        let cell_width = cell_dimen.width();
        let cell_height = cell_dimen.height();

        let mine_dimen = &cell_attrs.mine_dimen;
        let mine_width = mine_dimen.width();
        let mine_height = mine_dimen.height();

        let mine_x = (cell_width / 2 - (mine_width / 2)) as i32;
        let mine_y = (cell_height / 2 - (mine_height / 2)) as i32;
        let mine_pos = *cell_pos + (mine_x, mine_y);

        canvas.set_draw_color(self.cell_attrs.mine_color);
        canvas.fill_rect(Rect::new(mine_pos.x, mine_pos.y, mine_width, mine_height))?;

        Ok(())
    }

    fn cell_pos(&self, x: u32, y: u32) -> Point {
        let cell_attrs = &self.cell_attrs;

        let border_width = cell_attrs.border_width as i32;
        let cell_dimen = &cell_attrs.dimen;

        let step_x = (cell_dimen.width() + border_width as u32) as i32;
        let step_y = (cell_dimen.height() + border_width as u32) as i32;

        let dx = x as i32 * step_x;
        let dy = y as i32 * step_y;

        let origin = Point::new(self.pos.x + border_width, self.pos.y + border_width);

        origin + (dx, dy)
    }

    pub fn get_cell_at(&self, x: i32, y: i32) -> Option<Point> {
        let base_dimen = &self.base_dimen;

        let min_x = self.pos.x;
        let max_x = min_x + base_dimen.width() as i32;

        let min_y = self.pos.y;
        let max_y = min_y + base_dimen.height() as i32;

        if x < min_x || x > max_x || y < min_y || y > max_y {
            return None;
        }

        let cell_attrs = &self.cell_attrs;
        let cell_dimen = cell_attrs.dimen;

        let cell_width = cell_dimen.width() as i32;
        let cell_height = cell_dimen.height() as i32;

        let border_width = cell_attrs.border_width;

        let board = self.board.borrow();

        let cx = (x - min_x) / (cell_width + border_width as i32);
        let cx = cmp::min(cx, board.width() as i32 - 1);

        let cy = (y - min_y) / (cell_height + border_width as i32);
        let cy = cmp::min(cy, board.height() as i32 - 1);

        Some(Point::new(cx, cy))
    }
}
