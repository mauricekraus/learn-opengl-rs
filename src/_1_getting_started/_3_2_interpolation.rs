extern crate glfw;

use std::{ffi::CString, mem, os::raw::c_void, sync::mpsc::Receiver};

use self::glfw::{Action, Context, Key, Window};

extern crate gl;
// include the OpenGL type aliases
use gl::types::*;

use std::ptr;
use std::str;
const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

const VERTEX_SHADER_SRC: &str = "\
        #version 420 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        
        out vec3 ourColor;
        void main() 
        {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);        
            ourColor = aColor;
        }
        ";

const FRAGMENT_SHADER_SRC: &str = "\
        #version 420 core
        out vec4 FragColor;

        in vec3 ourColor;

        void main() 
        {
            FragColor = vec4(ourColor, 1.0); 
        }
        ";

pub fn main_1_3_2() {
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

    let (shader_prog, vba) = unsafe {
        let vert = create_shader(ShaderType::Vertex);
        let frag = create_shader(ShaderType::Fragment);

        let shader_prog = gl::CreateProgram();
        gl::AttachShader(shader_prog, vert);
        gl::AttachShader(shader_prog, frag);
        gl::LinkProgram(shader_prog);
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0; 512];
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramiv(shader_prog, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                shader_prog,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::LINKING_FAILED\n{}",
                str::from_utf8_unchecked(&info_log)
            );
        }

        gl::DeleteShader(vert);
        gl::DeleteShader(frag);

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

        (shader_prog, vba)
    };
    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_prog);

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

#[derive(Debug)]
enum ShaderType {
    Vertex,
    Fragment,
}

unsafe fn create_shader(shader_type: ShaderType) -> u32 {
    let (shader, c_str_shader) = match &shader_type {
        ShaderType::Vertex => {
            let shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_shader = CString::new(VERTEX_SHADER_SRC.as_bytes()).unwrap();
            (shader, c_str_shader)
        }
        ShaderType::Fragment => {
            let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_shader = CString::new(FRAGMENT_SHADER_SRC.as_bytes()).unwrap();
            (shader, c_str_shader)
        }
    };
    gl::ShaderSource(shader, 1, &c_str_shader.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut success = gl::FALSE as GLint;
    let mut info_log: Vec<u8> = vec![0; 1024];
    info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::{:?}::COMPILATION_FAILED\n{}",
            &shader_type,
            str::from_utf8(&info_log).unwrap()
        );
    }
    shader
}
