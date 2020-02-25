use super::math::Point;
use super::render::board::RenderBoard;
use super::BoardRef;

pub struct Input<T> {
    meta: Option<T>,
}

impl<T> Input<T> {
    pub fn new() -> Self {
        Self { meta: None }
    }

    pub fn with_meta(meta: T) -> Self {
        Self { meta: Some(meta) }
    }
}

pub trait MouseUp {
    fn mouse_up(&mut self, x: i32, y: i32) -> Result<(), String>;
}

pub struct ClickCell<'a> {
    board: BoardRef,
    board_render: &'a RenderBoard,
}

impl<'a> ClickCell<'a> {
    pub fn new(board: BoardRef, board_render: &'a RenderBoard) -> Self {
        Self {
            board,
            board_render,
        }
    }
}

impl<'a> MouseUp for Input<ClickCell<'a>> {
    fn mouse_up(&mut self, x: i32, y: i32) -> Result<(), String> {
        println!("mouse_up = {:?}", Point::new(x, y));
        let meta = self.meta.as_ref().ok_or_else(|| "missing meta")?;
        match meta.board_render.get_cell_at(x, y) {
            Some(p) => meta.board.borrow_mut().reveal_from(p.x as u32, p.y as u32),
            None => {}
        }
        Ok(())
    }
}
