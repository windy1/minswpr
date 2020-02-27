use crate::board::{Board, CellFlags};
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::render::colors;
use crate::render::Render;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cell::Ref;

pub(super) struct RenderCell<'a> {
    fonts: &'a Fonts<'a>,
    board: Ref<'a, Board>,
    pos: &'a Point<u32>,
    screen_pos: Point,
    cell_attrs: &'a CellAttrs,
}

impl<'a> Render for RenderCell<'a> {
    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let cell = self.board.get_cell(self.pos.x, self.pos.y).unwrap();
        let cell_attrs = &self.cell_attrs;

        if cell.contains(CellFlags::REVEALED) {
            self.fill(canvas, &cell_attrs.revealed_color)?;

            let adjacent_mines = self.board.count_adjacent_mines(self.pos.x, self.pos.y);

            if cell.contains(CellFlags::MINE) {
                self.draw_centered_rect(canvas, &cell_attrs.mine_dimen, &cell_attrs.mine_color)?;
            } else if adjacent_mines > 0 {
                self.draw_hint(canvas, adjacent_mines)?;
            }
        } else if cell.contains(CellFlags::FLAG) {
            self.draw_centered_rect(canvas, &cell_attrs.flag_dimen, &cell_attrs.flag_color)?;
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
        cell_attrs: &'a CellAttrs,
    ) -> Self {
        let screen_pos = Self::calc_screen_pos(pos, board_pos, cell_attrs);
        Self {
            fonts,
            board,
            pos,
            screen_pos,
            cell_attrs,
        }
    }

    fn calc_screen_pos(pos: &Point<u32>, board_pos: &Point, cell_attrs: &CellAttrs) -> Point {
        let cell_pos = pos.as_i32();
        let cell_attrs = cell_attrs;
        let cell_dimen = &cell_attrs.dimen.as_i32();
        let border_width = cell_attrs.border_width as i32;

        let step = *cell_dimen + (border_width, border_width);
        let delta_pos = step * cell_pos;
        let origin = point!(board_pos.x + border_width, board_pos.y + border_width);

        origin + delta_pos
    }

    fn fill(&self, canvas: &mut WindowCanvas, color: &Color) -> Result<(), String> {
        let cell_dimen = &self.cell_attrs.dimen;
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
        let cell_dimen = self.cell_attrs.dimen.as_i32();
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

        let cell_dimen = &self.cell_attrs.dimen.as_i32();
        let tex_dimen = point!(tq.width as i32, tq.height as i32);
        let hint_pos = self.screen_pos + *cell_dimen / (2, 2) - tex_dimen / (2, 2);

        canvas.copy(
            &texture,
            None,
            Some(Rect::new(hint_pos.x, hint_pos.y, tq.width, tq.height)),
        )
    }
}

pub struct CellAttrs {
    pub dimen: Dimen,
    pub color: Color,
    pub border_width: u32,
    pub border_color: Color,
    pub revealed_color: Color,
    pub mine_color: Color,
    pub mine_dimen: Dimen,
    pub flag_color: Color,
    pub flag_dimen: Dimen,
}

impl CellAttrs {
    pub fn new() -> Self {
        Self {
            dimen: point!(0, 0),
            color: colors::BLACK,
            border_width: 0,
            border_color: colors::BLACK,
            revealed_color: colors::BLACK,
            mine_color: colors::BLACK,
            mine_dimen: point!(0, 0),
            flag_color: colors::BLACK,
            flag_dimen: point!(0, 0),
        }
    }

    pub fn dimen(mut self, width: u32, height: u32) -> Self {
        self.dimen = Dimen::new(width, height);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
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

    pub fn revealed_color(mut self, revealed_color: Color) -> Self {
        self.revealed_color = revealed_color;
        self
    }

    pub fn mine_color(mut self, mine_color: Color) -> Self {
        self.mine_color = mine_color;
        self
    }

    pub fn mine_dimen(mut self, width: u32, height: u32) -> Self {
        self.mine_dimen = point!(width, height);
        self
    }

    pub fn flag_color(mut self, flag_color: Color) -> Self {
        self.flag_color = flag_color;
        self
    }

    pub fn flag_dimen(mut self, width: u32, height: u32) -> Self {
        self.flag_dimen = point!(width, height);
        self
    }
}
