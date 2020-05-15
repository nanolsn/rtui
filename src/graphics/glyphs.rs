use super::{
    super::common::Vec2d,
    font::GlyphSize,
};

#[derive(Copy, Clone, Debug)]
pub struct Glyph {
    pub size: GlyphSize,
    pub delta_x: i32,
    pub code: u32,
}

impl Glyph {
    pub fn new(size: GlyphSize, delta_x: i32, code: u32) -> Self { Glyph { size, delta_x, code } }
}

#[derive(Debug)]
pub struct Glyphs {
    glyphs: Vec<Glyph>,
    size: Vec2d<i32>,
}

impl Glyphs {
    pub fn new(glyphs: Vec<Glyph>, size: Vec2d<i32>) -> Self { Glyphs { glyphs, size } }

    pub fn size(&self) -> Vec2d<i32> { self.size }

    pub fn into_inner(self) -> Vec<Glyph> { self.glyphs }
}

impl std::ops::Deref for Glyphs {
    type Target = [Glyph];

    fn deref(&self) -> &Self::Target { self.glyphs.as_slice() }
}
