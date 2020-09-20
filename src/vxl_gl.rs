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
