use crate::{
    common::Rect,
    graphics::{
        Texture,
        Render,
        Draw,
    },
};

#[derive(Debug)]
pub struct Image {
    file: String,
    texture: Texture,
}

impl Image {
    pub fn new<S>(file: S, _render: &Render) -> Self
        where
            S: Into<String>,
    {
        let file = file.into();
        let texture = Texture::from_file(file.as_str()).unwrap();

        Image { file, texture }
    }
}

impl Draw for Image {
    fn draw(&self, render: &mut Render) {
        render.draw_texture(true);

        let (sw, sh) = render.size();
        let (w, h) = self.texture.size();
        let rect = Rect::new(
            (sw as i32 / 2 - w as i32 / 2, sh as i32 / 2 - h as i32 / 2),
            (w, h),
        );

        render.set_texture(&self.texture);
        render.draw_rect(&rect);

        render.draw_texture(false);
    }
}
