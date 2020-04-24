use std::cell::RefCell;

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
    texture: RefCell<Option<Texture>>,
}

impl Image {
    pub fn new<S>(file: S) -> Self
        where
            S: Into<String>,
    {
        let file = file.into();
        Image { file, texture: RefCell::new(None) }
    }
}

impl Draw for Image {
    fn draw(&self, render: &mut Render) {
        if self.texture.borrow().is_none() {
            *self.texture.borrow_mut() = Some(Texture::from_file(self.file.as_str()).unwrap());
        }

        let texture = self.texture.borrow();
        let texture = texture.as_ref().unwrap();

        render.draw_texture(true);

        let (sw, sh) = render.size();
        let (w, h) = texture.size();
        let rect = Rect::new(
            (sw as i32 / 2 - w as i32 / 2, sh as i32 / 2 - h as i32 / 2),
            (w, h),
        );

        render.set_texture(texture);
        render.draw_rect(&rect);

        render.draw_texture(false);
    }
}
