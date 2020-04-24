use super::{
    super::common::{color::Color, rect::Rect},
    window::Window,
    shaders::ShaderSet,
    rect_render::RectRender,
    draw::Draw,
};

#[derive(Debug)]
pub struct Render {
    shaders: ShaderSet,
    size: (u32, u32),
    rect_render: RectRender,
}

impl Render {
    pub fn new(context: &glutin::WindowedContext<glutin::PossiblyCurrent>) -> Self {
        gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        Render {
            shaders: {
                let mut shaders = ShaderSet::new();

                shaders.add(
                    c_str!(include_str!("../shaders/vs.glsl")),
                    c_str!(include_str!("../shaders/fs.glsl")),
                ).unwrap();

                shaders.use_shader(0);
                Render::set_defaults(&shaders);

                shaders
            },

            size: context
                .window()
                .inner_size()
                .to_logical::<u32>(context.window().scale_factor())
                .into(),

            rect_render: RectRender::new(0, 1),
        }
    }

    fn set_defaults(shader: &ShaderSet) {
        shader
            .make_uniform(Color::white(), c_str!("col"))
            .unwrap()
            .accept();

        shader
            .make_uniform(false, c_str!("draw_texture"))
            .unwrap()
            .accept();
    }

    pub fn size(&self) -> (u32, u32) { self.size }

    pub fn resize(&mut self, (w, h): (u32, u32)) {
        unsafe { gl::Viewport(0, 0, w as i32, h as i32) }

        self.size = (w, h);
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r(), color.g(), color.b(), 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn use_program(&mut self) { self.shaders.use_shader(0) }

    pub fn draw_rect(&self, rect: &Rect) { self.rect_render.draw(rect) }

    pub fn draw<D>(&self, draw: &D)
        where
            D: Draw,
    { draw.draw(self) }
}
