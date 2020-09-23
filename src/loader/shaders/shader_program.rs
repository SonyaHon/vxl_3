use crate::vxl_gl::{gl, Gl};

use super::shader::Shader;

pub struct ShaderProgram {
    id: gl::types::GLuint,
    shader_ids: Vec<gl::types::GLuint>,
}

impl ShaderProgram {
    pub fn from_shaders(gl: Gl, shaders: &Vec<Shader>) -> ShaderProgram {
        let id: gl::types::GLuint = gl.create_shader_program();
        let mut shader_ids: Vec<gl::types::GLuint> = vec![];
        for shader in shaders {
            gl.attach_shader(id, shader.get_id());
            shader_ids.push(shader.get_id());
        }
        gl.link_program(id);
        for shader in shaders {
            gl.detach_shader(id, shader.get_id());
        }

        ShaderProgram { id, shader_ids }
    }
}

impl ShaderProgram {
    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn get_shader_ids(&self) -> &Vec<gl::types::GLuint> {
        &self.shader_ids
    }
}
