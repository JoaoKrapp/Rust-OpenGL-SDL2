#![allow(warnings)]

use std::ffi::{CString};
use std::path::Path;
use gl::types::{GLsizei, GLuint, GLint};
use sdl2::event::Event;
use std::fs;
use std::time::{Duration, Instant};
use image::io::Reader as ImageReader;
use nalgebra::{Matrix4, Perspective3, Rotation3, Translation3, Vector3};

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
        -0.5, 0.0,  0.5,      0.83, 0.70, 0.44,	    0.0, 0.0,
        -0.5, 0.0, -0.5,      0.83, 0.70, 0.44,	    5.0, 0.0,
         0.5, 0.0, -0.5,      0.83, 0.70, 0.44,		0.0, 0.0,
         0.5, 0.0,  0.5,      0.83, 0.70, 0.44,		5.0, 0.0,
         0.0, 0.8,  0.0,      0.92, 0.86, 0.76,		2.5, 5.0
    ];

    let indices: Vec<GLuint> = vec![
        0, 1, 2,
        0, 2, 3,
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        3, 0, 4
    ];  // define the triangle indices

    let vao = VAO::new(&gl);
    vao.bind();

    let vbo = VBO::new(
        &gl,
        &vertices
    );

    let ebo  = EBO::new(
        &gl,
        &indices
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

    let scale_uniform_id : GLint = shader_program.get_uniform_id("scale");




    unsafe {
        // Background color
        gl.Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // 3d rotation
    let mut prev_time = Instant::now();
    let mut rotation = 0.0f32;

    unsafe { gl.Enable(gl::DEPTH_TEST); }

    'running : loop {

        for event in windsdl.event_pump.poll_iter() {

            match event {
                Event::Quit { .. } => break 'running,
                _ => { }
            }

        }



        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            shader_program.set_used();

            // 3D stuff

            let mut model: Matrix4<f32> = Matrix4::identity();
            let mut view: Matrix4<f32> = Matrix4::identity();
            let mut proj: Matrix4<f32> = Matrix4::identity();

            // Get current time in milliseconds
            let current_time = Instant::now();

            if current_time.duration_since(prev_time) >= Duration::from_millis(1000 / 60){
                rotation += 0.5;
                prev_time = current_time;
            }

            let rotation_radians = rotation.to_radians();
            let rotation_matrix = Rotation3::from_axis_angle(&Vector3::y_axis(), rotation_radians);
            model = model * rotation_matrix.to_homogeneous();

            let translation = Translation3::new(0.0, -0.5, -2.0);
            view = view * translation.to_homogeneous();

            let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
            let fov = 45.0f32.to_radians();
            proj = Perspective3::new(aspect_ratio, fov, 0.1, 100.0).to_homogeneous();

            let model_uniform_id = shader_program.get_uniform_id("model");
            gl.UniformMatrix4fv(model_uniform_id, 1, gl::FALSE, model.as_ptr());

            let view_uniform_id = shader_program.get_uniform_id("view");
            gl.UniformMatrix4fv(view_uniform_id, 1, gl::FALSE, view.as_ptr());

            let proj_uniform_id = shader_program.get_uniform_id("proj");
            gl.UniformMatrix4fv(proj_uniform_id, 1, gl::FALSE, proj.as_ptr());


            // Scale
            gl.Uniform1f(scale_uniform_id, 1.0);

            // Texture
            texture.bind();

            gl.BindVertexArray(vao.id);
            gl.DrawElements(
                gl::TRIANGLES,
                indices.len() as GLint,
                gl::UNSIGNED_INT,
                0 as *const _

            );
        }

        windsdl.window.gl_swap_window();

    }
}

