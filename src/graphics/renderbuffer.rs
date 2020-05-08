use crate::common::Vec2d;

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum Format {
    Depth16,
    Depth24,
    DepthF32,
    Depth24Stencil8,
    DepthF32Stencil8,
    Stencil8,
}

impl Format {
    pub fn internal_format(&self) -> u32 {
        match self {
            Format::Depth16 => gl::DEPTH_COMPONENT16,
            Format::Depth24 => gl::DEPTH_COMPONENT24,
            Format::DepthF32 => gl::DEPTH_COMPONENT32F,
            Format::Depth24Stencil8 => gl::DEPTH24_STENCIL8,
            Format::DepthF32Stencil8 => gl::DEPTH32F_STENCIL8,
            Format::Stencil8 => gl::STENCIL_INDEX8,
        }
    }

    #[allow(dead_code)]
    pub fn base_format(&self) -> u32 {
        match self {
            Format::Depth16 => gl::DEPTH_COMPONENT,
            Format::Depth24 => gl::DEPTH_COMPONENT,
            Format::DepthF32 => gl::DEPTH_COMPONENT,
            Format::Depth24Stencil8 => gl::DEPTH_STENCIL,
            Format::DepthF32Stencil8 => gl::DEPTH_STENCIL,
            Format::Stencil8 => gl::STENCIL,
        }
    }

    pub fn attachment(&self) -> u32 {
        match self {
            Format::Depth16 => gl::DEPTH_ATTACHMENT,
            Format::Depth24 => gl::DEPTH_ATTACHMENT,
            Format::DepthF32 => gl::DEPTH_ATTACHMENT,
            Format::Depth24Stencil8 => gl::DEPTH_STENCIL_ATTACHMENT,
            Format::DepthF32Stencil8 => gl::DEPTH_STENCIL_ATTACHMENT,
            Format::Stencil8 => gl::STENCIL_ATTACHMENT,
        }
    }
}

#[derive(Debug)]
pub enum RenderbufferError {
    NegativeSize,
}

#[derive(Debug)]
pub struct Renderbuffer {
    id: u32,
    format: Format,
}

impl Renderbuffer {
    /// The `Framebuffer` constructor.
    ///
    /// This function is unsafe, because a `Renderbuffer` needs to be properly deleted,
    /// but it doesn't implement `Drop`.
    pub unsafe fn new(size: Vec2d<i32>, format: Format) -> Result<Self, RenderbufferError> {
        let width = size.width();
        let height = size.height();

        if width < 0 || height < 0 {
            return Err(RenderbufferError::NegativeSize);
        }

        let mut id = 0;
        gl::GenRenderbuffers(1, &mut id);
        gl::BindRenderbuffer(gl::RENDERBUFFER, id);

        gl::RenderbufferStorageMultisample(
            gl::RENDERBUFFER,
            0,
            format.internal_format(),
            width,
            height,
        );

        Ok(Renderbuffer { id, format })
    }

    pub fn id(&self) -> u32 { self.id }

    pub fn format(&self) -> Format { self.format }

    pub fn attachment(&self) -> u32 { self.format.attachment() }
}
