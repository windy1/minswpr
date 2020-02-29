use super::{Config, ContextBuilder};
use crate::board::Board;
use crate::events;
use crate::fonts::Fonts;
use crate::layout::Layout;
use crate::math::{Dimen, Point};
use crate::render::board::RenderBoard;
use crate::render::Render;
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
    config: Config,
    event_pump: EventPump,
    ttf: Sdl2TtfContext,
    canvas: WindowCanvas,
    _sdl: Sdl,
    _video: VideoSubsystem,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let win = &config.window;

        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let event_pump = sdl.event_pump()?;
        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let canvas = Self::make_canvas(&video, &win.title, win.dimen)?;

        Ok(Self {
            config,
            event_pump,
            ttf,
            canvas,
            _sdl: sdl,
            _video: video,
        })
    }

    pub fn from_config<P: AsRef<Path>>(fname: P) -> Result<Self, String> {
        Self::new(super::read_config(fname)?)
    }

    pub fn start(&mut self) -> Result<(), String> {
        let bc = &self.config.board;

        let mut fonts = Fonts::new(&self.ttf)?;
        fonts.load_from_config(&self.config.fonts)?;
        let fonts = Rc::new(fonts);

        let board = Self::make_board(bc.dimen, bc.mine_frequency)?;

        let mut layout = Layout::new();
        let board_render = RenderBoard::new(
            Rc::clone(&fonts),
            Rc::clone(&board),
            self.config.board.cells.clone(),
        );
        layout.insert("board", Box::new(board_render));

        let mut ctx = ContextBuilder::default()
            .config(self.config.clone())
            .game_state(GameState::Ready)
            .board(board)
            .layout(layout)
            .build()?;

        self.canvas.clear();
        self.canvas.present();

        'main: loop {
            for event in self.event_pump.poll_iter() {
                ctx.set_game_state(events::handle_event(&ctx, event)?);
            }

            let game_state = match ctx.game_state() {
                GameState::Quit => break 'main,
                GameState::Reset => {
                    let bc = &self.config.board;
                    let bd = &bc.dimen;
                    ctx.board()
                        .replace(Board::new(bd.width(), bd.height(), bc.mine_frequency)?);
                    GameState::Ready
                }
                _ => *ctx.game_state(),
            };

            ctx.set_game_state(game_state);

            self.canvas.set_draw_color(self.config.window.bg_color);
            self.canvas.clear();

            ctx.layout().render(&mut self.canvas, &point!(10, 10))?;

            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn make_canvas(
        video: &VideoSubsystem,
        title: &str,
        dimen: Dimen,
    ) -> Result<WindowCanvas, String> {
        Ok(video
            .window(title, dimen.width(), dimen.height())
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?)
    }

    fn make_board(Dimen { x: w, y: h }: Dimen<usize>, mf: f64) -> Result<BoardRef, String> {
        Ok(Rc::new(RefCell::new(Board::new(w, h, mf)?)))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Unknown,
    Ready,
    InProgress,
    Over,
    Reset,
    Quit,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Unknown
    }
}
