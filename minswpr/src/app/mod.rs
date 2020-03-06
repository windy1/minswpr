pub(super) mod context;

pub use self::context::*;

use self::{Context, ContextBuilder};
use super::input;
use super::input::events::{MouseDownEvent, MouseMoveEvent, MouseUpEvent};
use crate::board::{Board, CellFlags};
use crate::config::{self, Config};
use crate::control::{Button, Stopwatch};
use crate::draw::board::DrawBoard;
use crate::draw::{CanvasRef, Draw, DrawContext, DrawRect};
use crate::fonts::Fonts;
use crate::layout::control::ControlLayoutBuilder;
use crate::layout::{Element, ElementBuilder, Layout, LayoutBuilder};
use crate::math::{Dimen, Point};
use crate::MsResult;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{self, EventPump, VideoSubsystem};
use std::cell::RefCell;
use std::convert::TryInto;
use std::path::Path;
use std::process;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

/// Helper type for a `Board` reference
pub type BoardRef = Rc<RefCell<Board>>;
/// Helper type for a `Stopwatch` reference
pub type StopwatchRef = Rc<RefCell<Stopwatch>>;
// Helper type for a `Button` reference
pub type ButtonRef = Rc<RefCell<Button>>;

/// The application root
pub struct Minswpr {
    config: Config,
    ttf: Sdl2TtfContext,
    video: VideoSubsystem,
    event_pump: EventPump,
}

impl Minswpr {
    /// Initializes SDL2 and creates a new `Minswpr` instance from the specified
    /// Config.
    pub fn new(config: Config) -> MsResult<Self> {
        let sdl = sdl2::init()?;
        Ok(Self {
            config,
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            video: sdl.video()?,
            event_pump: sdl.event_pump()?,
        })
    }

