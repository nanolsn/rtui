use im::{
    GenericImageView,
    DynamicImage,
    ImageError,
};

use crate::common::Vec2d;

#[derive(Copy, Clone, Debug)]
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

#[derive(Debug)]
pub enum TextureError {
    ImageError(ImageError),
    NegativeSize,
}

impl From<ImageError> for TextureError {
    fn from(err: ImageError) -> Self { TextureError::ImageError(err) }
}

#[derive(Debug)]
pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    format: Format,
}

#[allow(dead_code)]
impl Texture {
    pub fn from_size_and_format<S>(size: S, format: Format) -> Result<Self, TextureError>
        where
            S: Into<Vec2d<i32>>,
    { Texture::from_raw(None, format, size.into()) }

    pub fn from_file<S>(file: S) -> Result<Self, TextureError>
        where
            S: AsRef<str>,
    {
        let img = im::open(file.as_ref())?;
        Texture::from_image(img)
    }

    pub fn from_image(img: DynamicImage) -> Result<Self, TextureError> {
        let size = Vec2d::new(img.width() as i32, img.height() as i32);

        let (format, raw) = match img {
            DynamicImage::ImageLuma8(a) => (Format::R, a.into_raw()),
            DynamicImage::ImageLumaA8(a) => (Format::RG, a.into_raw()),
            DynamicImage::ImageRgb8(a) => (Format::RGB, a.into_raw()),
            DynamicImage::ImageRgba8(a) => (Format::RGBA, a.into_raw()),
            _ => panic!("Unsupported format!"),
        };

        Texture::from_raw(Some(raw), format, size)
    }

    fn from_raw(raw: Option<Vec<u8>>, format: Format, size: Vec2d<i32>)
                -> Result<Self, TextureError> {
        let width = size.width();
        let height = size.height();

        if width < 0 || height < 0 {
            return Err(TextureError::NegativeSize);
        }

        let ptr: *const u8 = raw
            .map(|r| r.as_ptr())
            .unwrap_or(std::ptr::null());

        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format.gl_format() as i32,
                width,
                height,
                0,
                format.gl_format(),
                gl::UNSIGNED_BYTE,
                ptr as *const std::ffi::c_void,
            );

            Texture::set_parameters();
        }

        Ok(Texture { id, width, height, format })
    }

    unsafe fn set_parameters() {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }

    pub fn id(&self) -> u32 { self.id }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn width(&self) -> i32 { self.width }

    pub fn height(&self) -> i32 { self.height }

    pub fn size(&self) -> Vec2d<i32> { Vec2d::new(self.width, self.height) }

    pub fn format(&self) -> Format { self.format }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::DeleteTextures(1, &self.id);
        }
    }
}
