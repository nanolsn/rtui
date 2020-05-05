use super::{
    super::common::{
        Rect,
        Vec2D,
    },
    Render,
    UsedShader,
    font::Font,
};

#[derive(Debug)]
pub struct FontRender {
    char_size: Vec2D<u32>,
    font: Font,
}

impl FontRender {
    pub fn new() -> Self {
        let font = Font::default();

        let (width, height) = font.page_size().into_inner();
        let char_width = width / font.atlas_size.x;
        let char_height = height / font.atlas_size.y;

        FontRender {
            char_size: Vec2D::new(char_width, char_height),
            font,
        }
    }

    pub fn print(&self, render: &mut Render, text: &str, pos: Vec2D<f32>) {
        for (i, code) in text.chars().map(|c| c as u32).enumerate() {
            let font = &self.font;

            let x_step = (i as u32 * (self.char_size.x + font.indent)) as f32;
            let char = Rect::new((pos.x + x_step, pos.y), self.char_size.cast::<f32>());

            let code_at_page = code % font.chars_on_page;
            let s = (code_at_page % font.atlas_size.x) as f32 * font.st_char.x;
            let t = (code_at_page / font.atlas_size.x) as f32 * font.st_char.y;
            let st = Rect::new((s, t), (font.st_char.x, font.st_char.y));

            let page_code = code / font.chars_on_page;
            match font.pages.get(page_code as usize) {
                None => continue,
                Some(texture) => render.set_texture(texture),
            }

            render.draw_rect_accept(UsedShader::Font, char, Some(st));
        }
    }

    pub fn text_size(&self, text: &str) -> Vec2D<u32> {
        match text.chars().count() {
            0 => Vec2D::new(0, self.char_size.y),
            1 => self.char_size,
            l => Vec2D::new(
                self.char_size.x + (l as u32 - 1) * (self.font.indent as u32 + self.char_size.x),
                self.char_size.y,
            ),
        }
    }
}
