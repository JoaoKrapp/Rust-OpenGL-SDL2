#![allow(warnings)]

use std::ffi::{CString};
use std::path::Path;
use gl::types::{GLsizei, GLuint, GLint};
use sdl2::event::Event;
use std::fs;
use image::io::Reader as ImageReader;

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
use crate::graphics::texture::Texture;


const WIDTH : usize = 600;
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


    let vertices: Vec<f32> = vec![
        // positions        // Color            // Texture
        -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 0.0,       // bottom left
        -0.5,  0.5, 0.0,    0.5, 0.5, 0.5,      0.0, 1.0,       // top left
         0.5,  0.5, 0.0,    1.0, 0.0, 0.0,      1.0, 1.0,       // top right
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0        // bottom right
    ];

    let indices: Vec<GLuint> = vec![0, 2, 1, 0, 3, 2];  // define the triangle indices

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
        (8 * std::mem::size_of::<f32>()) as GLint,
        std::ptr::null()
    );

    // Triangle colors
    vao.link_attrib(
        &vbo,
        1,
        3,
        (8 * std::mem::size_of::<f32>()) as GLint,
        (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
    );

    // Texture colors
    vao.link_attrib(
        &vbo,
        2,
        2,
        (8 * std::mem::size_of::<f32>()) as GLint,
        (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
    );

    // Unbind everything

    vao.unbind();
    vbo.unbind();
    ebo.unbind();

    // Texture
    let path = Path::new("./assets/textures/pop_cat.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap().flipv().to_rgba8();

    let texture = Texture::new(&gl, &img, gl::TEXTURE_2D);

    texture.tex_uniform(String::from("tex0"), &shader_program, 0);

    // Scale Uniform

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
            shader_program.set_used();

            // Scale
            gl.Uniform1f(uni_id, 1.0);

            // Texture
            texture.bind();

            gl.BindVertexArray(vao.id);
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const _

            );
        }

        windsdl.window.gl_swap_window();

    }
}
