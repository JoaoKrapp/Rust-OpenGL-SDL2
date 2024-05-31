use gl::types::{GLuint, GLint, GLsizei, GLvoid};
use crate::graphics::vbo::VBO;

pub struct VAO {
    gl : gl::Gl,
    pub id: GLuint,
}


impl VAO {
    pub fn new(gl : &gl::Gl) -> VAO{
        let mut vao : GLuint = 0;

        unsafe { gl.GenVertexArrays(1, &mut vao); }

        VAO{
            gl : gl.clone(),
            id : vao
        }

    }

    pub fn link_attrib(&self,
                       vbo : &VBO,
                       layout : GLuint,
                       size : GLint,
                       stride : GLsizei,
                       pointer : *const GLvoid){
        vbo.bind();
        unsafe {
            self.gl.EnableVertexAttribArray(layout);

            self.gl.VertexAttribPointer(
                layout,
                size,
                gl::FLOAT,
                gl::FALSE,
                stride,
                pointer
            );
        }
        vbo.unbind()


    }

    pub fn bind(&self){
        unsafe { self.gl.BindVertexArray(self.id); }
    }

    pub fn unbind(&self){
        unsafe { self.gl.BindVertexArray(0); }
    }

    pub fn delete(&self){
        unsafe { self.gl.DeleteVertexArrays(1, &self.id); }
    }
}