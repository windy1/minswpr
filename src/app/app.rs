use super::Config;
use crate::board::Board;
use crate::fonts::Fonts;
use crate::input::{ClickCell, Execute, Input};
use crate::render::board::{CellAttrs, RenderBoard};
use crate::render::colors;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{self, EventPump, Sdl, VideoSubsystem};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

pub type BoardRef = Rc<RefCell<Board>>;

pub struct Minswpr {
    _sdl: Sdl,
    _video: VideoSubsystem,
    ttf: Sdl2TtfContext,
    config: Config,
    event_pump: EventPump,
    canvas: WindowCanvas,
    board: BoardRef,
    board_render: RenderBoard,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let win = &config.window;
        let bc = &config.board;

        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let canvas = Self::make_canvas(&video, &win.title, win.width, win.height)?;
        let event_pump = sdl.event_pump()?;

        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let board = Self::make_board(bc.width, bc.height, bc.mine_frequency)?;
        let board_render = Self::make_board_render(&board, &config)?;

        let app = Self {
            _sdl: sdl,
            _video: video,
            ttf,
            config,
            event_pump,
            canvas,
            board,
            board_render,
        };

        Ok(app)
    }

    pub fn from_config<P>(fname: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        Self::new(super::read_config(fname)?)
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

    fn make_board(w: usize, h: usize, mf: f64) -> Result<BoardRef, String> {
        Ok(Rc::new(RefCell::new(Board::new(w, h, mf)?)))
    }

    fn make_board_render(board: &BoardRef, c: &Config) -> Result<RenderBoard, String> {
        let c = &c.board.cells;
        let mc = &c.mines;
        Ok(RenderBoard::builder()
            .board(Rc::clone(board))
            .cell_attrs(
                CellAttrs::new()
                    .dimen(c.width, c.height)
                    .color(c.color)
                    .border_width(c.border_width)
                    .border_color(c.border_color)
                    .revealed_color(c.revealed_color)
                    .mine_color(mc.color)
                    .mine_dimen(mc.width, mc.height),
            )
            .build()?)
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.present();

        let mut fonts = Fonts::new(&self.ttf)?;
        for (k, f) in &self.config.fonts {
            fonts.load(k, &f.path, f.pt)?;
        }

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::MouseButtonUp { x, y, .. } => {
                        Input::<ClickCell>::with_meta(
                            ClickCell::new()
                                .mouse_pos(x, y)
                                .board(Rc::clone(&self.board))
                                .board_render(&self.board_render),
                        )
                        .execute()?;
                    }
                    _ => {}
                }
            }

            self.canvas.set_draw_color(self.config.window.bg_color);
            self.canvas.clear();

            self.board_render.render(&mut self.canvas, &fonts)?;

            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
