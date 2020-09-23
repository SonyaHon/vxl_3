use crate::vxl_gl::{gl, Gl};

pub mod shader;
pub mod shader_program;

pub struct ShaderLoader {
    gl: &'static Gl,
    shader_ids: Vec<gl::types::GLuint>,
    program_ids: Vec<gl::types::GLuint>,
    programs: std::collections::HashMap<&'static str, shader_program::ShaderProgram>,
}

impl ShaderLoader {
    pub fn new(gl: &'static Gl) -> ShaderLoader {
        ShaderLoader {
            gl,
            shader_ids: vec![],
            program_ids: vec![],
            programs: std::collections::HashMap::new(),
        }
    }

    pub fn add_shader_program(
        &mut self,
        program_name: &'static str,
        shaders: Vec<(&'static str, gl::types::GLenum)>,
    ) {
    }

    pub fn get_shader_program(&self, program_name: &'static str) -> &shader_program::ShaderProgram {
        &self.programs.get(program_name).unwrap()
    }
}

impl Drop for ShaderLoader {
    fn drop(&mut self) {
        for shader in self.shader_ids {
            self.gl.drop_shader(shader);
        }

        for shader_program in self.program_ids {
            self.gl.drop_program(shader_program);
        }
    }
}
