use super::Config;
use crate::board::Board;
use crate::input::{ClickCell, Execute, Input};
use crate::render::board::{CellAttrs, RenderBoard};
use crate::render::colors;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
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
    config: Config,
    ttf: Sdl2TtfContext,
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
        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let event_pump = sdl.event_pump()?;

        let board = Self::make_board(bc.width, bc.height, bc.mine_frequency)?;
        let board_render = Self::make_board_render(&board, &config)?;

        Ok(Self {
            _sdl: sdl,
            _video: video,
            config,
            ttf,
            event_pump,
            canvas,
            board,
            board_render,
        })
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

    fn load_text<'a, T>(&self, textures: &'a TextureCreator<T>) -> Result<Texture<'a>, String> {
        let path = Path::new("/usr/share/fonts/truetype/ubuntu/Ubuntu-M.ttf");

        let mut font = self.ttf.load_font(path, 128)?;
        font.set_style(FontStyle::BOLD);

        let surface = font
            .render("Hello, world!")
            .blended(colors::GREEN)
            .map_err(|e| e.to_string())?;

        let texture = textures
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        Ok(texture)
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.present();

        let textures = self.canvas.texture_creator();

        let text_test = self.load_text(&textures)?;
        let tq = text_test.query();

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

            self.board_render.render(&mut self.canvas)?;

            self.canvas.copy(
                &text_test,
                None,
                Some(Rect::new(10, 10, tq.width, tq.height)),
            )?;

            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
