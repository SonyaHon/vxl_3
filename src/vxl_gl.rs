use std::ffi::CStr;

use cgmath::{vec3, Vector3};
use glutin::{self, PossiblyCurrent};

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
    pub clear_color: Vector3<f32>,
}

pub fn load(gl_context: &glutin::Context<PossiblyCurrent>) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    Gl {
        gl,
        clear_color: vec3(0.2, 0.2, 0.2),
    }
}

impl Gl {
    pub fn clear_screen(&self) {
        unsafe {
            self.gl.ClearColor(
                self.clear_color.x,
                self.clear_color.y,
                self.clear_color.z,
                1.0,
            );
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Gl {
    pub fn create_vao(&self) -> gl::types::GLuint {
        let mut vao: gl::types::GLuint = 0;
        unsafe { self.gl.GenVertexArrays(1, &mut vao) };
        vao
    }

    pub fn create_vertex_vbo(
        &self,
        vao_id: gl::types::GLuint,
        vertices: Vec<cgmath::Vector3<f32>>,
    ) -> gl::types::GLuint {
        let mut vbo: gl::types::GLuint = 0;
        unsafe { self.gl.GenBuffers(1, &mut vbo) };
        unsafe {
            self.gl.BindVertexArray(vao_id);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            let mut nverts = Vec::new();
            vertices.iter().for_each(|vertex| {
                nverts.push(vertex.x);
                nverts.push(vertex.y);
                nverts.push(vertex.z);
            });
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (nverts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                nverts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            self.gl.EnableVertexAttribArray(0);
            self.gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        };

        self.unbind_vao();

        vbo
    }

    pub fn create_index_vbo(
        &self,
        vao_id: gl::types::GLuint,
        indices: Vec<i32>,
    ) -> gl::types::GLuint {
        let mut vbo: gl::types::GLuint = 0;
        unsafe { self.gl.GenBuffers(1, &mut vbo) };
        unsafe {
            self.gl.BindVertexArray(vao_id);
            self.gl.GenBuffers(1, &mut vbo);
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        };
        self.unbind_vao();

        vbo
    }

    fn unbind_vao(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}
