use crate::board::{Board, CellFlags};
use crate::config::CellConfig;
use crate::draw::text::{self, Text};
use crate::draw::{Draw, DrawContext};
use crate::math::{Dimen, Point};
use crate::MsResult;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Builder)]
pub(super) struct DrawCell<'a> {
    board: &'a Board,
    board_pos: Point<u32>,
    config: &'a CellConfig,
}

impl DrawCell<'_> {
    pub fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        let cell = self.board.cell(self.board_pos.x, self.board_pos.y);
        if cell.contains(CellFlags::REVEALED) {
            self.draw_revealed(*cell, ctx, pos)
        } else {
            self.draw_hidden(*cell, ctx, pos)
        }
    }

    fn draw_revealed(&self, cell: CellFlags, ctx: &DrawContext, pos: Point) -> MsResult {
        let mines = &self.config.mines;

        let is_mine = cell.contains(CellFlags::MINE);
        let fill_color = if is_mine {
            mines.revealed_color
        } else {
            self.config.revealed_color
        };
        draw_rect!(self.config.dimen, fill_color, ctx, pos)?;

        let adjacent_mines = self
            .board
            .count_adjacent_mines(self.board_pos.x, self.board_pos.y);

        if is_mine {
            self.draw_centered_rect(&ctx, pos, mines.dimen, mines.color)
        } else if adjacent_mines > 0 {
            self.draw_hint(ctx, pos, adjacent_mines)
        } else {
            Ok(())
        }
    }

    fn draw_hidden(&self, cell: CellFlags, ctx: &DrawContext, pos: Point) -> MsResult {
        if cell.contains(CellFlags::PRESSED) {
            draw_rect!(self.config.dimen, self.config.pressed_color, ctx, pos)?;
        }

        if cell.contains(CellFlags::FLAG) {
            let flags = &self.config.flags;
            self.draw_centered_rect(&ctx, pos, flags.dimen, flags.color)
        } else {
            Ok(())
        }
    }

    fn draw_centered_rect(
        &self,
        ctx: &DrawContext,
        pos: Point,
        dimen: Dimen,
        color: Color,
    ) -> MsResult {
        let cell_dimen = self.config.dimen.as_i32();
        let pos = pos + cell_dimen / (2, 2) - dimen.as_i32() / (2, 2);
        draw_rect!(dimen, color, ctx, pos)
    }

    fn draw_hint(&self, ctx: &DrawContext, pos: Point, hint: usize) -> MsResult {
        let text = text::make_text(ctx, Text::new(hint, "board.cell", color!(green)))?;
        let tq = text.query();

        let cell_dimen = &self.config.dimen.as_i32();
        let tex_dimen = point!(tq.width as i32, tq.height as i32);
        let hint_pos = pos + *cell_dimen / (2, 2) - tex_dimen / (2, 2);

        ctx.canvas().copy(
            text.texture(),
            None,
            Some(Rect::new(hint_pos.x, hint_pos.y, tq.width, tq.height)),
        )
    }
}
