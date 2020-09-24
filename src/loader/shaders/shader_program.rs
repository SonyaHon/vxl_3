use crate::vxl_gl::{gl, Gl};

use super::shader::Shader;

pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn from_shaders(gl: &Gl, shaders: &Vec<Shader>) -> ShaderProgram {
        let id: gl::types::GLuint = gl.create_shader_program();
        for shader in shaders {
            gl.attach_shader(id, shader.get_id());
        }
        gl.link_program(id);
        for shader in shaders {
            gl.detach_shader(id, shader.get_id());
        }

        ShaderProgram { id }
    }
}

impl ShaderProgram {
    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl From<&ShaderProgram> for ShaderProgram {
    fn from(val: &ShaderProgram) -> Self {
        ShaderProgram { id: val.get_id() }
    }
}
