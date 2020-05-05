use super::{
    super::common::Vec2D,
    pages::Pages,
    Texture,
};

#[derive(Debug)]
pub struct Font {
    pub atlas_size: Vec2D<u32>,
    pub chars_on_page: u32,
    pub st_char: Vec2D<f32>,
    pub indent: u32,
    pub pages: Pages<Texture>,
}

impl Font {
    pub fn new<S>(atlas_size: S, indent: u32, pages: Pages<Texture>) -> Self
        where
            S: Into<Vec2D<u32>>,
    {
        let atlas_size = atlas_size.into();

        Font {
            atlas_size,
            chars_on_page: atlas_size.x * atlas_size.y,
            st_char: Vec2D::new(1.0 / atlas_size.x as f32, 1.0 / atlas_size.y as f32),
            indent,
            pages,
        }
    }

    pub fn page_size(&self) -> Vec2D<u32> { self.pages.first().size() }
}

impl Default for Font {
    fn default() -> Self {
        let p0 = Texture::from_file("./data/font/0.png").unwrap();
        let p4 = Texture::from_file("./data/font/4.png").unwrap();

        let mut pages = Pages::new(p0);
        pages.add(p4, 4);

        Font::new((16, 16), 0, pages)
    }
}
