pub(super) mod context;

pub use self::context::*;
use crate::layout::Layout;

use self::ContextBuilder;
use crate::board::Board;
use crate::config::{self, Config};
use crate::events;
use crate::fonts::Fonts;
use crate::layout::LayoutBuilder;
use crate::math::{Dimen, Point};
use crate::render::board::RenderBoard;
use crate::render::control;
use crate::render::{CanvasRef, DrawContext, Render, RenderRect};
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
    ttf: Sdl2TtfContext,
    video: VideoSubsystem,
    event_pump: EventPump,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        Ok(Self {
            config,
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            video: sdl.video()?,
            event_pump: sdl.event_pump()?,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        let board = self.make_board()?;

        let mut ctx = ContextBuilder::default()
            .config(self.config.clone())
            .game_state(GameState::Ready)
            .board(Rc::clone(&board))
            .layout(self.make_layout(&board)?)
            .build()?;

        lazy_static! {
            static ref LAYOUT_POS: Point = point!(0, 0);
        }

        let fonts = {
            let mut f = Fonts::new(&self.ttf);
            f.load_from_config(&self.config.fonts)?;
            f
        };

        let draw = DrawContext::new(self.make_canvas(ctx.layout().dimen())?, &fonts);
        draw.with_canvas(|mut c| {
            c.clear();
            c.present();
        });

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

            draw.with_canvas(|mut c| {
                c.set_draw_color(self.config.window.bg_color);
                c.clear();
            });

            ctx.layout_mut().render(&draw, *LAYOUT_POS)?;
            draw.canvas().present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn make_board(&self) -> Result<BoardRef, String> {
        let bc = &self.config.board;
        let Dimen { x: w, y: h } = bc.dimen;
        Ok(Rc::new(RefCell::new(Board::new(w, h, bc.mine_frequency)?)))
    }

    fn make_layout(&self, board: &BoardRef) -> Result<Layout, String> {
        let lc = &self.config.layout;
        let mut layout = LayoutBuilder::default()
            .color(Some(lc.color))
            .padding(lc.padding)
            .build()?;

        let cc = &self.config.control;

        let board = Box::new(RenderBoard::new(
            Rc::clone(&board),
            self.config.board.cells.clone(),
        ));
        let board_width = board.dimen().width();

        layout.insert_all(vec![
            ("control", Box::new(control::make_layout(&cc, board_width))),
            (
                "spacer",
                Box::new(RenderRect::new(
                    point!(board_width, cc.spacer_height),
                    cc.spacer_color,
                )),
            ),
            ("board", board),
        ]);

        Ok(layout)
    }

    fn make_canvas(&self, dimen: Dimen) -> Result<CanvasRef, String> {
        Ok(Rc::new(RefCell::new(
            self.video
                .window(&self.config.window.title, dimen.width(), dimen.height())
                .position_centered()
                .build()
                .map_err(|e| e.to_string())?
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())?,
        )))
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
