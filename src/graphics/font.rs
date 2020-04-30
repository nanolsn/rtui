use super::{
    texture::Texture,
    render::Render,
    super::common::{
        Rect,
        Vec2D,
    },
};

#[derive(Debug)]
pub(super) struct Font {
    texture: Texture,
    char_width: u32,
}

impl Font {
    const FONT_WIDTH: u32 = 16;
    const FONT_HEIGHT: u32 = 16;
    const S_CHAR: f32 = 1.0 / Font::FONT_WIDTH as f32;
    const T_CHAR: f32 = 1.0 / Font::FONT_HEIGHT as f32;

    pub(super) fn new() -> Self {
        let texture = Texture::from_file("./data/font/0.png").unwrap();
        let char_width = texture.size().x / Font::FONT_WIDTH;

        Font { texture, char_width }
    }

    pub(super) fn print(&self, render: &mut Render, text: &str, pos: Vec2D<f32>) {
        for (i, code) in text.chars().map(|c| c as u32).enumerate() {
            let x_step = (i as u32 * self.char_width) as f32;

            let char_rect = Rect::new(
                (pos.x + x_step, pos.y),
                (pos.x + self.char_width as f32 + x_step, pos.y + self.char_width as f32),
            );

            let s = (code % Font::FONT_WIDTH) as f32 * Font::S_CHAR;
            let t = (code / Font::FONT_WIDTH) as f32 * Font::T_CHAR;

            let st_rect = Rect::new((s, t), (s + Font::S_CHAR, t + Font::T_CHAR));

            render.set_texture(&self.texture);
            render.draw_rect_st(char_rect, st_rect);
            render.unset_texture();
        }
    }
}
