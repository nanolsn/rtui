use std::collections::HashMap;

use super::{
    super::common::{
        Vec2d,
        Rect,
    },
    glyphs::*,
    pages::Pages,
    Texture,
};

#[derive(Copy, Clone, Debug)]
pub struct GlyphSize {
    pub left_offset: f32,
    pub width: f32,
}

impl GlyphSize {
    pub fn new(left_offset: i32, width: i32) -> Self {
        GlyphSize {
            left_offset: left_offset as f32,
            width: width as f32,
        }
    }
}

#[derive(Debug)]
pub struct Font {
    atlas_size: Vec2d<i32>,
    glyphs_on_page: i32,
    glyph_size_default: Vec2d<i32>,
    glyphs_st_default: Vec2d<f32>,
    indent: i32,
    line_spacing: i32,
    pages: Pages<Texture>,
    glyph_widths: HashMap<char, GlyphSize>,
}

impl Font {
    pub fn new<S>(
        atlas_size: S,
        indent: i32,
        line_spacing: i32,
        pages: Pages<Texture>,
        glyph_widths: HashMap<char, GlyphSize>,
    ) -> Self
        where
            S: Into<Vec2d<i32>>,
    {
        let atlas_size = atlas_size.into();

        let (width, height) = pages.first().size().into_inner();
        let glyph_width = width / atlas_size.width();
        let glyph_height = height / atlas_size.height();

        Font {
            atlas_size,
            glyphs_on_page: atlas_size.width() * atlas_size.height(),
            glyph_size_default: Vec2d::new(glyph_width, glyph_height),
            glyphs_st_default: Vec2d::new(
                1.0 / atlas_size.width() as f32,
                1.0 / atlas_size.height() as f32,
            ),
            indent,
            line_spacing,
            pages,
            glyph_widths,
        }
    }

    pub fn glyphs(&self, text: &str, mut buf: Vec<Glyph>) -> Glyphs {
        let mut delta_x = 0.0;
        let indent = self.indent as f32;

        for ch in text.chars() {
            let size = self.glyph_widths
                .get(&ch)
                .cloned()
                .unwrap_or(GlyphSize::new(0, self.glyph_size_default.width()));

            buf.push(Glyph::new(size, delta_x, ch as u32));

            delta_x += size.width + indent;
        }

        Glyphs::new(buf, Vec2d::new(
            (delta_x - indent) as i32,
            self.glyph_size_default.height(),
        ))
    }

    fn placing(&self, glyph: Glyph, pos: Vec2d<f32>) -> Rect<f32> {
        Rect::new(
            (pos.x + glyph.delta_x, pos.y),
            (glyph.size.width, self.glyph_size_default.height() as f32),
        )
    }

    fn st_map(&self, glyph: Glyph) -> Rect<f32> {
        let code_at_page = glyph.code as i32 % self.glyphs_on_page;
        let default_width = self.glyph_size_default.width() as f32;
        let atlas_width = self.atlas_size.width();

        let left_offset = glyph.size.left_offset / default_width;

        let s = ((code_at_page % atlas_width) as f32 + left_offset)
            * self.glyphs_st_default.width();
        let t = (code_at_page / atlas_width) as f32 * self.glyphs_st_default.height();

        Rect::new((s, t), (
            (glyph.size.width / default_width) * self.glyphs_st_default.width(),
            self.glyphs_st_default.height(),
        ))
    }

    pub fn render_rect(&self, glyph: Glyph, pos: Vec2d<f32>) -> (Rect<f32>, Rect<f32>) {
        (self.placing(glyph, pos), self.st_map(glyph))
    }

    pub fn page(&self, code: u32) -> Option<&Texture> {
        let page_code = code as i32 / self.glyphs_on_page;
        self.pages.get(page_code as usize)
    }
}

impl Default for Font {
    fn default() -> Self {
        let p0 = Texture::from_file("./data/font/0.png").unwrap();
        let p4 = Texture::from_file("./data/font/4.png").unwrap();

        let mut pages = Pages::new(p0);
        pages.add(p4, 4);

        let mut glyph_widths = HashMap::new();
        glyph_widths.insert('!', GlyphSize::new(2, 4));
        glyph_widths.insert(':', GlyphSize::new(2, 4));
        glyph_widths.insert(';', GlyphSize::new(2, 4));
        glyph_widths.insert('.', GlyphSize::new(2, 4));
        glyph_widths.insert(',', GlyphSize::new(2, 4));
        glyph_widths.insert('`', GlyphSize::new(0, 6));
        glyph_widths.insert('|', GlyphSize::new(2, 4));
        glyph_widths.insert('i', GlyphSize::new(0, 6));
        glyph_widths.insert('j', GlyphSize::new(0, 6));
        glyph_widths.insert('l', GlyphSize::new(0, 6));
        glyph_widths.insert('M', GlyphSize::new(0, 9));
        glyph_widths.insert('m', GlyphSize::new(-1, 9));
        glyph_widths.insert('W', GlyphSize::new(0, 9));
        glyph_widths.insert('w', GlyphSize::new(0, 9));

        glyph_widths.insert('М', GlyphSize::new(0, 9));
        glyph_widths.insert('м', GlyphSize::new(0, 9));
        glyph_widths.insert('Ы', GlyphSize::new(0, 9));
        glyph_widths.insert('ы', GlyphSize::new(0, 9));
        glyph_widths.insert('Ю', GlyphSize::new(0, 9));
        glyph_widths.insert('ю', GlyphSize::new(0, 9));

        Font::new((16, 16), 0, 1, pages, glyph_widths)
    }
}
