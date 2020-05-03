use super::{
    super::common::{
        Rect,
        Vec2D,
    },
    Texture,
    Render,
    UsedShader,
};

#[derive(Debug)]
pub struct FontRender {
    texture: Texture,
    char_width: u32,
    char_height: u32,
}

impl FontRender {
    const FONT_WIDTH: u32 = 16;
    const FONT_HEIGHT: u32 = 16;
    const S_CHAR: f32 = 1.0 / FontRender::FONT_WIDTH as f32;
    const T_CHAR: f32 = 1.0 / FontRender::FONT_HEIGHT as f32;
    const INDENT: u32 = 0;

    pub fn new() -> Self {
        let texture = Texture::from_file("./data/font/0.png").unwrap();
        let char_width = texture.size().x / FontRender::FONT_WIDTH;
        let char_height = texture.size().y / FontRender::FONT_HEIGHT;

        FontRender { texture, char_width, char_height }
    }

    pub fn print(&self, render: &mut Render, text: &str, pos: Vec2D<f32>) {
        render.set_texture(&self.texture);

        for (i, code) in text.chars().map(|c| c as u32).enumerate() {
            let x_step = (i as u32 * (self.char_width + FontRender::INDENT)) as f32;

            let char_rect = Rect::new(
                (pos.x + x_step, pos.y),
                (self.char_width as f32, self.char_height as f32),
            );

            let s = (code % FontRender::FONT_WIDTH) as f32 * FontRender::S_CHAR;
            let t = (code / FontRender::FONT_WIDTH) as f32 * FontRender::T_CHAR;

            let st_rect = Rect::new((s, t), (FontRender::S_CHAR, FontRender::T_CHAR));

            render.draw_rect_accept(UsedShader::Font, char_rect, Some(st_rect));
        }
    }

    pub fn text_size(&self, text: &str) -> Vec2D<u32> {
        match text.len() {
            0 => Vec2D::new(0, self.char_height),
            1 => Vec2D::new(self.char_width, self.char_height),
            l => Vec2D::new(
                self.char_width + (l as u32 - 1) * (FontRender::INDENT as u32 + self.char_width),
                self.char_height,
            ),
        }
    }
}
