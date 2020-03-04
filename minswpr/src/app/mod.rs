pub(super) mod context;

pub use self::context::*;

use self::{Context, ContextBuilder};
use super::input::{Execute, KeyDown, MouseUp};
use crate::board::Board;
use crate::config::{self, Config};
use crate::draw::board::DrawBoard;
use crate::draw::control;
use crate::draw::{CanvasRef, Draw, DrawContext, DrawRect};
use crate::fonts::Fonts;
use crate::layout::{Layout, LayoutBuilder};
use crate::math::{Dimen, Point};
use crate::MsResult;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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
    pub fn new(config: Config) -> MsResult<Self> {
        let sdl = sdl2::init()?;
        Ok(Self {
            config,
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            video: sdl.video()?,
            event_pump: sdl.event_pump()?,
        })
    }

    pub fn start(&mut self) -> MsResult {
        let board = self.make_board()?;

        let mut ctx = ContextBuilder::default()
            .config(self.config.clone())
            .game_state(GameState::Ready)
            .board(Rc::clone(&board))
            .layout(self.make_layout(&board)?)
            .build()?;

        let fonts = Fonts::from_config(&self.config.fonts, &self.ttf)?;

        let draw = {
            let canvas = self.make_canvas(ctx.layout().dimen())?;
            let textures = canvas.borrow().texture_creator();
            DrawContext::new(canvas, &fonts, textures)
        };

        draw.with_canvas(|mut c| {
            c.clear();
            c.present();
        });

        lazy_static! {
            static ref LAYOUT_POS: Point = point!(0, 0);
        }

        'main: loop {
            for event in self.event_pump.poll_iter() {
                ctx.set_game_state(self::handle_event(&ctx, event)?);
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

            ctx.layout_mut().draw(&draw, *LAYOUT_POS)?;
            draw.canvas().present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn make_board(&self) -> MsResult<BoardRef> {
        let bc = &self.config.board;
        let Dimen { x: w, y: h } = bc.dimen;
        Ok(Rc::new(RefCell::new(Board::new(w, h, bc.mine_frequency)?)))
    }

    fn make_layout(&self, board: &BoardRef) -> MsResult<Layout> {
        let lc = &self.config.layout;
        let mut layout = LayoutBuilder::default()
            .color(Some(lc.color))
            .padding(lc.padding)
            .build()?;

        let cc = &self.config.control;

        let board_draw = Box::new(DrawBoard::new(
            Rc::clone(&board),
            self.config.board.cells.clone(),
        ));
        let board_width = board_draw.dimen().width();

        layout.insert_all(vec![
            (
                "control",
                Box::new(control::make_layout(&cc, board_width, &board)?),
            ),
            (
                "spacer",
                Box::new(DrawRect::new(
                    point!(board_width, cc.spacer_height),
                    cc.spacer_color,
                )),
            ),
            ("board", board_draw),
        ]);

        Ok(layout)
    }

    fn make_canvas(&self, dimen: Dimen) -> MsResult<CanvasRef> {
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

pub fn handle_event(context: &Context, event: Event) -> MsResult<GameState> {
    match event {
        Event::Quit { .. } => Ok(GameState::Quit),
        Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => self::handle_mouse_up(context, mouse_btn, x, y),
        Event::KeyDown { keycode, .. } => match keycode {
            Some(k) => self::handle_key_down(context, k),
            None => Ok(context.game_state()),
        },
        _ => Ok(context.game_state()),
    }
}

fn handle_mouse_up(
    context: &Context,
    mouse_btn: MouseButton,
    x: i32,
    y: i32,
) -> MsResult<GameState> {
    MouseUp::new(mouse_btn, point!(x, y), context).execute()
}

fn handle_key_down(context: &Context, keycode: Keycode) -> MsResult<GameState> {
    KeyDown::new(keycode, context).execute()
}
