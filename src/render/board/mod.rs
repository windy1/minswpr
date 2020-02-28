mod cell;

use super::Render;
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::{BoardRef, CellConfig};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cmp;
use std::rc::Rc;

use self::cell::RenderCell;

pub struct RenderBoard<'a, 'ttf> {
    fonts: Rc<Fonts<'ttf>>,
    board: BoardRef,
    base_dimen: Dimen,
    cell_config: &'a CellConfig,
}

impl<'a, 'ttf> Render for RenderBoard<'a, 'ttf> {
    fn render(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        self.draw_base(canvas, pos)?;
        self.draw_cell_borders(canvas, pos)?;
        self.draw_cells(canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        self.base_dimen
    }
}

impl<'a, 'ttf> RenderBoard<'a, 'ttf> {
    pub fn new(fonts: Rc<Fonts<'ttf>>, board: BoardRef, cell_config: &'a CellConfig) -> Self {
        let cell_dimen = cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;

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
            cell_config,
            base_dimen,
        }
    }

    fn draw_base(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        canvas.set_draw_color(self.cell_config.color);
        canvas.fill_rect(Rect::new(
            pos.x,
            pos.y,
            self.base_dimen.width(),
            self.base_dimen.height(),
        ))
    }

    fn draw_cell_borders(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        canvas.set_draw_color(self.cell_config.border_color);

        let cell_config = &self.cell_config;
        let cell_dimen = cell_config.dimen;
        let base_dimen = &self.base_dimen;

        let board_height = base_dimen.height();
        let board_width = base_dimen.width();

        let cell_width = cell_dimen.width() as usize;
        let cell_height = cell_dimen.height() as usize;

        let border_width = cell_config.border_width;

        let x_max = pos.x + board_width as i32;
        let y_max = pos.y + board_height as i32;
        let x_step = cell_width + border_width as usize;

        for x in (pos.x..x_max).step_by(x_step) {
            canvas.fill_rect(Rect::new(x, pos.y, border_width, board_height))?;
        }

        let y_step = cell_height + border_width as usize;

        for y in (pos.y..y_max).step_by(y_step) {
            canvas.fill_rect(Rect::new(pos.x, y as i32, board_width, border_width))?;
        }

        Ok(())
    }

    fn draw_cells(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        let board = self.board.borrow();
        for x in 0..board.width() as u32 {
            for y in 0..board.height() as u32 {
                self.draw_cell(canvas, pos, &self.fonts, x, y)?;
            }
        }
        Ok(())
    }

    fn calc_cell_screen_pos(
        cell_pos: &Point<u32>,
        board_pos: &Point,
        config: &CellConfig,
    ) -> Point {
        let cell_pos = cell_pos.as_i32();
        let cell_dimen = &config.dimen.as_i32();
        let border_width = config.border_width as i32;

        let step = *cell_dimen + (border_width, border_width);
        let delta_pos = step * cell_pos;
        let origin = point!(board_pos.x + border_width, board_pos.y + border_width);

        origin + delta_pos
    }

    fn draw_cell(
        &self,
        canvas: &mut WindowCanvas,
        pos: &Point,
        fonts: &Fonts,
        x: u32,
        y: u32,
    ) -> Result<(), String> {
        RenderCell::new(fonts, self.board.borrow(), &point!(x, y), &self.cell_config).render(
            canvas,
            &Self::calc_cell_screen_pos(&point!(x, y), pos, &self.cell_config),
        )
    }
}
