use super::colors;
use crate::board::{Board, CellFlags};
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::BoardRef;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cell::Ref;
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

    pub fn render(&mut self, canvas: &mut WindowCanvas, fonts: &Fonts) -> Result<(), String> {
        self.draw_base(canvas)?;
        self.draw_cell_borders(canvas)?;
        self.draw_cells(canvas, fonts)
    }

    fn draw_base(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let board = self.board.borrow();
        let cell_attrs = &self.cell_attrs;
        let cell_dimen = cell_attrs.dimen.as_i32();
        let border_width = cell_attrs.border_width as i32;
        let board_cell_dimen = point!(board.width() as i32, board.height() as i32);

        let board_px_dimen = cell_dimen * board_cell_dimen
            + point!(border_width, border_width) * (board_cell_dimen + (1, 1));

        self.base_dimen = point!(board_px_dimen.x as u32, board_px_dimen.y as u32);

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

    fn draw_cells(&self, canvas: &mut WindowCanvas, fonts: &Fonts) -> Result<(), String> {
        let board = self.board.borrow();
        for x in 0..board.width() as u32 {
            for y in 0..board.height() as u32 {
                self.draw_cell(canvas, fonts, x, y)?;
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
            &point!(x, y),
            self.board.borrow(),
            &self.cell_attrs,
            &self.pos,
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

struct RenderCell<'a> {
    fonts: &'a Fonts<'a>,
    pos: &'a Point<u32>,
    board: Ref<'a, Board>,
    cell_attrs: &'a CellAttrs,
    board_pos: &'a Point,
}

impl<'a> RenderCell<'a> {
    pub fn new(
        fonts: &'a Fonts<'a>,
        pos: &'a Point<u32>,
        board: Ref<'a, Board>,
        cell_attrs: &'a CellAttrs,
        board_pos: &'a Point,
    ) -> Self {
        Self {
            fonts,
            pos,
            board,
            cell_attrs,
            board_pos,
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let cell = self.board.get_cell(self.pos.x, self.pos.y).unwrap();
        let screen_pos = self.screen_pos(self.pos);

        if cell.contains(CellFlags::REVEALED) {
            self.fill(canvas, &screen_pos, &self.cell_attrs.revealed_color)?;

            let adjacent_mines = self.board.count_adjacent_mines(self.pos.x, self.pos.y);

            if cell.contains(CellFlags::MINE) {
                self.draw_mine(canvas, &screen_pos)?;
            } else if adjacent_mines > 0 {
                self.draw_hint(canvas, &screen_pos, adjacent_mines)?;
            }
        }

        Ok(())
    }

    fn screen_pos(&self, cell_pos: &Point<u32>) -> Point {
        let cell_attrs = self.cell_attrs;
        let cell_dimen = &cell_attrs.dimen.as_i32();
        let border_width = cell_attrs.border_width as i32;

        let step = *cell_dimen + (border_width, border_width);
        let delta_pos = step * (cell_pos.x as i32, cell_pos.y as i32);
        let origin = point!(
            self.board_pos.x + border_width,
            self.board_pos.y + border_width
        );

        origin + delta_pos
    }

    fn fill(&self, canvas: &mut WindowCanvas, pos: &Point, color: &Color) -> Result<(), String> {
        let cell_dimen = &self.cell_attrs.dimen;
        canvas.set_draw_color(*color);
        canvas.fill_rect(Rect::new(
            pos.x,
            pos.y,
            cell_dimen.width(),
            cell_dimen.height(),
        ))
    }

    fn draw_mine(&self, canvas: &mut WindowCanvas, screen_pos: &Point) -> Result<(), String> {
        let cell_attrs = &self.cell_attrs;
        let cell_dimen = cell_attrs.dimen.as_i32();
        let mine_dimen = cell_attrs.mine_dimen.as_i32();
        let mine_pos = *screen_pos + cell_dimen / (2, 2) - mine_dimen / (2, 2);

        canvas.set_draw_color(cell_attrs.mine_color);
        canvas.fill_rect(Rect::new(
            mine_pos.x,
            mine_pos.y,
            mine_dimen.width() as u32,
            mine_dimen.height() as u32,
        ))
    }

    fn draw_hint(
        &self,
        canvas: &mut WindowCanvas,
        screen_pos: &Point,
        hint: usize,
    ) -> Result<(), String> {
        let textures = canvas.texture_creator();

        let surface = self
            .fonts
            .get("board.cell")?
            .render(&hint.to_string())
            .blended(colors::GREEN)
            .map_err(|e| e.to_string())?;

        let texture = textures
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let tq = texture.query();

        let cell_dimen = &self.cell_attrs.dimen.as_i32();
        let tex_dimen = point!(tq.width as i32, tq.height as i32);
        let hint_pos = *screen_pos + *cell_dimen / (2, 2) - tex_dimen / (2, 2);

        canvas.copy(
            &texture,
            None,
            Some(Rect::new(hint_pos.x, hint_pos.y, tq.width, tq.height)),
        )
    }
}
