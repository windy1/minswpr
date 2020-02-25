use super::Config;
use crate::board::Board;
use crate::input::{ClickCell, Input, MouseUp};
use crate::render::board::{CellAttrs, RenderBoard};
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::{self, EventPump, Sdl, VideoSubsystem};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

pub type BoardRef = Rc<RefCell<Board>>;

pub struct Minswpr {
    _sdl: Sdl,
    _video: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
    board: BoardRef,
    board_render: RenderBoard,
    config: Config,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let canvas = Self::make_canvas(&video, &config.title, config.width, config.height)?;
        let event_pump = sdl.event_pump()?;
        let board = Rc::new(RefCell::new(Board::default()));
        let board_render = Self::make_board_render(&board)?;

        Ok(Self {
            _sdl: sdl,
            _video: video,
            canvas,
            event_pump,
            board,
            board_render,
            config,
        })
    }

    fn make_canvas(
        video: &VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<WindowCanvas, String> {
        Ok(video
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?)
    }

    fn make_board_render(board: &BoardRef) -> Result<RenderBoard, String> {
        Ok(RenderBoard::builder()
            .board(Rc::clone(board))
            .cell_attrs(CellAttrs::default())
            .build()?)
    }

    pub fn default() -> Result<Self, String> {
        Self::new(Config::default())
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.present();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseButtonUp { x, y, .. } => {
                        Input::<ClickCell>::with_meta(ClickCell::new(
                            Rc::clone(&self.board),
                            &self.board_render,
                        ))
                        .mouse_up(x, y)?;
                    }
                    _ => {}
                }
            }

            self.canvas.set_draw_color(self.config.bg_color);
            self.canvas.clear();

            self.board_render.render(&mut self.canvas)?;
            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
