use std::ffi::{CStr, CString};
use gl::types::GLsizei;
use sdl2::event::Event;
use crate::program::{Program, Shader};
use crate::windsdl::Winsdl;


mod windsdl;
mod program;


const WIDTH : usize = 800;
const HEIGHT : usize = 600;


fn main() {

    // Sdl window
    let mut windsdl = Winsdl::new(WIDTH, HEIGHT).unwrap();

    // OpenGL context load
    let _gl_context = windsdl.window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| windsdl.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Load Shaders
    let vert_shader = Shader::from_vert_source(
      &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    // Load objects
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    // Vertex Buffer Object
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                               // What the data is
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,    // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid,                                  // pointer to data
            gl::STATIC_DRAW,                                                                // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    // Create VAO (Vertex Array Object)
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);           // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,                         // index of the generic vertex attribute ("layout (location = 0)")
            3,                          // the number of components per generic vertex attribute
            gl::FLOAT,                       // data type
            gl::FALSE,                       // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    'running : loop {

        for event in windsdl.event_pump.poll_iter() {

            match event {
                Event::Quit { .. } => break 'running,
                _ => { }
            }

        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        windsdl.window.gl_swap_window();

    }
}
