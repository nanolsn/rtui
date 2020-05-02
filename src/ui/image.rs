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
        let texture_size = self.texture.size().cast();

        render.set_texture(&self.texture);
        render.draw_rect(Rect::new(
            render.size().half().cast::<f32>() - texture_size.half(),
            texture_size,
        ));
    }
}
