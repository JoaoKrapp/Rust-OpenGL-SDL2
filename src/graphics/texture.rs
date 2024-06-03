use std::ffi::CString;
use std::path::Path;
use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLuint};

use image::io::Reader as ImageReader;
use image::RgbaImage;
use crate::graphics::program::Program;

pub struct Texture {
    gl : gl::Gl,
    pub id: GLuint,
    pub texture_type : GLenum,
}

impl Texture {
    pub fn new(gl : &gl::Gl, img: &RgbaImage, texture_type : GLenum) -> Texture {
        // let img = ImageReader::open(file).unwrap().decode().unwrap().flipv().to_rgba8();
        let mut texture_id = 0;

        unsafe {
            gl.GenTextures(1, &mut texture_id);
            gl.BindTexture(gl::TEXTURE_2D, texture_id);

            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_raw().as_ptr() as *const _
            );

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            gl : gl.clone(),
            id : texture_id,
            texture_type,

        }

    }

    pub fn tex_uniform(&self, uniform_name : String, shader_program : &Program, value : GLint){
        unsafe {
            let name = CString::new(uniform_name).expect("CString::new failed");
            let uni_texture_id = self.gl.GetUniformLocation(shader_program.id(), name.as_ptr()) ;
            shader_program.activate();
            self.gl.Uniform1i(uni_texture_id, value);
        }
    }

    pub fn bind(&self){
        unsafe {
            self.gl.BindTexture(self.texture_type, self.id);
        }
    }

    pub fn unbind(&self){
        unsafe {
            self.gl.BindTexture(self.texture_type, 0);
        }
    }

    pub fn delete(&self){
        unsafe {
            self.gl.DeleteTextures(1, &self.id);
        }
    }
}