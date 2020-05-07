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
        let fb = unsafe { Framebuffer::new() };

        let mut set = FramebufferSet {
            framebuffers: vec![fb],
            bound: None,
        };

        set.bind(0);
        println!("{}", set.is_completed());
        set.bind_default();

        set
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

    pub fn bind_by_id(&mut self, id: u32) -> bool {
        self.framebuffers
            .iter()
            .enumerate()
            .find(|(_, fb)| fb.id == id)
            .map(|(idx, _)| idx)
            .map(|idx| self.bind(idx))
            .is_some()
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

        unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE }
    }
}

impl Drop for FramebufferSet {
    fn drop(&mut self) {
        self.bind_default();

        for framebuffer in &self.framebuffers {
            unsafe { gl::DeleteFramebuffers(1, &framebuffer.id) }
        }
    }
}
