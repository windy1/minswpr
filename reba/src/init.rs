use crate::RebaResult;
use crate::fonts::{Fonts, FontData};
use crate::context::ContextBuilder;
use crate::draw::DrawContext;
use crate::app::App;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{self, EventPump, VideoSubsystem};
use sdl2::render::WindowCanvas;
use std::thread;
use std::time::Duration;

pub struct Reba {
    ttf: Sdl2TtfContext,
    video: VideoSubsystem,
    event_pump: EventPump,
    app: Box<dyn App>,
}

impl Reba {
    pub fn mount(app: Box<dyn App>) -> RebaResult<Self> {
        let sdl = sdl2::init()?;
        Ok(Self {
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            video: sdl.video()?,
            event_pump: sdl.event_pump()?,
            app
        })
    }

    pub fn start(&mut self) -> RebaResult {
        let ctx = ContextBuilder::default()
            .build()?;

        let mut fonts = Fonts::new(&self.ttf);

        let mut draw = {
            let canvas = self.make_canvas()?;
            let textures = canvas.texture_creator();
            DrawContext::new(&ctx, canvas, &mut fonts, textures)
        };

        {
            let c = draw.canvas_mut();
            c.clear();
            c.present();
        }

        self.app.on_start();

        loop {
            for font in self.app.font_bus_mut() {
                draw.load_font(font);
            }

            {
                let c = draw.canvas_mut();
                c.set_draw_color(self.app.window().background_color());
            }

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn make_canvas(&self) -> RebaResult<WindowCanvas> {
        let w = self.app.window();
        let d = w.dimen();
        Ok(self.video
            .window(w.title(), d.width(), d.height())
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?)
    }
}
