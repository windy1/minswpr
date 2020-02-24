use super::board::Board;
use super::render::board::{CellAttrs, RenderBoard};
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::{self, EventPump, Sdl, VideoSubsystem};
use std::thread;
use std::time::Duration;

pub struct Minswp {
    _sdl: Sdl,
    _video: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl Minswp {
    pub fn new(config: Config) -> Result<Minswp, String> {
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
        Ok(Minswp {
            _sdl: sdl,
            _video: video,
            canvas,
            event_pump,
        })
    }

    pub fn default() -> Result<Minswp, String> {
        Self::new(Config::default())
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.present();

        let mut board_render = RenderBoard::builder()
            .board(Board::default())
            .cell_attrs(CellAttrs::default())
            .build();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

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
}

impl<'a> Config<'a> {
    const DEF_TITLE: &'static str = "minswp";
    const DEF_WIDTH: u32 = 800;
    const DEF_HEIGHT: u32 = 600;

    pub fn new() -> Config<'a> {
        Config {
            title: "",
            width: 0,
            height: 0,
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
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config::new()
            .title(Self::DEF_TITLE)
            .width(Self::DEF_WIDTH)
            .height(Self::DEF_HEIGHT)
    }
}
