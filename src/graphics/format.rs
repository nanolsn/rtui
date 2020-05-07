#[derive(Debug)]
pub enum Format {
    R,
    RG,
    RGB,
    RGBA,
}

impl Format {
    pub fn gl_format(&self) -> u32 {
        match self {
            Format::R => gl::RED,
            Format::RG => gl::RG,
            Format::RGB => gl::RGB,
            Format::RGBA => gl::RGBA,
        }
    }
}
