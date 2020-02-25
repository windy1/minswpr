use super::board::CellFlags;
use super::render::board::RenderBoard;

pub struct Input<T> {
    meta: Option<T>,
}

impl<T> Input<T> {
    pub fn new() -> Self {
        Self { meta: None }
    }

    pub fn meta<'a>(&'a mut self, meta: T) -> &'a mut Self {
        self.meta = Some(meta);
        self
    }

    pub fn take_meta(&mut self) -> Option<T> {
        self.meta.take()
    }
}

pub trait MouseUp {
    fn mouse_up<'a>(&'a mut self, x: i32, y: i32) -> Result<&'a mut Self, String>;
}

impl MouseUp for Input<RenderBoard> {
    fn mouse_up<'a>(&'a mut self, x: i32, y: i32) -> Result<&'a mut Self, String> {
        let board_render = self.meta.as_mut().ok_or_else(|| "missing RenderBoard")?;
        match board_render.get_cell_at(x, y) {
            Some(p) => match board_render
                .board_mut()
                .get_cell_mut(p.x as u32, p.y as u32)
            {
                Some(c) => c.insert(CellFlags::MINE),
                None => {}
            },
            None => {}
        }
        Ok(self)
    }
}