    /// Starts the game. Returns an `Err` if an error occurs during
    /// initialization or the main game loop.
    pub fn start(&mut self) -> MsResult {
        let mut ctx = ContextBuilder::default()
            .config(self.config.clone())
            .game_state(GameState::Ready)
            .board(Rc::new(RefCell::new(self.make_board()?)))
            .stopwatch(Rc::new(RefCell::new(Stopwatch::new())))
            .build()?;

        ctx.buttons_mut()
            .insert("reset", Rc::new(RefCell::new(Button::new())));
        ctx.set_layout(self.make_layout(&ctx)?);

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

        let mut last_game_state = GameState::Unknown; // debug

        loop {
            // handle input events
            for event in self.event_pump.poll_iter() {
                ctx.set_game_state(self::handle_event(&ctx, event));
            }

            // do some post-processing on the game state produced by the input events
            self.handle_game_state(&mut ctx)?;

            // initialize the canvas
            draw.with_canvas(|mut c| {
                c.set_draw_color(self.config.window.bg_color);
                c.clear();
            });

            ctx.layout_mut().draw(&draw, *LAYOUT_POS)?;
            draw.canvas().present();

            // debug
            let game_state = ctx.game_state();
            if game_state != last_game_state {
                println!("game_state = {:?}", game_state);
                last_game_state = game_state;
            }
            // debug

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn handle_game_state(&self, ctx: &mut Context) -> MsResult {
        ctx.set_game_state(match ctx.game_state() {
            GameState::Quit => {
                process::exit(0);
            }
            GameState::Reset => {
                // reset the board and the stopwatch
                let bc = &self.config.board;
                let bd = &bc.dimen;
                ctx.board()
                    .replace(Board::new(bd.width(), bd.height(), bc.num_mines)?);
                ctx.stopwatch().borrow_mut().reset();
                GameState::Ready
            }
            GameState::Start => {
                ctx.stopwatch().borrow_mut().start();
                GameState::Started
            }
            GameState::Over => {
                ctx.stopwatch().borrow_mut().stop();
                GameState::Over
            }
            _ => ctx.game_state(),
        });

        Ok(())
    }

    fn make_board(&self) -> MsResult<Board> {
        let bc = &self.config.board;
        let Dimen { x: w, y: h } = bc.dimen;
        Board::new(w, h, bc.num_mines)
    }

    fn make_layout(&self, ctx: &Context) -> MsResult<Layout> {
        let lc = &self.config.layout;
        let mut layout = LayoutBuilder::default()
            .color(lc.color)
            .padding(lc.padding)
            .guides(lc.guides)
            .build()?;

        let cc = &self.config.control;

        let board_draw = Box::new(DrawBoard::new(
            Rc::clone(ctx.board()),
            self.config.board.cells.clone(),
        ));
        let board_width = board_draw.dimen().width();

        layout.insert_all(vec![
            (
                "control",
                ElementBuilder::default()
                    .draw_ref(Box::new(
                        (ControlLayoutBuilder::default()
                            .config(&cc)
                            .board_width(board_width)
                            .board(ctx.board())
                            .stopwatch(ctx.stopwatch())
                            .reset_button(&ctx.buttons()["reset"])
                            .build()?
                            .try_into()?): Layout,
                    ))
                    .mouse_up(input::defer_mouse("control", &input::mouse_up))
                    .mouse_down(input::defer_mouse("control", &input::mouse_down))
                    .mouse_move(input::defer_mouse("control", &input::mouse_move))
                    .build()?,
            ),
            (
                "spacer",
                Element::new(Box::new(DrawRect::new(
                    point!(board_width, cc.spacer_height),
                    cc.spacer_color,
                ))),
            ),
            (
                "board",
                ElementBuilder::default()
                    .draw_ref(board_draw)
                    .mouse_up(Box::new(input::board::on_click_board))
                    .mouse_move(Box::new(input::board::on_mouse_move_board))
                    .mouse_down(Box::new(input::board::on_mouse_down_board))
                    .mouse_leave(Box::new(input::board::on_mouse_leave_board))
                    .build()?,
            ),
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

/// Represents different states the application can be in
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameState {
    Unknown,
    Ready,
    Start,
    Started,
    Over,
    Reset,
    Quit,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Unknown
    }
}

fn handle_event(ctx: &Context, event: Event) -> GameState {
    match event {
        Event::Quit { .. } => GameState::Quit,
        Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => self::handle_mouse_up(ctx, mouse_btn, x, y),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => self::handle_mouse_down(ctx, mouse_btn, x, y),
        Event::MouseMotion {
            mousestate, x, y, ..
        } => self::handle_mouse_motion(ctx, mousestate, x, y),
        Event::KeyDown { keycode, .. } => match keycode {
            Some(k) => self::handle_key_down(ctx, k),
            None => ctx.game_state(),
        },
        Event::Window { win_event, .. } => match win_event {
            WindowEvent::Leave => {
                ctx.board().borrow_mut().clear_all(CellFlags::PRESSED);
                ctx.game_state()
            }
            _ => ctx.game_state(),
        },
        _ => ctx.game_state(),
    }
}

fn handle_mouse_up(ctx: &Context, mouse_btn: MouseButton, x: i32, y: i32) -> GameState {
    for button in ctx.buttons().values() {
        button.borrow_mut().set_released(true);
    }

    ctx.layout().defer_mouse_event(
        ctx,
        MouseUpEvent::new(mouse_btn, point!(x, y)),
        &input::mouse_up,
    )
}

fn handle_mouse_down(ctx: &Context, mouse_btn: MouseButton, x: i32, y: i32) -> GameState {
    ctx.layout().defer_mouse_event(
        ctx,
        MouseDownEvent::new(mouse_btn, point!(x, y)),
        &input::mouse_down,
    )
}

fn handle_mouse_motion(ctx: &Context, mouse_state: MouseState, x: i32, y: i32) -> GameState {
    ctx.layout().defer_mouse_event(
        ctx,
        MouseMoveEvent::new(mouse_state, point!(x, y)),
        &input::mouse_move,
    )
}

fn handle_key_down(ctx: &Context, keycode: Keycode) -> GameState {
    match keycode {
        Keycode::F2 => GameState::Reset,
        _ => ctx.game_state(),
    }
}
