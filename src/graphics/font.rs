use crate::common::Vec2D;

#[derive(Debug)]
pub struct Font {
    pub atlas_size: Vec2D<u32>,
    pub st_char: Vec2D<f32>,
    pub indent: u32,
}

impl Font {
    pub fn new<S>(atlas_size: S, indent: u32) -> Self
        where
            S: Into<Vec2D<u32>>,
    {
        let atlas_size = atlas_size.into();

        Font {
            atlas_size,
            st_char: Vec2D::new(1.0 / atlas_size.x as f32, 1.0 / atlas_size.y as f32),
            indent,
        }
    }
}

impl Default for Font {
    fn default() -> Self { Font::new((16, 16), 0) }
}
