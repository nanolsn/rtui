use super::{
    super::common::{
        Rect,
        Vec2D,
    },
    Texture,
    Render,
    UsedShader,
    font::Font,
};

#[derive(Debug)]
pub struct FontRender {
    texture: Texture,
    char_size: Vec2D<u32>,
    font: Font,
}

impl FontRender {
    pub fn new() -> Self {
        let texture = Texture::from_file("./data/font/0.png").unwrap();
        let font = Font::default();
        let char_width = texture.size().x / font.atlas_size.x;
        let char_height = texture.size().y / font.atlas_size.y;

        FontRender {
            texture,
            char_size: Vec2D::new(char_width, char_height),
            font,
        }
    }

    pub fn print(&self, render: &mut Render, text: &str, pos: Vec2D<f32>) {
        render.set_texture(&self.texture);

        for (i, code) in text.chars().map(|c| c as u32).enumerate() {
            let font = &self.font;

            let x_step = (i as u32 * (self.char_size.x + font.indent)) as f32;
            let char = Rect::new((pos.x + x_step, pos.y), self.char_size.cast::<f32>());

            let s = (code % font.atlas_size.x) as f32 * font.st_char.x;
            let t = (code / font.atlas_size.x) as f32 * font.st_char.y;
            let st = Rect::new((s, t), (font.st_char.x, font.st_char.y));

            render.draw_rect_accept(UsedShader::Font, char, Some(st));
        }
    }

    pub fn text_size(&self, text: &str) -> Vec2D<u32> {
        match text.len() {
            0 => Vec2D::new(0, self.char_size.y),
            1 => self.char_size,
            l => Vec2D::new(
                self.char_size.x + (l as u32 - 1) * (self.font.indent as u32 + self.char_size.x),
                self.char_size.y,
            ),
        }
    }
}
