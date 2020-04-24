use image::{
    GenericImageView,
    DynamicImage,
    ImageError,
};

#[derive(Debug)]
pub enum TextureError {
    ImageError(ImageError),
    EmptySize,
}

impl From<ImageError> for TextureError {
    fn from(err: ImageError) -> Self { TextureError::ImageError(err) }
}

#[derive(Debug)]
pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Texture {
    pub fn from_size((width, height): (u32, u32)) -> Result<Self, TextureError> {
        let img = image::DynamicImage::new_rgba8(width, height);
        Texture::new(img)
    }

    pub fn from_file<S>(file: S) -> Result<Self, TextureError>
        where
            S: AsRef<str>,
    {
        let img = image::open(file.as_ref())?;
        Texture::new(img)
    }

    pub fn new(img: DynamicImage) -> Result<Self, TextureError> {
        let width = img.width();
        let height = img.height();

        if width == 0 || height == 0 {
            return Err(TextureError::EmptySize);
        }

        let (format, raw) = match img {
            DynamicImage::ImageLuma8(a) => (gl::RED, a.into_raw()),
            DynamicImage::ImageLumaA8(a) => (gl::RG, a.into_raw()),
            DynamicImage::ImageRgb8(a) => (gl::RGB, a.into_raw()),
            DynamicImage::ImageRgba8(a) => (gl::RGBA, a.into_raw()),
            _ => panic!("Unsupported format!"),
        };

        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                raw.as_ptr() as *const std::ffi::c_void,
            );

            Texture::set_parameters();

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture { id, width, height })
    }

    unsafe fn set_parameters() {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    pub(super) fn id(&self) -> u32 { self.id }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn size(&self) -> (u32, u32) { (self.width, self.height) }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::DeleteTextures(1, &self.id);
        }
    }
}
