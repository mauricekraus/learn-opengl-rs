use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    ptr, str,
};

use gl::types::{GLchar, GLint};
use nalgebra_glm as glm;

#[derive(Debug)]
enum CompilationType {
    Vertex,
    Fragment,
    Program,
}

pub trait Uniform {
    unsafe fn set_uniform(&self, id: u32);

    /// Queries the uniform location and returns the corresponding index
    unsafe fn get_uniform_loc(name: &CStr, id: u32) -> i32 {
        gl::GetUniformLocation(id, name.as_ptr())
    }
}

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_str: &str, fragment_str: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        let mut vert_shader_file =
            File::open(vertex_str).unwrap_or_else(|_| panic!("Failed to open {}", vertex_str));
        let mut frag_shader_file =
            File::open(fragment_str).unwrap_or_else(|_| panic!("Failed to open {}", fragment_str));
        //
        let mut vert_code = String::new();
        let mut frag_code = String::new();
        vert_shader_file
            .read_to_string(&mut vert_code)
            .expect("Failed to read vertex shader");
        frag_shader_file
            .read_to_string(&mut frag_code)
            .expect("Failed to read fragment shader");

        let c_vert = CString::new(vert_code.as_bytes()).unwrap();
        let c_frag = CString::new(frag_code.as_bytes()).unwrap();

        unsafe {
            let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vert_shader, 1, &c_vert.as_ptr(), ptr::null());
            gl::CompileShader(vert_shader);
            shader.check_compile_errors(vert_shader, &CompilationType::Vertex);

            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(frag_shader, 1, &c_frag.as_ptr(), ptr::null());
            gl::CompileShader(frag_shader);
            shader.check_compile_errors(frag_shader, &CompilationType::Fragment);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vert_shader);
            gl::AttachShader(id, frag_shader);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, &CompilationType::Program);

            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);

            shader.id = id;
        }

        shader
    }

    /// Activate the shader
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }
    #[allow(dead_code)]
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    #[allow(dead_code)]
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    #[allow(dead_code)]
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_matrix4(&self, name: &CStr, value: glm::Mat4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            value.as_ptr(),
        )
    }
    pub unsafe fn set_struct(&self, value: &impl Uniform) {
        value.set_uniform(self.id);
    }

    pub unsafe fn set_vec3(&self, name: &CStr, value: glm::Vec3) {
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        gl::Uniform3fv(location, 1, value.as_ptr());
    }

    // Utility function to check for compilation errors
    unsafe fn check_compile_errors(&self, shader: u32, comp_type: &CompilationType) {
        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = vec![0; 1024];
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character

        match comp_type {
            CompilationType::Vertex | CompilationType::Fragment => {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::SHADER::{:?}::COMPILATION_FAILED\n{}",
                        &comp_type,
                        str::from_utf8_unchecked(&info_log)
                    );
                }
            }
            CompilationType::Program => {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::SHADER::{:?}::LINKING_FAILED\n{}",
                        &comp_type,
                        str::from_utf8_unchecked(&info_log)
                    );
                }
            }
        }
    }
}
