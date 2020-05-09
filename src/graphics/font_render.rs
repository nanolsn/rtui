use super::{
    super::common::Vec2d,
    font::Font,
    glyphs::*,
    Render,
    shader_data::UsedShader,
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

    pub fn print(&self, render: &mut Render, glyphs: &[Glyph], pos: Vec2d<f32>) {
        for glyph in glyphs {
            match self.font.page(glyph.code) {
                None => continue,
                Some(texture) => render.set_texture(texture),
            }

            let (placing, st_map) = self.font.render_rect(*glyph, pos);
            render.draw_rect_accept(UsedShader::Font, placing, Some(st_map), true);
        }
    }

    pub fn glyphs(&mut self, text: &str, monospaced: bool) -> Glyphs {
        self.font.glyphs(text, self.buf.take().unwrap(), monospaced)
    }

    pub fn print_end(&mut self, mut glyphs: Vec<Glyph>) {
        glyphs.clear();
        self.buf = Some(glyphs);
    }
}
