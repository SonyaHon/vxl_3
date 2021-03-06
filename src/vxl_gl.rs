use std::ffi::{CStr, CString};

use crate::utils::create_whitespace_csting_with_len;
use cgmath::prelude::*;
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

/// General
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

/// VAOs
impl Gl {
    pub fn create_vao(&self) -> gl::types::GLuint {
        let mut vao: gl::types::GLuint = 0;
        unsafe { self.gl.GenVertexArrays(1, &mut vao) };
        vao
    }

    pub fn unbind_vao(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }

    pub fn bind_vao(&self, vao_id: gl::types::GLuint) {
        unsafe {
            self.gl.BindVertexArray(vao_id);
        }
    }

    pub fn enable_vertex_attrib_arrays(&self, attribs: &Vec<gl::types::GLuint>) {
        unsafe {
            attribs.iter().for_each(|attrib| {
                let index = *attrib;
                self.gl.EnableVertexAttribArray(index);
            });
        }
    }

    pub fn disable_vertex_attrib_arrays(&self, attribs: &Vec<gl::types::GLuint>) {
        unsafe {
            attribs
                .iter()
                .for_each(|attrib| self.gl.DisableVertexAttribArray(*attrib));
        }
    }
}

/// VBOs
impl Gl {
    pub fn create_vertex_vbo(&self, vertices: Vec<cgmath::Vector3<f32>>) -> gl::types::GLuint {
        let mut vbo: gl::types::GLuint = 0;
        unsafe { self.gl.GenBuffers(1, &mut vbo) };
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            let mut nverts: Vec<f32> = Vec::new();
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
            self.gl.DisableVertexAttribArray(0);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        };

        vbo
    }

    pub fn create_index_vbo(&self, indices: Vec<u32>) -> gl::types::GLuint {
        let mut vbo: gl::types::GLuint = 0;
        unsafe { self.gl.GenBuffers(1, &mut vbo) };
        unsafe {
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        };

        vbo
    }

    pub fn create_uvs_vbo(&self, uvs: Vec<cgmath::Vector2<f32>>) -> gl::types::GLuint {
        let mut vbo: gl::types::GLuint = 0;

        let mut nverts: Vec<f32> = Vec::new();
        uvs.iter().for_each(|uv_vec| {
            nverts.push(uv_vec.x);
            nverts.push(uv_vec.y);
        });

        unsafe { self.gl.GenBuffers(1, &mut vbo) };
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (nverts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                nverts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            self.gl.EnableVertexAttribArray(1);
            self.gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            self.gl.DisableVertexAttribArray(1);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        vbo
    }
}

/// Shaders
impl Gl {
    pub fn create_shader(&self, shader_type: gl::types::GLenum) -> gl::types::GLuint {
        unsafe { self.gl.CreateShader(shader_type) }
    }

    pub fn compile_shader(&self, shader_id: gl::types::GLuint, shader_source: &CString) {
        unsafe {
            self.gl
                .ShaderSource(shader_id, 1, &shader_source.as_ptr(), std::ptr::null());
            self.gl.CompileShader(shader_id);
        }

        let mut result: gl::types::GLint = 1;
        unsafe {
            self.gl
                .GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut result)
        };

        if result == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                self.gl
                    .GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len)
            };

            let error: CString = create_whitespace_csting_with_len(len as usize);

            unsafe {
                self.gl.GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            };

            println!("Error: {:?}", error);
            panic!(error);
        }
    }

    pub fn drop_shader(&self, shader_id: gl::types::GLuint) {
        unsafe {
            self.gl.DeleteShader(shader_id);
        }
    }
}

/// Shader Programs
impl Gl {
    pub fn create_shader_program(&self) -> gl::types::GLuint {
        unsafe { self.gl.CreateProgram() }
    }

    pub fn attach_shader(&self, program_id: gl::types::GLuint, shader_id: gl::types::GLuint) {
        unsafe { self.gl.AttachShader(program_id, shader_id) };
    }

    pub fn link_program(&self, program_id: gl::types::GLuint) {
        unsafe { self.gl.LinkProgram(program_id) };

        let mut result: gl::types::GLint = 1;
        unsafe {
            self.gl
                .GetProgramiv(program_id, gl::LINK_STATUS, &mut result)
        };

        if result == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                self.gl
                    .GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len)
            };
            let error = create_whitespace_csting_with_len(len as usize);

            unsafe {
                self.gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            panic!(error);
        }
    }

    pub fn detach_shader(&self, program_id: gl::types::GLuint, shader_id: gl::types::GLuint) {
        unsafe { self.gl.DetachShader(program_id, shader_id) };
    }

    pub fn drop_program(&self, program_id: gl::types::GLuint) {
        unsafe {
            self.gl.DeleteProgram(program_id);
        }
    }

    pub fn bind_program(&self, program_id: gl::types::GLuint) {
        unsafe {
            self.gl.UseProgram(program_id);
        }
    }

    pub fn unbind_program(&self) {
        unsafe {
            self.gl.UseProgram(0);
        }
    }

    pub fn get_uniform_location(&self, program_id: gl::types::GLuint, location_name: &str) -> i32 {
        unsafe {
            self.gl.GetUniformLocation(
                program_id,
                location_name.to_owned().to_string().as_ptr() as *const i8,
            )
        }
    }

    pub fn add_uniform_matrix4f(&self, location: i32, matrix: cgmath::Matrix4<f32>) {
        unsafe {
            self.gl
                .UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }
}

impl Gl {
    pub fn draw_elements(&self, vertex_count: i32) {
        unsafe {
            self.gl.DrawElements(
                gl::TRIANGLES,
                vertex_count as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

impl Gl {
    pub fn create_texture(&self) -> gl::types::GLuint {
        let mut tex_id: gl::types::GLuint = 0;
        unsafe { self.gl.GenTextures(1, &mut tex_id) };
        tex_id
    }

    pub fn bind_texture(&self, texture_id: gl::types::GLuint) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }

    pub fn unbind_texture(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn generate_mipmap(&self) {
        unsafe {
            self.gl.GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn set_texture_data(&self, dimensions: cgmath::Vector2<u32>, texture_data: Vec<u8>) {
        unsafe {
            self.gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                dimensions.x as i32,
                dimensions.y as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &texture_data[0] as *const u8 as *const std::ffi::c_void,
            )
        }
    }

    pub fn set_active_texture(&self) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
        }
    }
}

impl Gl {
    pub fn print_error(&self) {
        unsafe {
            let errno = self.gl.GetError();
            if errno != 0 {
                println!("GL Error: {:?}", errno);
            }
        }
    }
}
