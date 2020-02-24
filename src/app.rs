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
    pub fn new() -> Result<Minswp, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let canvas = video
            .window("minswp", 800, 600)
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
