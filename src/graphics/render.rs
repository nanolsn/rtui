use super::{
    super::common::*,
    Draw,
    DrawParameters,
    font_render::FontRender,
    framebuffers::{FramebufferSet, FramebufferError},
    rect_render::RectRender,
    shader_data::*,
    shaders::*,
    texture::{Texture, Format as TextureFormat},
    renderbuffer::Format as RenderbufferFormat,
    uniforms::UniformError,
    viewport::Viewport,
};

#[derive(Debug)]
pub enum RenderError {
    UniformError(UniformError),
    ShaderError(ShaderError),
    FramebufferError(FramebufferError),
}

impl From<UniformError> for RenderError {
    fn from(e: UniformError) -> Self { RenderError::UniformError(e) }
}

impl From<ShaderError> for RenderError {
    fn from(e: ShaderError) -> Self { RenderError::ShaderError(e) }
}

impl From<FramebufferError> for RenderError {
    fn from(e: FramebufferError) -> Self { RenderError::FramebufferError(e) }
}

#[derive(Debug)]
pub struct Render {
    viewport: Viewport,
    shaders: ShaderSet,
    framebuffers: FramebufferSet,
    size: Vec2d<i32>,
    rect_render: RectRender,
    font_render: Option<FontRender>,
    base_data: BaseData,
    post_data: PostData,
    shader_data: ShaderData,
}

impl Render {
    pub fn new(context: &glutin::WindowedContext<glutin::PossiblyCurrent>)
               -> Result<Self, RenderError> {
        gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

        unsafe { Render::set_defaults() };

        let mut shaders = Render::make_shader_set()?;

        let (w, h): (i32, i32) = context
            .window()
            .inner_size()
            .to_logical::<i32>(context.window().scale_factor())
            .into();

        let projection = Render::make_ortho((w as f32, h as f32));
        let base_data = BaseData::new(&mut shaders)?;
        let post_data = PostData::new(&mut shaders)?;
        let shader_data = ShaderData::new(&mut shaders, projection)?;

        let mut framebuffers = FramebufferSet::new();
        framebuffers.add_framebuffer((w, h));
        framebuffers.add_texture(TextureFormat::RGB)?;
        framebuffers.add_renderbuffer(RenderbufferFormat::Depth24)?;

        Ok(Render {
            viewport: Viewport::new((w, h)),
            shaders,
            framebuffers,
            size: (w, h).into(),
            rect_render: RectRender::new(0, 1),
            font_render: Some(FontRender::new()),
            base_data,
            post_data,
            shader_data,
        })
    }

    unsafe fn set_defaults() {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE);
    }

    fn make_shader_set() -> Result<ShaderSet, ShaderError> {
        let mut shaders = ShaderSet::new();

        assert_eq!(shaders.len(), UsedShader::Base as usize);
        shaders.add(
            c_str!(include_str!("../shaders/ui_vs.glsl")),
            c_str!(include_str!("../shaders/ui_fs.glsl")),
        )?;

        assert_eq!(shaders.len(), UsedShader::Font as usize);
        shaders.add(
            c_str!(include_str!("../shaders/ui_vs.glsl")),
            c_str!(include_str!("../shaders/font_fs.glsl")),
        )?;

        assert_eq!(shaders.len(), UsedShader::Post as usize);
        shaders.add(
            c_str!(include_str!("../shaders/post_vs.glsl")),
            c_str!(include_str!("../shaders/post_fs.glsl")),
        )?;

        Ok(shaders)
    }

    fn make_ortho<S>(size: S) -> glm::Mat4
        where
            S: Into<Vec2d<f32>>,
    {
        const NEAR: f32 = 0.0;
        const FAR: f32 = 10.0;

        let size = size.into();
        glm::ortho(0.0, size.x, 0.0, size.y, NEAR, FAR)
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Vec2d<i32> { self.size }

    pub(super) fn resize(&mut self, size: Vec2d<i32>) {
        self.viewport.resize(size);

        let projection = Render::make_ortho(size.cast::<f32>());
        self.shader_data.projection.set_value(projection);

        self.framebuffers.bind(0);
        self.framebuffers.resize(size).unwrap();

        self.size = size;
    }

    pub(super) fn begin_draw_frame(&mut self) {
        self.framebuffers.bind_default();
    }

    pub(super) fn end_draw_frame(&mut self) {
        //
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r(), color.g(), color.b(), color.a());
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_rect(&mut self, rect: Rect<f32>) {
        self.draw_rect_accept(UsedShader::Base, rect, None);
    }

    #[allow(dead_code)]
    pub fn draw_rect_st(&mut self, rect: Rect<f32>, st: Rect<f32>) {
        self.draw_rect_accept(UsedShader::Base, rect, Some(st));
    }

    pub(super) fn draw_rect_accept(&mut self,
                                   shader: UsedShader,
                                   rect: Rect<f32>,
                                   st: Option<Rect<f32>>,
    ) {
        self.shaders.use_shader(shader as usize);
        self.shader_data.accept(&self.shaders);

        if shader == UsedShader::Base {
            self.base_data.draw_texture.accept(&self.shaders);
        }

        if shader == UsedShader::Post {
            self.post_data.frame.accept(&self.shaders);
        }

        self.rect_render.draw(rect, st);
    }

    pub fn draw<D>(&mut self, draw: &D)
        where
            D: Draw,
    {
        draw.draw(self, DrawParameters {
            color: Color::white(),
            position: Position::default(),
            frame: self.size.into_rect(),
        })
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        const TEXTURE0_UNIT: i32 = 0;

        self.shader_data.texture0.set_value(TEXTURE0_UNIT);
        texture.bind(self.shader_data.texture0.get() as u32);

        self.base_data.draw_texture.set_value(true);
    }

    pub fn unset_texture(&mut self) { self.base_data.draw_texture.set_value(false) }

    pub fn print(&mut self, text: &str, params: &DrawParameters) {
        let mut font = self.font_render.take().unwrap();

        let glyphs = font.glyphs(text);
        let pos = params.render_rect(glyphs.size()).pos();

        font.print(self, glyphs.into_inner(), pos.cast());

        self.font_render = Some(font);
    }

    pub fn set_color(&mut self, color: Color) { self.shader_data.col.set_value(color) }
}
