use std::ffi::CString;

use crate::vxl_gl::{gl, Gl};

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(gl: &Gl, source: &CString, shader_type: gl::types::GLenum) -> Shader {
        let id = gl.create_shader(shader_type);
        gl.compile_shader(id, source);

        Shader { id }
    }
}

impl Shader {
    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }
}
