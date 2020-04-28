use super::{
    texture::Texture,
    render::Render,
    super::common::{
        Rect,
        Pos,
    },
};

#[derive(Debug)]
pub(super) struct Font {
    texture: Texture,
    char_width: i32,
}

impl Font {
    const FONT_WIDTH: u32 = 16;
    const FONT_HEIGHT: u32 = 16;
    const S_CHAR: f32 = 1.0 / Font::FONT_WIDTH as f32;
    const T_CHAR: f32 = 1.0 / Font::FONT_HEIGHT as f32;

    pub(super) fn new() -> Self {
        let texture = Texture::from_file("./data/font/0.png").unwrap();
        let char_width = (texture.size().width() / Font::FONT_WIDTH) as i32;

        Font { texture, char_width }
    }

    pub(super) fn print(&self, render: &mut Render, text: &str, pos: Pos) {
        for (i, code) in text.chars().map(|ch| ch as u32).enumerate() {
            let x_step = i as i32 * self.char_width;

            let char_rect = Rect::new(
                (pos.x() + x_step, pos.y()),
                (pos.x() + self.char_width + x_step, pos.y() + self.char_width),
            );

            let x = code % Font::FONT_WIDTH;
            let y = code / Font::FONT_WIDTH;

            let s = x as f32 * Font::S_CHAR;
            let t = y as f32 * Font::T_CHAR;

            let _st_rect = Rect::new((s, t), (s + Font::S_CHAR, t + Font::T_CHAR));

            render.set_texture(&self.texture);
            render.draw_rect(char_rect);
            render.unset_texture();
        }
    }
}
