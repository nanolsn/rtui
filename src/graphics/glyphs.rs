use super::{
    super::common::Vec2D,
    font::GlyphSize,
};

#[derive(Copy, Clone, Debug)]
pub struct Glyph {
    pub size: GlyphSize,
    pub delta_x: f32,
    pub code: u32,
}

impl Glyph {
    pub fn new(size: GlyphSize, delta_x: f32, code: u32) -> Self { Glyph { size, delta_x, code } }
}

#[derive(Debug)]
pub struct Glyphs {
    glyphs: Vec<Glyph>,
    size: Vec2D<i32>,
}

impl Glyphs {
    pub fn new(glyphs: Vec<Glyph>, size: Vec2D<i32>) -> Self { Glyphs { glyphs, size } }

    pub fn size(&self) -> Vec2D<i32> { self.size }

    pub fn into_inner(self) -> Vec<Glyph> { self.glyphs }
}