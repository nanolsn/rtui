use super::{
    super::common::Vec2D,
    shader_data::UsedShader,
    font::Font,
    glyphs::*,
    Render,
};

#[derive(Debug)]
pub struct FontRender {
    font: Font,
    buf: Option<Vec<Glyph>>,
}

impl FontRender {
    pub fn new() -> Self {
        let font = Font::default();
        FontRender { font, buf: Some(Vec::new()) }
    }

    pub fn print(&mut self, render: &mut Render, mut glyphs: Vec<Glyph>, pos: Vec2D<f32>) {
        for glyph in glyphs.drain(..) {
            match self.font.page(glyph.code) {
                None => continue,
                Some(texture) => render.set_texture(texture),
            }

            let (placing, st_map) = self.font.render_rect(glyph, pos);
            render.draw_rect_accept(UsedShader::Font, placing, Some(st_map));
        }

        self.buf = Some(glyphs);
    }

    pub fn glyphs(&mut self, text: &str) -> Glyphs {
        self.font.glyphs(text, self.buf.take().unwrap())
    }
}
