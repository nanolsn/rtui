use super::{
    super::common::Color,
    uniform::Uniform,
    shaders::ShaderSet,
};

#[derive(Debug)]
pub(super) enum UsedShader {
    Base = 0,
    Font = 1,
}

#[derive(Debug)]
pub(super) struct BaseData {
    pub(super) projection: Uniform<glm::Mat4>,
    pub(super) texture0: Uniform<i32>,
    pub(super) draw_texture: Uniform<bool>,
    pub(super) col: Uniform<Color>,
}

impl BaseData {
    pub(super) fn new(shaders: &mut ShaderSet, projection: glm::Mat4) -> Self {
        shaders.use_shader(UsedShader::Base as usize);

        let projection = shaders
            .make_uniform(projection, c_str!("projection"))
            .unwrap();

        shaders.accept(&projection);

        let texture0 = shaders
            .make_uniform(0, c_str!("texture0"))
            .unwrap();

        shaders.accept(&texture0);

        let draw_texture = shaders
            .make_uniform(false, c_str!("draw_texture"))
            .unwrap();

        shaders.accept(&draw_texture);

        let col = shaders
            .make_uniform(Color::white(), c_str!("col"))
            .unwrap();

        shaders.accept(&col);

        BaseData {
            projection,
            texture0,
            draw_texture,
            col,
        }
    }
}

#[derive(Debug)]
pub(super) struct FontData {
    pub(super) projection: Uniform<glm::Mat4>,
    pub(super) texture0: Uniform<i32>,
    pub(super) col: Uniform<Color>,
}

impl FontData {
    pub(super) fn new(shaders: &mut ShaderSet, projection: glm::Mat4) -> Self {
        shaders.use_shader(UsedShader::Font as usize);

        let projection = shaders
            .make_uniform(projection, c_str!("projection"))
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

        FontData {
            projection,
            texture0,
            col,
        }
    }
}
