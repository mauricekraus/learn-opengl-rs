use std::ffi::CStr;

use nalgebra_glm::Vec3;

use crate::shader::Uniform;

pub struct Material {
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    shininess: f32,
}

impl Uniform for Material {
    unsafe fn set_uniform(&self, id: u32) {
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("material.ambient"), id),
            1,
            self.ambient.as_ptr(),
        );
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("material.diffuse"), id),
            1,
            self.diffuse.as_ptr(),
        );
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("material.specular"), id),
            1,
            self.specular.as_ptr(),
        );
        gl::Uniform1f(
            Self::get_uniform_loc(c_str!("material.shininess"), id),
            self.shininess,
        );
    }
}

impl Material {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Material {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}
