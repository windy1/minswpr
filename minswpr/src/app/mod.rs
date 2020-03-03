pub(super) mod context;

pub use self::context::*;

use self::ContextBuilder;
use crate::board::Board;
use crate::config::{self, Config};
use crate::events;
use crate::fonts::Fonts;
use crate::layout::LayoutBuilder;
use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{self, EventPump, VideoSubsystem};
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
    video: VideoSubsystem,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        Ok(Self {
            config,
            event_pump: sdl.event_pump()?,
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            video: sdl.video()?,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        let fonts = {
            let mut f = Fonts::new(&self.ttf);
            f.load_from_config(&self.config.fonts)?;
            Rc::new(f)
        };

        let lc = &self.config.layout;

        let mut ctx = ContextBuilder::default()
            .config(self.config.clone())
            .game_state(GameState::Ready)
            .board(self.make_board()?)
            .layout(
                LayoutBuilder::default()
                    .color(Some(lc.color))
                    .padding(lc.padding)
                    .build()?,
            )
            .build()?;

        lazy_static! {
            static ref LAYOUT_POS: Point = point!(0, 0);
        }

        ctx.make_components(&fonts);

        let mut canvas = self.make_canvas(ctx.layout().dimen())?;
        canvas.clear();
        canvas.present();

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
                _ => ctx.game_state(),
            };

            ctx.set_game_state(game_state);

            canvas.set_draw_color(self.config.window.bg_color);
            canvas.clear();
            ctx.layout_mut().render(&mut canvas, *LAYOUT_POS)?;
            canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn make_canvas(&self, dimen: Dimen) -> Result<WindowCanvas, String> {
        Ok(self
            .video
            .window(&self.config.window.title, dimen.width(), dimen.height())
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?)
    }

    fn make_board(&self) -> Result<BoardRef, String> {
        let bc = &self.config.board;
        let Dimen { x: w, y: h } = bc.dimen;
        Ok(Rc::new(RefCell::new(Board::new(w, h, bc.mine_frequency)?)))
    }
}

impl<P> From<P> for Minswpr
where
    P: AsRef<Path>,
{
    fn from(p: P) -> Self {
        // https://github.com/rust-lang/rust/issues/50133
        let config = config::read_config(p)
            .map_err(|e| format!("could not load configuration file: `{}`", e))
            .unwrap();
        Self::new(config).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Unknown,
    Ready,
    Over,
    Reset,
    Quit,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Unknown
    }
}
