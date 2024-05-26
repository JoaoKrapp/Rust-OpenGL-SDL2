use std::ffi::{CStr, CString};
use crate::graphics::{Shader, shader_from_source};

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_vert_source(source : &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source : &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}


