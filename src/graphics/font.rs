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
pub(super) struct Font {
    texture: Texture,
    char_width: u32,
    char_height: u32,
}

impl Font {
    const FONT_WIDTH: u32 = 16;
    const FONT_HEIGHT: u32 = 16;
    const S_CHAR: f32 = 1.0 / Font::FONT_WIDTH as f32;
    const T_CHAR: f32 = 1.0 / Font::FONT_HEIGHT as f32;
    const INDENT: u32 = 1;

    pub(super) fn new() -> Self {
        let texture = Texture::from_file("./data/font/0.png").unwrap();
        let char_width = texture.size().x / Font::FONT_WIDTH;
        let char_height = texture.size().y / Font::FONT_HEIGHT;

        Font { texture, char_width, char_height }
    }

    pub(super) fn print(&self, render: &mut Render, text: &str, pos: Vec2D<f32>) {
        render.set_texture(&self.texture);

        for (i, code) in text.chars().map(|c| c as u32).enumerate() {
            let x_step = (i as u32 * (self.char_width + Font::INDENT)) as f32;

            let char_rect = Rect::new(
                (pos.x + x_step, pos.y),
                (self.char_width as f32, self.char_height as f32),
            );

            let s = (code % Font::FONT_WIDTH) as f32 * Font::S_CHAR;
            let t = (code / Font::FONT_WIDTH) as f32 * Font::T_CHAR;

            let st_rect = Rect::new((s, t), (Font::S_CHAR, Font::T_CHAR));

            render.draw_rect_st(UsedShader::Font, char_rect, st_rect);
        }
    }

    pub(super) fn text_size(&self, text: &str) -> Vec2D<u32> {
        match text.len() {
            0 => Vec2D::new(0, self.char_height),
            1 => Vec2D::new(self.char_width, self.char_height),
            l => Vec2D::new(
                self.char_width + (l as u32 - 1) * (Font::INDENT as u32 + self.char_width),
                self.char_height,
            ),
        }
    }
}
