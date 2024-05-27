use gl::types::GLuint;

pub struct EBO {
    gl : gl::Gl,
    id: GLuint,
}

impl EBO{
    pub fn new(gl : &gl::Gl, indices : Vec<GLuint>) -> EBO{
        let mut ebo: GLuint = 0;

        unsafe {
            gl.GenBuffers(1, &mut ebo);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }

        EBO {
            gl : gl.clone(),
            id : ebo
        }
    }

    pub fn bind(&self){
        unsafe { self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self){
        unsafe { self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self){
        unsafe { self.gl.DeleteBuffers(1, &self.id); }
    }
}