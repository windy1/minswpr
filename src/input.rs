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

pub struct ClickCell<'a> {
    mouse_pos: Point,
    board: Option<BoardRef>,
    board_render: Option<&'a RenderBoard>,
}

impl<'a> ClickCell<'a> {
    pub fn new() -> Self {
        Self {
            mouse_pos: point!(0, 0),
            board: None,
            board_render: None,
        }
    }

    pub fn mouse_pos(mut self, x: i32, y: i32) -> Self {
        self.mouse_pos = point!(x, y);
        self
    }

    pub fn board(mut self, board: BoardRef) -> Self {
        self.board = Some(board);
        self
    }

    pub fn board_render(mut self, board_render: &'a RenderBoard) -> Self {
        self.board_render = Some(board_render);
        self
    }
}

impl<'a> Execute for Input<ClickCell<'a>> {
    fn execute(&mut self) -> Result<(), String> {
        let meta = self.meta.as_ref().unwrap();
        let Point { x, y } = meta.mouse_pos;
        println!("mouse_up = {:?}", point!(x, y));
        match meta.board_render.unwrap().get_cell_at(x, y) {
            Some(p) => meta
                .board
                .as_ref()
                .unwrap()
                .borrow_mut()
                .reveal_from(p.x, p.y),
            None => {}
        }
        Ok(())
    }
}
