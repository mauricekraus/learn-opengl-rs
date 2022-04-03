extern crate glfw;

use std::{mem, os::raw::c_void, sync::mpsc::Receiver};

use crate::camera::{Camera, MovementEvent};
use crate::shader::Shader;

use self::glfw::{Action, Context, Key, Window};

extern crate gl;
// include the OpenGL type aliases
use gl::types::*;

use nalgebra_glm as glm;
use std::ffi::CStr;

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

pub fn main_2_2_1() {
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 3.0), false);

    let mut first_mouse = true;
    let mut last_x: f32 = SRC_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SRC_HEIGHT as f32 / 2.0;

    // timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

    let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw_instance.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw_instance.window_hint(glfw::WindowHint::ContextVersionMinor(6));
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
    window.set_cursor_pos_polling(true);
    window.set_size_polling(true);
    window.set_scroll_polling(true);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let (shader_cube, shader_light, vao_cube, vao_light) = unsafe {
        // Id generation
        let (mut vao_cube, mut vao_light, mut vbo) = (0_u32, 0_u32, 0_u32);
        gl::CreateVertexArrays(1, &mut vao_cube);
        gl::CreateVertexArrays(1, &mut vao_light);
        gl::CreateBuffers(1, &mut vbo);

        let shader_cube = Shader::new(
            "src/_2_lighting/shaders/2.1.basic_lighting.vert",
            "src/_2_lighting/shaders/2.1.basic_lighting.frag",
        );

        let shader_light = Shader::new(
            "src/_2_lighting/shaders/2.1.lamp.vert",
            "src/_2_lighting/shaders/2.1.lamp.frag",
        );

        let vertices: [f32; 216] = [
            // vert            Normal
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, // Vert
            0.5, -0.5, -0.5, 0.0, 0.0, -1.0, // Vert
            0.5, 0.5, -0.5, 0.0, 0.0, -1.0, // Vert
            0.5, 0.5, -0.5, 0.0, 0.0, -1.0, // Vert
            -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, // Vert
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, // Vert
            //
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, // Vert
            0.5, -0.5, 0.5, 0.0, 0.0, 1.0, // Vert
            0.5, 0.5, 0.5, 0.0, 0.0, 1.0, // Vert
            0.5, 0.5, 0.5, 0.0, 0.0, 1.0, // Vert
            -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, // Vert
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, // Vert
            //
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, // Vert
            -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, // Vert
            -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, // Vert
            -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, // Vert
            -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, // Vert
            -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, // Vert
            //
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, // Vert
            0.5, 0.5, -0.5, 1.0, 0.0, 0.0, // Vert
            0.5, -0.5, -0.5, 1.0, 0.0, 0.0, // Vert
            0.5, -0.5, -0.5, 1.0, 0.0, 0.0, // Vert
            0.5, -0.5, 0.5, 1.0, 0.0, 0.0, // Vert
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, // Vert
            //
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, // Vert
            0.5, -0.5, -0.5, 0.0, -1.0, 0.0, // Vert
            0.5, -0.5, 0.5, 0.0, -1.0, 0.0, // Vert
            0.5, -0.5, 0.5, 0.0, -1.0, 0.0, // Vert
            -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, // Vert
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, // Vert
            //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, // Vert
            0.5, 0.5, -0.5, 0.0, 1.0, 0.0, // Vert
            0.5, 0.5, 0.5, 0.0, 1.0, 0.0, // Vert
            0.5, 0.5, 0.5, 0.0, 1.0, 0.0, // Vert
            -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, // Vert
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, // Vert
        ];
        gl::NamedBufferData(
            vbo,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        // position
        gl::EnableVertexArrayAttrib(vao_cube, 0);
        gl::VertexArrayAttribBinding(vao_cube, 0, 0);
        gl::VertexArrayAttribFormat(vao_cube, 0, 3, gl::FLOAT, gl::FALSE, 0);

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexArrayVertexBuffer(vao_cube, 0, vbo, 0, stride);

        // normal
        gl::EnableVertexArrayAttrib(vao_cube, 1);
        gl::VertexArrayAttribBinding(vao_cube, 1, 0);
        gl::VertexArrayAttribFormat(
            vao_cube,
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as u32,
        );

        // light
        gl::EnableVertexArrayAttrib(vao_light, 0);
        gl::VertexArrayAttribBinding(vao_light, 0, 0);
        gl::VertexArrayAttribFormat(vao_light, 0, 3, gl::FLOAT, gl::FALSE, 0);

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexArrayVertexBuffer(vao_light, 0, vbo, 0, stride);

        shader_cube.use_program();

        (shader_cube, shader_light, vao_cube, vao_light)
    };

    // render loop
    while !window.should_close() {
        let current_frame = glfw_instance.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_events(
            &events,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
            &mut camera,
        );

        process_inputs(&mut window, delta_time, &mut camera);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader_cube.use_program();

            let projection_matrix = glm::perspective_rh(
                SRC_WIDTH as f32 / SRC_HEIGHT as f32,
                camera.zoom.to_radians(),
                0.1,
                100.0,
            );

            shader_cube.set_matrix4(c_str!("view"), camera.get_view_matrix());
            shader_cube.set_matrix4(c_str!("projection"), projection_matrix);
            gl::BindVertexArray(vao_cube);

            // cube
            let model_matrix = glm::Mat4::identity();
            shader_cube.set_matrix4(c_str!("model"), model_matrix);

            shader_cube.set_vec3(c_str!("objectColor"), glm::vec3(1.0, 0.5, 0.31));
            shader_cube.set_vec3(c_str!("lightColor"), glm::vec3(1.0, 1.0, 1.0));

            // light position

            let light_pos = glm::vec3(1.2, 3.0, 8.5);

            shader_cube.set_vec3(c_str!("lightPos"), light_pos);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // light cube
            shader_light.use_program();
            let mut model_matrix = glm::Mat4::identity();
            model_matrix = glm::scale(&model_matrix, &glm::vec3(0.2, 0.2, 0.2));
            model_matrix = glm::translate(&model_matrix, &light_pos);

            shader_light.set_matrix4(c_str!("model"), model_matrix);
            shader_light.set_matrix4(c_str!("view"), camera.get_view_matrix());
            shader_light.set_matrix4(c_str!("projection"), projection_matrix);

            gl::BindVertexArray(vao_light);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.swap_buffers();
        glfw_instance.poll_events();
    }
}

fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *first_mouse {
                    *last_x = xpos;
                    *last_y = ypos;
                    *first_mouse = false;
                }
                let x_offset = xpos - *last_x;
                let y_offset = *last_y - ypos; // reversed: y ranges bottom to top;
                *last_x = xpos;
                *last_y = ypos;

                camera.process_mouse_movement(x_offset, y_offset);
            }
            glfw::WindowEvent::Scroll(_, y_offset) => camera.process_mouse_scroll(y_offset as f32),
            _ => {}
        }
    }
}

fn process_inputs(window: &mut Window, delta_time: f32, camera: &mut Camera) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }
    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(MovementEvent::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(MovementEvent::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(MovementEvent::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(MovementEvent::Right, delta_time);
    }
}
