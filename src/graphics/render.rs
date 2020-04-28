use super::{
    super::common::{
        Color,
        Rect,
        Size,
    },
    shaders::ShaderSet,
    rect_render::RectRender,
    uniform::Uniform,
    texture::Texture,
    Draw,
};

#[derive(Debug)]
pub struct Render {
    shaders: ShaderSet,
    size: Size,
    rect_render: RectRender,
    projection: Uniform<glm::Mat4>,
    texture0: Uniform<i32>,
    draw_texture: Uniform<bool>,
}

impl Render {
    pub fn new(context: &glutin::WindowedContext<glutin::PossiblyCurrent>) -> Self {
        gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        let mut shaders = ShaderSet::new();

        shaders.add(
            c_str!(include_str!("../shaders/vs.glsl")),
            c_str!(include_str!("../shaders/fs.glsl")),
        ).unwrap();

        shaders.use_shader(0);

        let size: (u32, u32) = context
            .window()
            .inner_size()
            .to_logical::<u32>(context.window().scale_factor())
            .into();

        let mat = glm::ortho(0.0, size.0 as f32, 0.0, size.1 as f32, 0.0, 100.0);

        let projection = shaders
            .make_uniform(mat, c_str!("projection"))
            .unwrap();

        shaders.accept(&projection);

        let texture0 = shaders
            .make_uniform(0, c_str!("texture0"))
            .unwrap();

        shaders.accept(&texture0);

        let col = shaders
            .make_uniform(Color::white(), c_str!("col"))
            .unwrap();

        shaders.accept(&col);

        let draw_texture = shaders
            .make_uniform(false, c_str!("draw_texture"))
            .unwrap();

        shaders.accept(&draw_texture);

        Render {
            shaders,
            size: size.into(),
            rect_render: RectRender::new(0, 1),
            projection,
            texture0,
            draw_texture,
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Size { self.size }

    pub fn resize(&mut self, Size(w, h): Size) {
        unsafe { gl::Viewport(0, 0, w as i32, h as i32) }

        self.size = Size(w, h);

        self.projection.set_value(glm::ortho(0.0, w as f32, 0.0, h as f32, 0.0, 100.0));
        self.shaders.accept(&self.projection);
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r(), color.g(), color.b(), 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn use_program(&mut self) { self.shaders.use_shader(0) }

    pub fn draw_rect(&self, rect: &Rect) { self.rect_render.draw(rect) }

    pub fn draw<D>(&mut self, draw: &D)
        where
            D: Draw,
    { draw.draw(self) }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture0.set_value(0);
        self.shaders.accept(&self.texture0);
        texture.bind(*self.texture0.as_ref() as u32);
    }

    pub fn draw_texture(&mut self, draw: bool) {
        self.draw_texture.set_value(draw);
        self.draw_texture.accept(&self.shaders);
    }
}
