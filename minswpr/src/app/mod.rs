pub(super) mod context;

pub use self::context::*;

use self::{Context, ContextBuilder};
use crate::board::Board;
use crate::config::{self, Config};
use crate::events;
use crate::fonts::Fonts;
use crate::layout::{Layout, RenderRef};
use crate::math::{Dimen, Point};
use crate::render::board::RenderBoard;
use crate::render::control::RenderControlBuilder;
use crate::render::{Render, RenderMut, RenderRect};
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
        let mut fonts = Fonts::new(&self.ttf);
        fonts.load_from_config(&self.config.fonts)?;
        let fonts = Rc::new(fonts);

        let mut ctx = {
            let bc = &self.config.board;
            ContextBuilder::default()
                .config(self.config.clone())
                .game_state(GameState::Ready)
                .board(Self::make_board(bc.dimen, bc.mine_frequency)?)
                .layout(Layout::new(10, color!(red)))
                .build()?
        };

        let components = Self::make_components(&fonts, &ctx)?;
        ctx.layout_mut().insert_all(components);

        let mut canvas =
            Self::make_canvas(&self.video, &self.config.window.title, ctx.layout().dimen())?;

        let layout_pos = point!(0, 0);

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

            ctx.layout_mut().render(&mut canvas, layout_pos)?;

            canvas.present();

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

    fn make_components<'a>(
        fonts: &Rc<Fonts<'a>>,
        context: &Context,
    ) -> Result<Vec<(&'static str, Option<RenderRef<'a>>)>, String> {
        let config = context.config();
        let board = Box::new(RenderBoard::new(
            Rc::clone(fonts),
            Rc::clone(context.board()),
            config.board.cells.clone(),
        ));
        let board_width = board.dimen().width();

        Ok(vec![
            (
                "control",
                Some(Box::new(
                    RenderControlBuilder::default()
                        .fonts(Rc::clone(&fonts))
                        .board_width(board_width)
                        .color(color!(blue))
                        .config(config.control.clone())
                        .build()?,
                )),
            ),
            (
                "spacer",
                Some(Box::new(RenderRect::new(
                    point!(board_width, config.control.spacer_height),
                    color!(red),
                ))),
            ),
            ("board", Some(board)),
        ])
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
