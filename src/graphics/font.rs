use super::{
    super::common::{
        Vec2D,
        Rect,
    },
    pages::Pages,
    Texture,
};

#[derive(Debug)]
pub struct Font {
    pub atlas_size: Vec2D<u32>,
    pub chars_on_page: u32,
    pub char_size: Vec2D<u32>,
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

        let (width, height) = pages.first().size().into_inner();
        let char_width = width / atlas_size.x;
        let char_height = height / atlas_size.y;

        Font {
            atlas_size,
            chars_on_page: atlas_size.x * atlas_size.y,
            char_size: Vec2D::new(char_width, char_height),
            st_char: Vec2D::new(1.0 / atlas_size.x as f32, 1.0 / atlas_size.y as f32),
            indent,
            pages,
        }
    }

    pub fn text_size(&self, text: &str) -> Vec2D<u32> {
        match text.chars().count() {
            0 => Vec2D::new(0, self.char_size.y),
            1 => self.char_size,
            l => Vec2D::new(
                self.char_size.x + (l as u32 - 1) * (self.indent as u32 + self.char_size.x),
                self.char_size.y,
            ),
        }
    }

    pub fn char_rect(&self, char_num: u32, pos: Vec2D<f32>) -> Rect<f32> {
        let x_step = (char_num * (self.char_size.x + self.indent)) as f32;
        Rect::new((pos.x + x_step, pos.y), self.char_size.cast::<f32>())
    }

    pub fn st_rect(&self, code: u32) -> Rect<f32> {
        let code_at_page = code % self.chars_on_page;
        let s = (code_at_page % self.atlas_size.x) as f32 * self.st_char.x;
        let t = (code_at_page / self.atlas_size.x) as f32 * self.st_char.y;
        Rect::new((s, t), (self.st_char.x, self.st_char.y))
    }

    pub fn page(&self, code: u32) -> Option<&Texture> {
        let page_code = code / self.chars_on_page;
        self.pages.get(page_code as usize)
    }
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
