mod base;

pub use self::base::*;

use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

pub type RenderRef<'a> = Box<dyn Render + 'a>;
pub type ComponentMap<'a> = HashMap<&'static str, Component<'a>>;

pub trait Layout<'a> {
    fn components(&self) -> &ComponentMap;

    fn components_mut(&mut self) -> &mut ComponentMap<'a>;

    fn color(&self) -> Option<Color> {
        None
    }

    fn padding(&self) -> u32 {
        0
    }

    fn orientation(&self) -> Orientation {
        Default::default()
    }

    fn insert(&mut self, key: &'static str, order: i32, component: RenderRef<'a>) {
        self.components_mut()
            .insert(key, Component::new(order, component));
    }

    fn insert_all(&mut self, mut components: Vec<(&'static str, RenderRef<'a>)>) {
        for (i, c) in components.drain(..).enumerate() {
            self.insert(c.0, i as i32, c.1);
        }
    }

    fn get(&self, key: &'static str) -> Result<&Component, String> {
        self.components()
            .get(key)
            .ok_or_else(|| format!("missing required layout component `{}`", key))
    }
}

pub fn do_render<'a, T>(layout: &mut T, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String>
where
    T: Layout<'a> + Render,
{
    if let Some(c) = layout.color() {
        render_rect!(layout.dimen(), c, canvas, pos)?;
    }

    let padding = layout.padding();
    let mut cur = pos + point!(padding, padding).as_i32();
    let mut components = layout.components_mut().values_mut().collect::<Vec<_>>();

    components.sort();

    for component in components {
        let r = &mut component.render;
        let m = r.margins();

        cur += point!(m.left, m.top).as_i32();
        component.pos = cur;
        r.render(canvas, component.pos)?;
        cur += point!(0, r.dimen().height()).as_i32() + point!(m.right, m.bottom).as_i32();
    }

    Ok(())
}

pub fn calc_dimen<'a, T>(layout: &T) -> Dimen
where
    T: Layout<'a>,
{
    let padding = layout.padding();
    let values = || layout.components().values();
    let width = values().map(|c| c.render.dimen().width()).max().unwrap();
    values().fold(point!(width, 0), |a, b| a + (0, b.render.dimen().height()))
        + (padding * 2, padding * 2)
}

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
    order: i32,
    render: Box<dyn Render + 'a>,
    #[new(default)]
    pos: Point,
}

impl<'a> Component<'a> {
    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn render(&self) -> &(dyn Render + 'a) {
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
