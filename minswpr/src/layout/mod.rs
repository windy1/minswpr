use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use self::Orientation::*;

pub type RenderRef<'a> = Box<dyn Render + 'a>;
pub type ComponentMap<'a> = HashMap<&'static str, Component<'a>>;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Layout<'a> {
    #[builder(setter(skip))]
    components: ComponentMap<'a>,
    #[builder(default)]
    color: Option<Color>,
    #[builder(default)]
    padding: u32,
    #[builder(default)]
    orientation: Orientation,
    #[builder(default)]
    guides: bool,
}

impl<'a> Layout<'a> {
    pub fn components(&self) -> &ComponentMap {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut ComponentMap<'a> {
        &mut self.components
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn insert(&mut self, key: &'static str, order: i32, component: RenderRef<'a>) {
        self.components_mut()
            .insert(key, Component::new(key, order, component));
    }

    pub fn insert_all(&mut self, mut components: Vec<(&'static str, RenderRef<'a>)>) {
        for (i, c) in components.drain(..).enumerate() {
            self.insert(c.0, i as i32, c.1);
        }
    }

    pub fn get(&self, key: &'static str) -> Result<&Component, String> {
        self.components()
            .get(key)
            .ok_or_else(|| format!("missing required layout component `{}`", key))
    }

    pub fn get_at(&self, x: i32, y: i32) -> Option<&Component> {
        for component in self.components().values() {
            let Point { x: min_x, y: min_y } = component.pos;
            let cd = component.render.dimen();
            let max_x = min_x + cd.width() as i32;
            let max_y = min_y + cd.height() as i32;

            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                return Some(component);
            }
        }

        None
    }

    pub fn draw_guides(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        let Dimen { x: w, y: h } = self.dimen();
        render_rect!(
            point!(1, h),
            color!(magenta),
            canvas,
            pos + point!(w / 2, 0).as_i32()
        )?;
        render_rect!(
            point!(w, 1),
            color!(magenta),
            canvas,
            pos + point!(0, h / 2).as_i32()
        )
    }
}

impl Render for Layout<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        if let Some(c) = self.color {
            render_rect!(self.dimen(), c, canvas, pos)?;
        }

        let orien = self.orientation;
        let padding = self.padding;
        let mut cur = pos + point!(padding, padding).as_i32();
        let mut components = self.components.values_mut().collect::<Vec<_>>();

        components.sort();

        for component in components {
            let r = &mut component.render;
            let m = r.margins();

            component.pos = cur + point!(m.left, m.top).as_i32();
            r.render(canvas, component.pos)?;

            let d = r.dimen();

            cur += match &orien {
                Vertical => point!(0, d.height() + m.bottom + m.top).as_i32(),
                Horizontal => point!(d.width() + m.right + m.left, 0).as_i32(),
            };
        }

        if self.guides {
            self.draw_guides(canvas, pos)
        } else {
            Ok(())
        }
    }

    fn dimen(&self) -> Dimen {
        let padding = self.padding;
        let values = || self.components.values();

        let margins: Dimen = values()
            .map(|c| c.render.margins())
            .map(|m| point!(m.left + m.right, m.top + m.bottom))
            .sum();

        match self.orientation {
            Vertical => {
                let width = values().map(|c| c.render.dimen().width()).max().unwrap();
                values().fold(point!(width, 0), |a, b| a + (0, b.render.dimen().height()))
                    + (padding * 2, padding * 2)
                    + margins
            }
            Horizontal => {
                let height = values().map(|c| c.render.dimen().height()).max().unwrap();
                values().fold(point!(0, height), |a, b| a + (b.render.dimen().width(), 0))
                    + (padding * 2, padding * 2)
                    + margins
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Vertical
    }
}

#[derive(new)]
pub struct Component<'a> {
    id: &'static str,
    order: i32,
    render: Box<dyn Render + 'a>,
    #[new(default)]
    pos: Point,
}

impl<'a> Component<'a> {
    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn render(&self) -> &dyn Render {
        &*self.render
    }
}

impl PartialOrd for Component<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Component<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialEq for Component<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Component<'_> {}

impl fmt::Debug for Component<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Component {{ order: {}, pos: {:?}, dimen: {:?} }}",
            self.order,
            self.pos,
            self.render.dimen()
        )
    }
}
