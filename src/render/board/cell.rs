use crate::board::{Board, CellFlags};
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::render::colors;
use crate::render::Render;
use crate::CellConfig;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cell::Ref;

pub(super) struct RenderCell<'a> {
    fonts: &'a Fonts<'a>,
    board: Ref<'a, Board>,
    pos: &'a Point<u32>,
    screen_pos: Point,
    config: &'a CellConfig,
}

impl<'a> Render for RenderCell<'a> {
    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let cell = self.board.get_cell(self.pos.x, self.pos.y).unwrap();
        let config = &self.config;
        let mines = &config.mines;
        let flags = &config.flags;

        if cell.contains(CellFlags::REVEALED) {
            let is_mine = cell.contains(CellFlags::MINE);
            let fill_color = if is_mine {
                &mines.revealed_color
            } else {
                &config.revealed_color
            };
            self.fill(canvas, fill_color)?;

            let adjacent_mines = self.board.count_adjacent_mines(self.pos.x, self.pos.y);

            if is_mine {
                self.draw_centered_rect(canvas, &mines.dimen, &mines.color)?;
            } else if adjacent_mines > 0 {
                self.draw_hint(canvas, adjacent_mines)?;
            }
        } else if cell.contains(CellFlags::FLAG) {
            self.draw_centered_rect(canvas, &flags.dimen, &flags.color)?;
        }

        Ok(())
    }
}

impl<'a> RenderCell<'a> {
    pub fn new(
        fonts: &'a Fonts<'a>,
        board: Ref<'a, Board>,
        pos: &'a Point<u32>,
        board_pos: &'a Point,
        config: &'a CellConfig,
    ) -> Self {
        let screen_pos = Self::calc_screen_pos(pos, board_pos, config);
        Self {
            fonts,
            board,
            pos,
            screen_pos,
            config,
        }
    }

    fn calc_screen_pos(pos: &Point<u32>, board_pos: &Point, config: &CellConfig) -> Point {
        let cell_pos = pos.as_i32();
        let cell_dimen = &config.dimen.as_i32();
        let border_width = config.border_width as i32;

        let step = *cell_dimen + (border_width, border_width);
        let delta_pos = step * cell_pos;
        let origin = point!(board_pos.x + border_width, board_pos.y + border_width);

        origin + delta_pos
    }

    fn fill(&self, canvas: &mut WindowCanvas, color: &Color) -> Result<(), String> {
        let cell_dimen = &self.config.dimen;
        canvas.set_draw_color(*color);
        canvas.fill_rect(Rect::new(
            self.screen_pos.x,
            self.screen_pos.y,
            cell_dimen.width(),
            cell_dimen.height(),
        ))
    }

    fn draw_centered_rect(
        &self,
        canvas: &mut WindowCanvas,
        dimen: &Dimen,
        color: &Color,
    ) -> Result<(), String> {
        let cell_dimen = self.config.dimen.as_i32();
        let dimen = dimen.as_i32();
        let pos = self.screen_pos + cell_dimen / (2, 2) - dimen / (2, 2);

        canvas.set_draw_color(*color);
        canvas.fill_rect(Rect::new(
            pos.x,
            pos.y,
            dimen.width() as u32,
            dimen.height() as u32,
        ))
    }

    fn draw_hint(&self, canvas: &mut WindowCanvas, hint: usize) -> Result<(), String> {
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

        let cell_dimen = &self.config.dimen.as_i32();
        let tex_dimen = point!(tq.width as i32, tq.height as i32);
        let hint_pos = self.screen_pos + *cell_dimen / (2, 2) - tex_dimen / (2, 2);

        canvas.copy(
            &texture,
            None,
            Some(Rect::new(hint_pos.x, hint_pos.y, tq.width, tq.height)),
        )
    }
}
