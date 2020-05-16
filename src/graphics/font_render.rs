use super::{
    super::common::Rect,
    font::Font,
    glyphs::*,
    Render,
    shader_data::UsedShader,
};

#[derive(Debug)]
pub struct FontRender {
    font: Font,
    buf: Option<Vec<Char>>,
}

impl FontRender {
    pub fn new() -> Self {
        let font = Font::default();
        FontRender { font, buf: Some(vec![]) }
    }

    pub fn print(&self, render: &mut Render, chars: &[Char], rect: Rect<i32>) {
        let mut pos = rect.pos();
        pos.y += rect.height - self.font.default_size().height();

        for ch in chars {
            match ch {
                Char::Print(glyph) => {
                    match self.font.page(glyph.code) {
                        None => continue,
                        Some(texture) => render.set_texture(texture),
                    }

                    let placing = self.font.placing(*glyph, pos).cast();
                    let st_map = self.font.st_map(*glyph);
                    render.draw_rect_accept(UsedShader::Font, placing, Some(st_map), true);
                }
                Char::NewLine => {
                    pos.y -= self.font.new_line_height();
                }
            }
        }
    }

    pub fn glyphs(&mut self, text: &str, monospaced: bool) -> Glyphs {
        self.font.glyphs(text, self.buf.take().unwrap(), monospaced)
    }

    pub fn print_end(&mut self, mut buf: Vec<Char>) {
        buf.clear();
        self.buf = Some(buf);
    }
}
