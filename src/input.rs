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

pub trait Execute {
    fn execute(&mut self) -> Result<(), String>;
}

pub trait MouseUp {
    fn mouse_up(&mut self, x: i32, y: i32) -> Result<(), String>;
}

pub struct ClickCell<'a> {
    mouse_pos: Point,
    board: BoardRef,
    board_render: &'a RenderBoard,
}

impl<'a> ClickCell<'a> {
    pub fn new(x: i32, y: i32, board: BoardRef, board_render: &'a RenderBoard) -> Self {
        Self {
            mouse_pos: Point::new(x, y),
            board,
            board_render,
        }
    }
}

impl<'a> Execute for Input<ClickCell<'a>> {
    fn execute(&mut self) -> Result<(), String> {
        let meta = self.meta.as_ref().ok_or_else(|| "missing meta")?;
        let Point { x, y } = meta.mouse_pos;
        println!("mouse_up = {:?}", Point::new(x, y));
        match meta.board_render.get_cell_at(x, y) {
            Some(p) => meta.board.borrow_mut().reveal_from(p.x as u32, p.y as u32),
            None => {}
        }
        Ok(())
    }
}
