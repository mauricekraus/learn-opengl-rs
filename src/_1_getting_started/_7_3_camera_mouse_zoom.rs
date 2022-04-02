extern crate glfw;

use std::{mem, os::raw::c_void, path::Path, ptr, sync::mpsc::Receiver};

use crate::shader::Shader;

use self::glfw::{Action, Context, Key, Window};

extern crate gl;
// include the OpenGL type aliases
use gl::types::*;

use nalgebra_glm as glm;
use std::ffi::CStr;

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

const CAMERA_UP: glm::Vec3 = glm::Vec3::new(0.0, 1.0, 0.0);

pub fn main_1_7_3() {
    let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
    let mut camera_front: glm::Vec3 = glm::Vec3::new(0.0, 0.0, -1.0);
    let mut first_mouse = true;
    let mut yaw: f32 = -90.0; // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
    let mut pitch: f32 = 0.0;
    let mut last_x: f32 = SRC_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SRC_HEIGHT as f32 / 2.0;
    let mut fov = 45.0_f32;

    // timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

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
    window.set_cursor_pos_polling(true);
    window.set_size_polling(true);
    window.set_scroll_polling(true);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let (shader, vao, texture1, texture2) = unsafe {
        // Id generation
        let (mut vao, mut vbo) = (0_u32, 0_u32);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        let shader = Shader::new(
            "src/_1_getting_started/shaders/6.2.coords_depth.vs",
            "src/_1_getting_started/shaders/6.2.coords_depth.fs",
        );
        // bind VBA
        gl::BindVertexArray(vao);

        // Buffer setup

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
            0.5, -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5,
            0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0,
            1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5,
            -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5,
            0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0,
            1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
            0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5,
            0.0, 1.0,
        ];
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        // tex attribute
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        let (mut texture1, mut texture2) = (0, 0);
        gl::GenTextures(1, &mut texture1);
        gl::GenTextures(1, &mut texture2);

        gl::BindTexture(gl::TEXTURE_2D, texture1);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::open(&Path::new("resources/textures/container.jpg"))
            .expect("Failed to load texture");
        let data = img.to_rgb8().into_raw();
        // create texture
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindTexture(gl::TEXTURE_2D, texture2);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::open(&Path::new("resources/textures/awesomeface.png"))
            .expect("Failed to load texture");
        let img = img.flipv();
        let data = img.to_rgba8().into_raw();
        // create texture
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);
        shader.set_int(c_str!("texture2"), 1);

        (shader, vao, texture1, texture2)
    };
    let cube_positions: [glm::Vec3; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

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
            &mut yaw,
            &mut pitch,
            &mut camera_front,
            &mut fov,
        );

        process_inputs(&mut window, delta_time, &mut camera_pos, &camera_front);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader.use_program();

            let view_matrix = glm::look_at(&camera_pos, &(camera_pos + camera_front), &CAMERA_UP);

            let projection_matrix = glm::perspective_rh(
                SRC_WIDTH as f32 / SRC_HEIGHT as f32,
                fov.to_radians(),
                0.1,
                100.0,
            );

            shader.set_matrix4(c_str!("view"), view_matrix);
            shader.set_matrix4(c_str!("projection"), projection_matrix);
            gl::BindVertexArray(vao);
            for (i, pos) in cube_positions.iter().enumerate() {
                let mut model_matrix = glm::Mat4::identity();
                model_matrix = glm::translate(&model_matrix, pos);
                let angle: f32 = 20.0 * i as f32;
                model_matrix =
                    glm::rotate(&model_matrix, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
                shader.set_matrix4(c_str!("model"), model_matrix);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
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
    yaw: &mut f32,
    pitch: &mut f32,
    camera_front: &mut glm::Vec3,
    fov: &mut f32,
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
                let mut x_offset = xpos - *last_x;
                let mut y_offset = *last_y - ypos; // reversed: y ranges bottom to top;
                *last_x = xpos;
                *last_y = ypos;

                let sensitivity = 0.1;
                x_offset *= sensitivity;
                y_offset *= sensitivity;

                *yaw += x_offset;
                *pitch += y_offset;

                *pitch = pitch.clamp(-89.0, 89.0);

                let direction = glm::vec3(
                    yaw.to_radians().cos() * pitch.to_radians().cos(),
                    pitch.to_radians().sin(),
                    yaw.to_radians().sin() * pitch.to_radians().cos(),
                );

                *camera_front = direction.normalize();
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                *fov = (*fov - yoffset as f32).clamp(1.0, 45.0);
            }
            _ => {}
        }
    }
}

fn process_inputs(
    window: &mut Window,
    delta_time: f32,
    camera_pos: &mut glm::Vec3,
    camera_front: &glm::Vec3,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }
    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_speed * camera_front;
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos += -(camera_speed * camera_front);
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos += -(camera_front.cross(&CAMERA_UP).normalize() * camera_speed);
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += camera_front.cross(&CAMERA_UP).normalize() * camera_speed;
    }
}
