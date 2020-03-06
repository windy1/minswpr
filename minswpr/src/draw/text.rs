use super::DrawContext;
use crate::MsResult;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

/// Helper result type for font rendering
pub type TextResult<'a> = MsResult<RenderedText<'a>>;

/// Represents text that *will* be rendered
#[derive(new)]
pub struct Text<T: ToString> {
    content: T,
    font_id: &'static str,
    color: Color,
}

/// Represents text that *has* been rendered and can be copied to the canvas
#[derive(new)]
pub struct RenderedText<'a> {
    query: TextureQuery,
    texture: Texture<'a>,
}

impl RenderedText<'_> {
    /// Returns the rendered `TextureQuery`
    pub fn query(&self) -> &TextureQuery {
        &self.query
    }

    /// Returns the rendered `Texture`
    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

/// Tries to render the specified `Text` and returns a `Some(RenderedText)` if
/// successful. Returns `Err` otherwise.
///
/// # Arguments
/// `ctx` - The game's `DrawContext`
/// `text` - The `Text` to render
pub fn make_text<'a, T>(ctx: &'a DrawContext<'a>, text: Text<T>) -> TextResult<'a>
where
    T: ToString,
{
    fn map_err<T: ToString>(e: T) -> String {
        format!("could not draw text: `{}`", e.to_string())
    }

    let surface = ctx.fonts()[text.font_id]
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
