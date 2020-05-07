#[derive(Debug)]
pub struct Framebuffer {
    id: u32,
}

impl Framebuffer {
    /// The `Framebuffer` constructor.
    ///
    /// This function is unsafe, because a `Framebuffer` needs to be properly deleted,
    /// but it doesn't implement `Drop`.
    unsafe fn new() -> Self {
        let mut id = 0;
        gl::GenFramebuffers(1, &mut id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, id);

        Framebuffer { id }
    }

    pub fn id(&self) -> u32 { self.id }
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

    #[allow(dead_code)]
    pub fn len(&self) -> usize { self.framebuffers.len() }

    pub fn active(&self) -> Option<&Framebuffer> {
        self.bound.map(|idx| &self.framebuffers[idx])
    }

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
}

impl Drop for FramebufferSet {
    fn drop(&mut self) {
        self.bind_default();

        for framebuffer in self.framebuffers.iter() {
            unsafe { gl::DeleteFramebuffers(1, &framebuffer.id) }
        }
    }
}
