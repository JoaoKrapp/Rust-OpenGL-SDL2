use gl;
use gl::types::{GLuint, GLsizeiptr};

pub struct VBO {
    gl : gl::Gl,
    id: GLuint,
}

impl VBO {
    pub fn new(gl : &gl::Gl, vertices : Vec<f32>) -> VBO{
        let mut vbo : GLuint = 0;

        unsafe {
            gl.GenBuffers(1, &mut vbo);

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }

        VBO{
            gl : gl.clone(),
            id : vbo
        }
    }

    pub fn bind(&self){
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self){
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self){
        unsafe { self.gl.DeleteBuffers(1, &self.id); }
    }
}