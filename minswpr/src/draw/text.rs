use super::DrawContext;
use crate::MsResult;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

#[derive(new)]
pub struct Text<T: ToString> {
    content: T,
    font_id: &'static str,
    color: Color,
}

#[derive(new)]
pub struct RenderedText<'a> {
    query: TextureQuery,
    texture: Texture<'a>,
}

impl RenderedText<'_> {
    pub fn query(&self) -> &TextureQuery {
        &self.query
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

pub type TextResult<'a> = MsResult<RenderedText<'a>>;

pub fn make_text<'a, T>(ctx: &'a DrawContext<'a>, text: Text<T>) -> TextResult<'a>
where
    T: ToString,
{
    fn map_err<T: ToString>(e: T) -> String {
        format!("could not draw text: `{}`", e.to_string())
    }

    let surface = ctx
        .fonts()
        .get(text.font_id)?
        .render(&text.content.to_string())
        .blended(text.color)
        .map_err(map_err)?;

    let texture = ctx
        .textures()
        .create_texture_from_surface(&surface)
        .map_err(map_err)?;

    let query = texture.query();

    Ok(RenderedText::new(query, texture))
}
