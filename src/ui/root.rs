use crate::common::color::Color;

#[derive(Debug)]
pub struct Root {
    pub bg: Color,
}

impl Root {
    pub fn new(bg: Color) -> Self { Root { bg } }
}
