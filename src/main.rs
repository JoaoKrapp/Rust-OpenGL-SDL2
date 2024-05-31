#![allow(warnings)]

use std::ffi::{CString};
use gl::types::{GLchar, GLsizei, GLuint, GLint};
use sdl2::event::Event;

use crate::windsdl::Winsdl;

extern crate gl;

mod windsdl;

mod graphics;

use graphics::{
    shader::*,
    resources::*,
    program::*,
    vbo::*,
    vao::*,
    ebo::*
};


const WIDTH : usize = 800;
const HEIGHT : usize = 600;



fn main() {

    // Sdl window
    let mut windsdl = Winsdl::new(WIDTH, HEIGHT).unwrap();

    // OpenGL context load
    let _gl_context = windsdl.window.gl_create_context().unwrap();

    // GL used in the program
    let gl = gl::Gl::load_with(|s| windsdl.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Load Shaders
    let vert_shader = Shader::from_vert_source(
        &gl,
      &CString::new(include_str!("../assets/shaders/triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("../assets/shaders/triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = Program::from_shaders(
        &gl,
        &[vert_shader, frag_shader]
    ).unwrap();

    // Load vertices
    let vertices: Vec<f32> = vec![
        // positions                        // colors

        // X   // Y  // Z
        -0.5,  -0.5 , 0.0,                  1.0, 0.0, 0.0,      // Lower left corner
        0.5,   -0.5 , 0.0,                  0.0, 1.0, 0.0,      // Lower right corner
        0.0,    0.5 , 0.0,                  0.0, 0.0, 1.0,      // Upper corner

        -0.25,  0.0 , 0.0,                  0.0, 0.5, 0.5,      // Inner left
         0.25,  0.0 , 0.0,                  0.5, 0.0, 0.5,      // Inner right
         0.0,  -0.5 , 0.0,                  0.5, 0.5, 0.0,      // Inner down

    ];

    let indices : Vec<GLuint> = vec![
      0, 3, 5,      // Lower left triangle
      3, 2, 4,      // Lower right triangle
      5, 4, 1       // Upper triangle
    ];

    let vao = VAO::new(&gl);
    vao.bind();

    let vbo = VBO::new(
        &gl,
        vertices
    );

    let ebo  = EBO::new(
        &gl,
        indices
    );

    // Link VAO to VBO

    // Triangle coordinates
    vao.link_attrib(
        &vbo,
        0,
        3,
        (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
        std::ptr::null()
    );

    // Triangle colors
    vao.link_attrib(
        &vbo,
        1,
        3,
        (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
        (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
    );

    // Unbind everything

    vao.unbind();
    vbo.unbind();
    ebo.unbind();

    let mut uni_id : GLint;

    unsafe {
        let name = CString::new(String::from("scale")).expect("CString::new failed");
        uni_id = gl.GetUniformLocation(shader_program.id(), name.as_ptr()) ;

    }



    unsafe {
        // Background color
        gl.Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }


    'running : loop {

        for event in windsdl.event_pump.poll_iter() {

            match event {
                Event::Quit { .. } => break 'running,
                _ => { }
            }

        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl.Uniform1f(uni_id, 1.5);

            gl.BindVertexArray(vao.id);
            gl.DrawElements(
                gl::TRIANGLES,
                9,
                gl::UNSIGNED_INT,
                0 as *const _

            );
        }

        windsdl.window.gl_swap_window();

    }
}
