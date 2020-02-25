use super::board::Board;
use super::input::{ClickCell, Input, MouseUp};
use super::render::board::{CellAttrs, RenderBoard};
use super::render::colors;
use sdl2::event::Event;
use sdl2::pixels::Color;
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
    bg_color: Color,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let canvas = video
            .window(config.title, config.width, config.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        let event_pump = sdl.event_pump()?;

        Ok(Self {
            _sdl: sdl,
            _video: video,
            canvas,
            event_pump,
            bg_color: config.bg_color,
        })
    }

    pub fn default() -> Result<Self, String> {
        Self::new(Config::default())
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.present();

        let board = Rc::new(RefCell::new(Board::default()));
        let mut board_render = RenderBoard::builder()
            .board(Rc::clone(&board))
            .cell_attrs(CellAttrs::default())
            .build()?;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseButtonUp { x, y, .. } => {
                        Input::<ClickCell>::with_meta(ClickCell::new(
                            Rc::clone(&board),
                            &board_render,
                        ))
                        .mouse_up(x, y)?;
                    }
                    _ => {}
                }
            }

            self.canvas.set_draw_color(self.bg_color);
            self.canvas.clear();

            board_render.render(&mut self.canvas)?;
            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}

pub struct Config<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    bg_color: Color,
}

impl<'a> Config<'a> {
    const DEF_TITLE: &'static str = "minswpr";
    const DEF_WIDTH: u32 = 800;
    const DEF_HEIGHT: u32 = 600;
    const DEF_BG_COLOR: Color = colors::BLACK;

    pub fn new() -> Config<'a> {
        Self {
            title: "",
            width: 0,
            height: 0,
            bg_color: colors::BLACK,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn bg_color(mut self, bg_color: Color) -> Self {
        self.bg_color = bg_color;
        self
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config::new()
            .title(Self::DEF_TITLE)
            .width(Self::DEF_WIDTH)
            .height(Self::DEF_HEIGHT)
            .bg_color(Self::DEF_BG_COLOR)
    }
}
