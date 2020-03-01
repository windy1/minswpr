mod cell;

use self::cell::RenderCellBuilder;
use super::Render;
use crate::config::CellConfig;
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::BoardRef;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::rc::Rc;

#[derive(Clone)]
pub struct RenderBoard<'ttf> {
    fonts:       Rc<Fonts<'ttf>>,
    board:       BoardRef,
    dimen:       Dimen,
    cell_config: CellConfig,
}

impl Render for RenderBoard<'_> {
    fn render(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        render_rect!(self.dimen, self.cell_config.color, canvas, pos)?;
        self.draw_cell_borders(canvas, pos)?;
        self.draw_cells(canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}

impl<'ttf> RenderBoard<'ttf> {
    pub fn new(fonts: Rc<Fonts<'ttf>>, board: BoardRef, cell_config: CellConfig) -> Self {
        let cell_dimen = cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;

        let board_cell_dimen = {
            let b = board.borrow();
            point!(b.width() as i32, b.height() as i32)
        };

        let board_px_dimen = cell_dimen * board_cell_dimen
            + point!(border_width, border_width) * (board_cell_dimen + (1, 1));

        let dimen = point!(board_px_dimen.x as u32, board_px_dimen.y as u32);

        Self {
            fonts,
            board,
            cell_config,
            dimen,
        }
    }

    fn draw_cell_borders(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        canvas.set_draw_color(self.cell_config.border_color);

        let cell_config = &self.cell_config;

        let Dimen {
            x: cell_width,
            y: cell_height,
        } = cell_config.dimen;

        let Dimen {
            x: board_width,
            y: board_height,
        } = self.dimen;

        let border_width = cell_config.border_width;

        let x_max = pos.x + board_width as i32;
        let y_max = pos.y + board_height as i32;
        let x_step = (cell_width + border_width) as usize;

        for x in (pos.x..x_max).step_by(x_step) {
            canvas.fill_rect(Rect::new(x, pos.y, border_width, board_height))?;
        }

        let y_step = (cell_height + border_width) as usize;

        for y in (pos.y..y_max).step_by(y_step) {
            canvas.fill_rect(Rect::new(pos.x, y as i32, board_width, border_width))?;
        }

        Ok(())
    }

    fn draw_cells(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        let b = self.board.borrow();
        for x in 0..b.width() as u32 {
            for y in 0..b.height() as u32 {
                self.draw_cell(canvas, pos, &self.fonts, x, y)?;
            }
        }
        Ok(())
    }

    fn calc_cell_screen_pos(cell_pos: Point<u32>, board_pos: Point, config: &CellConfig) -> Point {
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
        pos: Point,
        fonts: &Fonts,
        x: u32,
        y: u32,
    ) -> Result<(), String> {
        let screen_pos = Self::calc_cell_screen_pos(point!(x, y), pos, &self.cell_config);
        RenderCellBuilder::default()
            .fonts(fonts)
            .board(&self.board.borrow())
            .board_pos(point!(x, y))
            .config(&self.cell_config)
            .build()?
            .render(canvas, screen_pos)
    }
}
