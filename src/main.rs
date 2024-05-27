#![allow(warnings)]

use std::ffi::{CString};
use gl::types::{GLsizei, GLuint};
use sdl2::event::Event;

use crate::windsdl::Winsdl;

extern crate gl;

mod windsdl;

mod graphics;

use graphics::{
    shader::*,
    resources::*,
    program::*
};


const WIDTH : usize = 800;
const HEIGHT : usize = 600;



fn main() {

    let sqrt_3: f32 = f32::sqrt(3.0);

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
        // positions                                            // colors

        // X   // Y  // Z
        -0.5,  -0.5 , 0.0,                  1.0, 0.0, 0.0,      // Lower left corner
        0.5,   -0.5 , 0.0,                  0.0, 1.0, 0.0,      // Lower right corner
        0.0,    0.5 , 0.0,                  0.0, 0.0, 1.0,      // Upper corner

        -0.25,  0.0 , 0.0,                 0.0, 0.5, 0.5,      // Inner left
         0.25,  0.0 , 0.0,                 0.5, 0.0, 0.5,      // Inner right
         0.0,  -0.5 , 0.0,                 0.5, 0.5, 0.0,      // Inner down

    ];

    let indices : Vec<GLuint> = vec![
      0, 3, 5,      // Lower left triangle
      3, 2, 4,      // Lower right triangle
      5, 4, 1       // Upper triangle
    ];

    let mut vao: GLuint = 0;
    let mut vbo : GLuint = 0;
    let mut ebo : GLuint = 0;

    unsafe {


        // Generate the VAO, VBO, and EBO with only 1 object each
        gl.GenVertexArrays(1, &mut vao);
        gl.GenBuffers(1, &mut vbo);
        gl.GenBuffers(1, &mut ebo);

        // Use the VAO now

        gl.BindVertexArray(vao);

        // VBO stuff

        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );

        // EBO Stuff

        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );

        // VAO Stuff

        // Code Coordinates
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        // Color Coordinates
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        // Unbind everything
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);            // VBO
        gl.BindVertexArray(0);                          // VAO
        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);    // EBO

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
            gl.BindVertexArray(vao);
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
