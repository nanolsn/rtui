use super::{
    super::common::Vec2d,
    renderbuffer::{
        Renderbuffer,
        RenderbufferError,
        Format as RenderbufferFormat,
    },
    texture::{
        Texture,
        TextureError,
        Format as TextureFormat,
    },
};

#[derive(Debug)]
pub enum FramebufferError {
    RenderbufferError(RenderbufferError),
    TextureError(TextureError),
}

impl From<RenderbufferError> for FramebufferError {
    fn from(e: RenderbufferError) -> Self { FramebufferError::RenderbufferError(e) }
}

impl From<TextureError> for FramebufferError {
    fn from(e: TextureError) -> Self { FramebufferError::TextureError(e) }
}

#[derive(Debug)]
pub struct Framebuffer {
    id: u32,
    renderbuffer: Option<Renderbuffer>,
    textures: Vec<Texture>,
    size: Vec2d<i32>,
}

impl Framebuffer {
    /// The `Framebuffer` constructor.
    ///
    /// This function is unsafe, because a `Framebuffer` needs to be properly deleted,
    /// but it doesn't implement `Drop`.
    unsafe fn new(size: Vec2d<i32>) -> Self {
        let mut id = 0;
        gl::GenFramebuffers(1, &mut id);

        Framebuffer {
            id,
            renderbuffer: None,
            textures: vec![],
            size,
        }
    }

    unsafe fn delete(self) {
        if let Some(renderbuffer) = self.renderbuffer {
            gl::DeleteRenderbuffers(1, &renderbuffer.id());
        }

        gl::DeleteFramebuffers(1, &self.id);
    }
}

#[derive(Debug)]
pub struct FramebufferSet {
    framebuffers: Vec<Framebuffer>,
    bound: Option<usize>,
}

impl FramebufferSet {
    pub fn new() -> Self {
        FramebufferSet {
            framebuffers: vec![],
            bound: None,
        }
    }

    pub fn add_framebuffer<S>(&mut self, size: S)
        where
            S: Into<Vec2d<i32>>,
    {
        let framebuffer = unsafe { Framebuffer::new(size.into()) };
        self.framebuffers.push(framebuffer);

        self.bind(self.framebuffers.len() - 1);
    }

    pub fn add_renderbuffer(&mut self, format: RenderbufferFormat) -> Result<(), FramebufferError> {
        let framebuffer = self.active_mut();

        if framebuffer.renderbuffer.is_some() {
            panic!("The renderbuffer already added!");
        }

        unsafe {
            let renderbuffer = Renderbuffer::new(framebuffer.size, format)?;

            gl::BindRenderbuffer(gl::RENDERBUFFER, renderbuffer.id());
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                renderbuffer.attachment(),
                gl::RENDERBUFFER,
                renderbuffer.id(),
            );

            framebuffer.renderbuffer = Some(renderbuffer);
        }

        Ok(())
    }

    pub fn add_texture(&mut self, format: TextureFormat) -> Result<(), FramebufferError> {
        let framebuffer = self.active_mut();

        let texture = Texture::from_size_and_format(framebuffer.size, format)?;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id());
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0 + framebuffer.textures.len() as u32,
                gl::TEXTURE_2D,
                texture.id(),
                0,
            );
        }

        framebuffer.textures.push(texture);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize { self.framebuffers.len() }

    #[allow(dead_code)]
    pub fn active(&self) -> &Framebuffer {
        self.bound
            .map(|idx| &self.framebuffers[idx])
            .expect("Framebuffer not bound!")
    }

    pub fn active_mut(&mut self) -> &mut Framebuffer {
        self.bound
            .map(move |idx| &mut self.framebuffers[idx])
            .expect("Framebuffer not bound!")
    }

    #[allow(dead_code)]
    pub fn bound(&self) -> Option<usize> { self.bound }

    pub fn bind(&mut self, idx: usize) {
        match self.bound {
            Some(i) if i == idx => return,
            _ => {
                unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffers[idx].id) };
                self.bound = Some(idx);
            }
        }
    }

    pub fn bind_default(&mut self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) };
        self.bound = None;
    }

    #[allow(dead_code)]
    pub fn is_completed(&self) -> bool {
        if self.bound.is_none() {
            panic!("Framebuffer not bound!");
        }

        unsafe { gl::CheckFramebufferStatus(gl::DRAW_FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE }
    }
}

impl Drop for FramebufferSet {
    fn drop(&mut self) {
        self.bind_default();

        for framebuffer in self.framebuffers.drain(..) {
            unsafe { framebuffer.delete() }
        }
    }
}
