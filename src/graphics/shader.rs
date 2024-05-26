use std::ffi::{CStr, CString};
use crate::graphics::{Shader, shader_from_source};

impl Shader {
    pub fn from_source(gl : &gl::Gl, source : &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { gl : gl.clone() ,id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_vert_source(gl : &gl::Gl , source : &CStr) -> Result<Shader, String>{
        Shader::from_source(gl ,source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl : &gl::Gl ,source : &CStr) -> Result<Shader, String>{
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}


