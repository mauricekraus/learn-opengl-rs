use std::ffi::CStr;

use crate::shader::Uniform;
use nalgebra_glm::Vec3;

pub struct Light {
    position: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

impl Uniform for Light {
    unsafe fn set_uniform(&self, id: u32) {
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("light.position"), id),
            1,
            self.position.as_ptr(),
        );
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("light.ambient"), id),
            1,
            self.ambient.as_ptr(),
        );
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("light.diffuse"), id),
            1,
            self.diffuse.as_ptr(),
        );
        gl::Uniform3fv(
            Self::get_uniform_loc(c_str!("light.specular"), id),
            1,
            self.specular.as_ptr(),
        );
    }
}

impl Light {
    pub fn new(position: Vec3, ambient: Vec3, diffuse: Vec3, specular: Vec3) -> Light {
        Self {
            position,
            ambient,
            diffuse,
            specular,
        }
    }
}
