use specs::prelude::*;

use crate::{loader::shaders::ShaderManager, vxl_gl::gl};

pub struct Material {
    shader_program_id: gl::types::GLuint,
}
impl Component for Material {
    type Storage = DenseVecStorage<Self>;
}

impl Material {
    pub fn default(shader_loader: &ShaderManager) -> Material {
        let program = shader_loader.get_shader_program("default");

        Material {
            shader_program_id: program.get_id(),
        }
    }
}

impl Material {
    pub fn get_program_id(&self) -> gl::types::GLuint {
        self.shader_program_id
    }
}
