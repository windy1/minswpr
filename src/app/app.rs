use super::{CellConfig, Config};
use crate::board::Board;
use crate::fonts::Fonts;
use crate::input::{Execute, KeyDown, MouseUpBuilder};
use crate::layout::Layout;
use crate::math::{Dimen, Point};
use crate::render::board::RenderBoard;
use crate::render::Render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{self, EventPump, Sdl, VideoSubsystem};
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

pub type BoardRef = Rc<RefCell<Board>>;

pub struct Context<'a> {
    config: Rc<Config>,
    // event_pump: EventPump,
    canvas: WindowCanvas,
    // fonts: Fonts<'ttf>,
    board: BoardRef,
    game_state: GameState,
    layout: Layout<'a>,
    // renderer: Option<Renderer<'a>>,
}

// pub struct Renderer<'a> {
//     layout: Layout<'a>,
// }

impl<'a> Context<'a> {
    // pub fn init(&mut self) {
    //
    // }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn board(&self) -> &BoardRef {
        &self.board
    }

    pub fn get_cell_at(&self, x: i32, y: i32, pos: &Point) -> Option<Point<u32>> {
        let base_dimen = &self.layout.get("board").unwrap().dimen();
        let min_x = pos.x;
        let min_y = pos.y;
        let max_x = min_x + base_dimen.width() as i32;
        let max_y = min_y + base_dimen.height() as i32;

        if x < min_x || x > max_x || y < min_y || y > max_y {
            return None;
        }

        let cell_config = &self.config.board.cells;
        let cell_dimen = &cell_config.dimen.as_i32();
        let border_width = cell_config.border_width as i32;
        let board = self.board.borrow();
        let screen_pos = point!(x, y);

        let mut c = (screen_pos - *pos) / (*cell_dimen + (border_width, border_width));
        c.x = cmp::min(c.x, board.width() as i32 - 1);
        c.y = cmp::min(c.y, board.height() as i32 - 1);

        Some(point!(c.x as u32, c.y as u32))
    }
}

struct EventHandler;

impl EventHandler {
    fn handle_event(
        &self,
        context: &Context,
        event: Event,
        // game_state: GameState,
        // board: &BoardRef,
        // board_render: &RenderBoard,
    ) -> Result<GameState, String> {
        match event {
            Event::Quit { .. } => Ok(GameState::Quit),
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => self.handle_mouse_up(context, mouse_btn, x, y),
            Event::KeyDown { keycode, .. } => match keycode {
                Some(k) => self.handle_key_down(context, k),
                None => Ok(*context.game_state()),
            },
            _ => Ok(*context.game_state()),
        }
    }

    fn handle_mouse_up(
        &self,
        context: &Context,
        mouse_btn: MouseButton,
        x: i32,
        y: i32,
        // board: &BoardRef,
        // board_render: &RenderBoard,
        // game_state: GameState,
    ) -> Result<GameState, String> {
        MouseUpBuilder::default()
            .mouse_btn(mouse_btn)
            .mouse_pos(Point::new(x, y))
            // .board(Some(Rc::clone(board)))
            // .board_render(Some(board_render))
            // .game_state(game_state)
            .context(Some(context))
            .build()?
            .execute()
    }

    fn handle_key_down(&self, context: &Context, keycode: Keycode) -> Result<GameState, String> {
        KeyDown::new(keycode, *context.game_state()).execute()
    }
}

pub struct Minswpr {
    config: Rc<Config>,
    sdl: Sdl,
    video: VideoSubsystem,
    ttf: Sdl2TtfContext,
    event_pump: EventPump,
    // fonts: Rc<Fonts<'ttf>>,
}

impl Minswpr {
    pub fn new(config: Config) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let event_pump = sdl.event_pump()?;
        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

        Ok(Self {
            sdl,
            video,
            ttf,
            event_pump,
            // fonts,
            config: Rc::new(config),
        })
    }

    pub fn from_config<P: AsRef<Path>>(fname: P) -> Result<Self, String> {
        Self::new(super::read_config(fname)?)
    }

    pub fn start(&mut self) -> Result<(), String> {
        let win = &self.config.window;
        let bc = &self.config.board;

        let config = Rc::clone(&self.config);
        let canvas = Self::make_canvas(&self.video, &win.title, win.dimen)?;
        let board = Self::make_board(bc.dimen, bc.mine_frequency)?;
        let game_state = GameState::Unknown;

        let mut fonts = Fonts::new(&self.ttf)?;
        for (k, f) in &config.fonts {
            fonts.load(k, &f.path, f.pt)?;
        }
        let fonts = Rc::new(fonts);

        let layout = Layout::new();

        let mut ctx = Context {
            config,
            canvas,
            // event_pump,
            // fonts,
            board,
            game_state,
            layout,
            // renderer: None,
        };

        // let renderer = Renderer { layout };
        // ctx.renderer = Some(renderer);

        let board_render = RenderBoard::new(
            Rc::clone(&fonts),
            Rc::clone(&ctx.board),
            &self.config.board.cells,
        );
        // layout.insert("board", Box::new(board_render));
        // ctx.layout.test = Some(board_render);

        // let bc = &self.config.board;

        ctx.game_state = GameState::Ready;
        ctx.canvas.clear();
        ctx.canvas.present();

        'main: loop {
            for event in self.event_pump.poll_iter() {
                // ctx.handle_event(event)?;
                let e = EventHandler {};
                e.handle_event(&ctx, event)?;
            }

            ctx.game_state = match ctx.game_state {
                GameState::Quit => break 'main,
                GameState::Reset => {
                    let bc = &self.config.board;
                    let bd = &bc.dimen;
                    ctx.board
                        .replace(Board::new(bd.width(), bd.height(), bc.mine_frequency)?);
                    GameState::Ready
                }
                _ => ctx.game_state,
            };

            ctx.canvas.set_draw_color(self.config.window.bg_color);
            ctx.canvas.clear();

            // board_render.render(&mut self.canvas, &point!(10, 10))?;
            ctx.layout.render(&mut ctx.canvas, &point!(10, 10))?;

            ctx.canvas.present();

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

    // fn make_board_render<'b, 'ttf>(
    //     fonts: &'ttf Fonts<'ttf>,
    //     board: &BoardRef,
    //     c: &'b Config,
    // ) -> Result<RenderBoard<'b, 'ttf>, String> {
    //     Ok(RenderBoard::new(&fonts, Rc::clone(&board), &c.board.cells))
    // }
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
