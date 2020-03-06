mod cell;

use self::cell::DrawCellBuilder;
use super::{Draw, DrawContext};
use crate::board::Board;
use crate::config::CellConfig;
use crate::math::{Dimen, Point};
use crate::models::Model;
use crate::utils;
use crate::MsResult;
use sdl2::rect::Rect;

#[derive(AsAny)]
pub struct DrawBoard {
    board: Model<Board>,
    dimen: Dimen,
    cell_config: CellConfig,
}

impl DrawBoard {
    pub fn new(board: Model<Board>, cell_config: CellConfig) -> Self {
        let cell_dimen = cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;

        let board_cell_dimen = utils::borrow_safe(&board.as_ref(), |b| {
            point!(b.width() as i32, b.height() as i32)
        });

        let board_px_dimen = cell_dimen * board_cell_dimen
            + point!(border_width, border_width) * (board_cell_dimen + (1, 1));

        let dimen = point!(board_px_dimen.x as u32, board_px_dimen.y as u32);

        Self {
            board,
            cell_config,
            dimen,
        }
    }
}

impl Draw for DrawBoard {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        draw_rect!(self.dimen, self.cell_config.color, ctx, pos)?;
        self.draw_cell_borders(ctx, pos)?;
        self.draw_cells(ctx, pos)
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}

impl DrawBoard {
    fn draw_cell_borders(&self, ctx: &DrawContext, pos: Point) -> MsResult {
        let mut canvas = ctx.canvas();
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

    fn draw_cells(&self, ctx: &DrawContext, pos: Point) -> MsResult {
        let b = self.board.borrow();
        for x in 0..b.width() as u32 {
            for y in 0..b.height() as u32 {
                self.draw_cell(ctx, pos, x, y)?;
            }
        }
        Ok(())
    }

    fn draw_cell(&self, ctx: &DrawContext, pos: Point, x: u32, y: u32) -> MsResult {
        let screen_pos = Self::calc_cell_screen_pos(point!(x, y), pos, &self.cell_config);
        DrawCellBuilder::default()
            .board(&self.board.borrow())
            .board_pos(point!(x, y))
            .config(&self.cell_config)
            .build()?
            .draw(ctx, screen_pos)
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
}
