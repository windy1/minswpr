use crate::board::{Board, CellFlags};
use crate::config::CellConfig;
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Builder)]
pub(super) struct RenderCell<'a> {
    fonts: &'a Fonts<'a>,
    board: &'a Board,
    board_pos: Point<u32>,
    config: &'a CellConfig,
}

impl Render for RenderCell<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        let cell = self.board.cell(self.board_pos.x, self.board_pos.y);
        let config = &self.config;
        let mines = &config.mines;
        let flags = &config.flags;

        if cell.contains(CellFlags::REVEALED) {
            let is_mine = cell.contains(CellFlags::MINE);
            let fill_color = if is_mine {
                mines.revealed_color
            } else {
                config.revealed_color
            };
            render_rect!(self.config.dimen, fill_color, canvas, pos)?;

            let adjacent_mines = self
                .board
                .count_adjacent_mines(self.board_pos.x, self.board_pos.y);

            if is_mine {
                self.draw_centered_rect(canvas, pos, mines.dimen, mines.color)?;
            } else if adjacent_mines > 0 {
                self.draw_hint(canvas, pos, adjacent_mines)?;
            }
        } else if cell.contains(CellFlags::FLAG) {
            self.draw_centered_rect(canvas, pos, flags.dimen, flags.color)?;
        }

        Ok(())
    }

    fn dimen(&self) -> Dimen {
        self.config.dimen
    }
}

impl RenderCell<'_> {
    fn draw_centered_rect(
        &self,
        canvas: &mut WindowCanvas,
        pos: Point,
        dimen: Dimen,
        color: Color,
    ) -> Result<(), String> {
        let cell_dimen = self.config.dimen.as_i32();
        let pos = pos + cell_dimen / (2, 2) - dimen.as_i32() / (2, 2);
        render_rect!(dimen, color, canvas, pos)
    }

    fn draw_hint(&self, canvas: &mut WindowCanvas, pos: Point, hint: usize) -> Result<(), String> {
        let textures = canvas.texture_creator();

        let surface = self
            .fonts
            .get("board.cell")?
            .render(&hint.to_string())
            .blended(color!(green))
            .map_err(|e| e.to_string())?;

        let texture = textures
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let tq = texture.query();

        let cell_dimen = &self.config.dimen.as_i32();
        let tex_dimen = point!(tq.width as i32, tq.height as i32);
        let hint_pos = pos + *cell_dimen / (2, 2) - tex_dimen / (2, 2);

        canvas.copy(
            &texture,
            None,
            Some(Rect::new(hint_pos.x, hint_pos.y, tq.width, tq.height)),
        )
    }
}
