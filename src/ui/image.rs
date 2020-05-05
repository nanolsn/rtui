use crate::graphics::{
    Texture,
    Render,
    Draw,
    DrawParameters,
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
    fn draw(&self, render: &mut Render, params: DrawParameters) {
        let rect = params.position.rect(
            render.size().into_rect(),
            self.texture.size(),
        );

        render.set_color(params.color);
        render.set_texture(&self.texture);
        render.draw_rect(rect.cast());
    }
}
