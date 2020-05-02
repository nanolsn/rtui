use super::{
    super::common::Color,
    uniform::Uniform,
    shaders::ShaderSet,
};

#[derive(Debug)]
pub enum UsedShader {
    Base = 0,
    Font = 1,
}

#[derive(Debug)]
pub(super) struct BaseData {
    pub(super) draw_texture: Uniform<bool>,
    pub(super) col: Uniform<Color>,
}

impl BaseData {
    pub(super) fn new(shaders: &mut ShaderSet) -> Self {
        shaders.use_shader(UsedShader::Base as usize);

        let draw_texture = shaders
            .make_uniform(false, c_str!("draw_texture"))
            .unwrap();

        shaders.accept(&draw_texture);

        let col = shaders
            .make_uniform(Color::white(), c_str!("col"))
            .unwrap();

        shaders.accept(&col);

        BaseData { draw_texture, col }
    }
}

#[derive(Debug)]
pub(super) struct FontData {
    pub(super) col: Uniform<Color>,
}

impl FontData {
    pub(super) fn new(shaders: &mut ShaderSet) -> Self {
        shaders.use_shader(UsedShader::Font as usize);

        let col = shaders
            .make_uniform(Color::white(), c_str!("col"))
            .unwrap();

        shaders.accept(&col);

        FontData { col }
    }
}
