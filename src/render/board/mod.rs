mod cell;

use super::Render;
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::BoardRef;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cmp;

pub use self::cell::*;

pub struct RenderBoard<'a> {
    fonts: &'a Fonts<'a>,
    board: BoardRef,
    base_dimen: Dimen,
    pos: Point,
    cell_attrs: CellAttrs,
}

impl<'a> Render for RenderBoard<'a> {
    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.draw_base(canvas)?;
        self.draw_cell_borders(canvas)?;
        self.draw_cells(canvas)
    }
}

impl<'a> RenderBoard<'a> {
    pub fn new(fonts: &'a Fonts<'a>, board: BoardRef, cell_attrs: CellAttrs) -> Self {
        let cell_dimen = cell_attrs.dimen.as_i32();
        let border_width = cell_attrs.border_width as i32;

        let board_cell_dimen: Point;
        {
            let b = board.borrow();
            board_cell_dimen = point!(b.width() as i32, b.height() as i32);
        }

        let board_px_dimen = cell_dimen * board_cell_dimen
            + point!(border_width, border_width) * (board_cell_dimen + (1, 1));

        let base_dimen = point!(board_px_dimen.x as u32, board_px_dimen.y as u32);

        Self {
            fonts,
            board,
            cell_attrs,
            base_dimen,
            pos: Point::new(10, 10),
        }
    }

    fn draw_base(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.cell_attrs.color);
        canvas.fill_rect(Rect::new(
            self.pos.x,
            self.pos.y,
            self.base_dimen.width(),
            self.base_dimen.height(),
        ))
    }

    fn draw_cell_borders(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.cell_attrs.border_color);

        let cell_attrs = &self.cell_attrs;
        let cell_dimen = cell_attrs.dimen;
        let base_dimen = &self.base_dimen;

        let board_height = base_dimen.height();
        let board_width = base_dimen.width();

        let cell_width = cell_dimen.width() as usize;
        let cell_height = cell_dimen.height() as usize;

        let border_width = cell_attrs.border_width;

        let x_max = self.pos.x + board_width as i32;
        let y_max = self.pos.y + board_height as i32;
        let x_step = cell_width + border_width as usize;

        for x in (self.pos.x..x_max).step_by(x_step) {
            canvas.fill_rect(Rect::new(x, self.pos.y, border_width, board_height))?;
        }

        let y_step = cell_height + border_width as usize;

        for y in (self.pos.y..y_max).step_by(y_step) {
            canvas.fill_rect(Rect::new(self.pos.x, y as i32, board_width, border_width))?;
        }

        Ok(())
    }

    fn draw_cells(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let board = self.board.borrow();
        for x in 0..board.width() as u32 {
            for y in 0..board.height() as u32 {
                self.draw_cell(canvas, self.fonts, x, y)?;
            }
        }
        Ok(())
    }

    fn draw_cell(
        &self,
        canvas: &mut WindowCanvas,
        fonts: &Fonts,
        x: u32,
        y: u32,
    ) -> Result<(), String> {
        RenderCell::new(
            fonts,
            self.board.borrow(),
            &point!(x, y),
            &self.pos,
            &self.cell_attrs,
        )
        .render(canvas)
    }

    pub fn get_cell_at(&self, x: i32, y: i32) -> Option<Point<u32>> {
        let base_dimen = &self.base_dimen;
        let min_x = self.pos.x;
        let min_y = self.pos.y;
        let max_x = min_x + base_dimen.width() as i32;
        let max_y = min_y + base_dimen.height() as i32;

        if x < min_x || x > max_x || y < min_y || y > max_y {
            return None;
        }

        let cell_attrs = &self.cell_attrs;
        let cell_dimen = &cell_attrs.dimen.as_i32();
        let border_width = cell_attrs.border_width as i32;
        let board = self.board.borrow();
        let screen_pos = point!(x, y);

        let mut c = (screen_pos - self.pos) / (*cell_dimen + (border_width, border_width));
        c.x = cmp::min(c.x, board.width() as i32 - 1);
        c.y = cmp::min(c.y, board.height() as i32 - 1);

        Some(point!(c.x as u32, c.y as u32))
    }
}
