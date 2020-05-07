use super::{
    super::common::Color,
    shaders::ShaderSet,
    uniforms::{
        Uniform,
        SharedUniform,
        UniformError,
    },
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedShader {
    Base = 0,
    Font = 1,
    Post = 2,
}

#[derive(Debug)]
pub struct BaseData {
    pub draw_texture: Uniform<bool>,
}

impl BaseData {
    pub fn new(shaders: &mut ShaderSet) -> Result<Self, UniformError> {
        shaders.use_shader(UsedShader::Base as usize);

        Ok(BaseData {
            draw_texture: shaders.make_uniform(false, c_str!("draw_texture"))?,
        })
    }
}

#[derive(Debug)]
pub struct PostData {
    pub frame: Uniform<i32>,
}

impl PostData {
    pub fn new(shaders: &mut ShaderSet) -> Result<Self, UniformError> {
        shaders.use_shader(UsedShader::Post as usize);

        Ok(PostData {
            frame: shaders.make_uniform(0, c_str!("frame"))?,
        })
    }
}

#[derive(Debug)]
pub struct ShaderData {
    pub projection: SharedUniform<glm::Mat4>,
    pub texture0: SharedUniform<i32>,
    pub col: SharedUniform<Color>,
}

impl ShaderData {
    pub fn new(shaders: &mut ShaderSet, projection: glm::Mat4) -> Result<Self, UniformError> {
        let used_shaders = [UsedShader::Font, UsedShader::Base];

        Ok(ShaderData {
            projection: shaders.make_shared(projection, c_str!("projection"), &used_shaders)?,
            texture0: shaders.make_shared(0, c_str!("texture0"), &used_shaders)?,
            col: shaders.make_shared(Color::white(), c_str!("col"), &used_shaders)?,
        })
    }

    pub fn accept(&self, shader: &ShaderSet) {
        self.projection.accept(&shader);
        self.texture0.accept(&shader);
        self.col.accept(&shader);
    }
}
