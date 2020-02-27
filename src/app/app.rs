use super::Config;
use crate::board::Board;
use crate::fonts::Fonts;
use crate::input::{Execute, Input, KeyDown, MouseUp};
use crate::math::Dimen;
use crate::render::board::RenderBoard;
use crate::render::Render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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
    _sdl: Sdl,
    _video: VideoSubsystem,
    ttf: Sdl2TtfContext,
    config: Config,
    event_pump: EventPump,
    canvas: WindowCanvas,
    board: BoardRef,
    game_state: GameState,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let win = &config.window;
        let bc = &config.board;

        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let canvas = Self::make_canvas(&video, &win.title, win.dimen)?;
        let event_pump = sdl.event_pump()?;
        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let board = Self::make_board(bc.dimen, bc.mine_frequency)?;
        let game_state = GameState::Unknown;

        let app = Self {
            _sdl: sdl,
            _video: video,
            ttf,
            config,
            event_pump,
            canvas,
            board,
            game_state,
        };

        Ok(app)
    }

    pub fn from_config<P: AsRef<Path>>(fname: P) -> Result<Self, String> {
        Self::new(super::read_config(fname)?)
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

    fn make_board_render<'a>(
        fonts: &'a Fonts<'a>,
        board: &BoardRef,
        c: &'a Config,
    ) -> Result<RenderBoard<'a>, String> {
        Ok(RenderBoard::new(&fonts, Rc::clone(&board), &c.board.cells))
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut fonts = Fonts::new(&self.ttf)?;
        for (k, f) in &self.config.fonts {
            fonts.load(k, &f.path, f.pt)?;
        }

        let board_render = Self::make_board_render(&fonts, &self.board, &self.config)?;

        self.game_state = GameState::Ready;

        self.canvas.clear();
        self.canvas.present();

        'main: loop {
            for event in self.event_pump.poll_iter() {
                self.game_state = match event {
                    Event::Quit { .. } => break 'main,
                    Event::MouseButtonUp {
                        mouse_btn, x, y, ..
                    } => Self::handle_mouse_up(
                        mouse_btn,
                        x,
                        y,
                        &self.board,
                        &board_render,
                        self.game_state,
                    )?,
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(k) => Self::handle_key_down(k, self.game_state)?,
                        None => self.game_state,
                    },
                    _ => self.game_state,
                }
            }

            self.game_state = match self.game_state {
                GameState::Reset => {
                    let bc = &self.config.board;
                    let bd = &bc.dimen;
                    self.board
                        .replace(Board::new(bd.width(), bd.height(), bc.mine_frequency)?);
                    GameState::Ready
                }
                _ => self.game_state
            };

            self.canvas.set_draw_color(self.config.window.bg_color);
            self.canvas.clear();

            board_render.render(&mut self.canvas)?;

            self.canvas.present();

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn handle_mouse_up(
        mouse_btn: MouseButton,
        x: i32,
        y: i32,
        board: &BoardRef,
        board_render: &RenderBoard,
        game_state: GameState,
    ) -> Result<GameState, String> {
        Input::with_meta(
            MouseUp::new()
                .mouse_btn(mouse_btn)
                .mouse_pos(x, y)
                .board(Rc::clone(board))
                .board_render(board_render)
                .game_state(game_state),
        )
        .execute()
    }

    fn handle_key_down(keycode: Keycode, game_state: GameState) -> Result<GameState, String> {
        Input::with_meta(KeyDown::new(keycode, game_state)).execute()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Unknown,
    Ready,
    InProgress,
    Over,
    Reset,
}
