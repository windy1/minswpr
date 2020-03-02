use crate::config::LayoutConfig;
use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::render::WindowCanvas;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type RenderRef<'a> = Box<dyn Render + 'a>;
pub type ComponentMap<'a> = HashMap<&'static str, Component<'a>>;

pub trait Layout<'a> {
    fn components(&self) -> &ComponentMap;

    fn components_mut(&mut self) -> &mut ComponentMap<'a>;

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

#[derive(new)]
pub struct LayoutBase<'a> {
    #[new(default)]
    components: ComponentMap<'a>,
    config: LayoutConfig,
}

impl<'a> Layout<'a> for LayoutBase<'a> {
    fn components(&self) -> &ComponentMap {
        &self.components
    }

    fn components_mut(&mut self) -> &mut ComponentMap<'a> {
        &mut self.components
    }
}

impl Render for LayoutBase<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        let c = &self.config;
        render_rect!(self.dimen(), c.color, canvas, pos)?;

        let mut cur = pos + point!(c.padding, c.padding).as_i32();
        let components = &mut self.components.values_mut().collect::<Vec<_>>();

        components.sort();

        for component in components {
            component.pos = cur;
            let r = &mut component.render;
            r.render(canvas, cur)?;
            cur += (0, r.dimen().height() as i32);
        }

        Ok(())
    }

    fn dimen(&self) -> Dimen {
        let c = &self.config;
        let values = || self.components.values();
        let width = values().map(|c| c.render.dimen().width()).max().unwrap();
        values().fold(point!(width, 0), |a, b| a + (0, b.render.dimen().height()))
            + (c.padding * 2, c.padding * 2)
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
