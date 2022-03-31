extern crate glfw;

use std::{
    ffi::{CStr, CString},
    mem,
    os::raw::c_void,
    sync::mpsc::Receiver,
};

use crate::shader::Shader;

use self::glfw::{Action, Context, Key, Window};

extern crate gl;
// include the OpenGL type aliases
use gl::types::*;

use std::ptr;

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

pub fn main_1_3_3() {
    let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw_instance.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw_instance.window_hint(glfw::WindowHint::ContextVersionMinor(2));
    glfw_instance.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw_instance
        .create_window(
            SRC_WIDTH,
            SRC_HEIGHT,
            "Learn OpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_size_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let shader = Shader::new(
        "src/_1_getting_started/shaders/3.3.shader.vs",
        "src/_1_getting_started/shaders/3.3.shader.fs",
    );

    let vba = unsafe {
        // Id generation
        let (mut vba, mut vbo) = (0_u32, 0_u32);
        gl::GenVertexArrays(1, &mut vba);
        gl::GenBuffers(1, &mut vbo);

        // bind VBA
        gl::BindVertexArray(vba);

        // Buffer setup

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // setup triangle
        let vertices: [f32; 18] = [
            // positions         // colors
            0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
        ];

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        vba
    };
    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_program();
            let u_name = CString::new("rCol".as_bytes()).unwrap();
            shader.set_float(&u_name, 0.4);

            gl::BindVertexArray(vba);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::BindVertexArray(0);
        }

        window.swap_buffers();
        glfw_instance.poll_events();
    }
}

fn process_events(window: &mut Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
