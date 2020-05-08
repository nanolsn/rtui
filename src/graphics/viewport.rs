use crate::common::Vec2d;

#[derive(Debug)]
pub struct Viewport {
    size: Vec2d<i32>,
}

impl Viewport {
    pub fn new<S>(size: S) -> Self
        where
            S: Into<Vec2d<i32>>,
    {
        let viewport = Viewport {
            size: size.into(),
        };

        viewport.resize_viewport();
        viewport
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Vec2d<i32> { self.size }

    pub fn resize<S>(&mut self, size: S)
        where
            S: Into<Vec2d<i32>>,
    {
        let size = size.into();

        if self.size == size {
            return;
        }

        self.size = size;
        self.resize_viewport();
    }

    fn resize_viewport(&self) {
        let size = self.size;

        if size.width() < 0 || size.height() < 0 {
            return;
        }

        unsafe { gl::Viewport(0, 0, size.width(), size.height()) }
    }
}
