use std::ffi::{CStr};
use gl::types::GLuint;

use crate::graphics::shader_from_source;
use crate::Resources;

pub struct Shader {
    gl : gl::Gl,
    id: GLuint,
}

impl Shader {
    pub fn from_source(gl : &gl::Gl, source : &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { gl : gl.clone() ,id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn from_vert_source(gl : &gl::Gl , source : &CStr) -> Result<Shader, String>{
        Shader::from_source(gl ,source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl : &gl::Gl ,source : &CStr) -> Result<Shader, String>{
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, String> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| format!("Can not determine shader type for resource {}", name))?;

        let source = res.load_cstring(name)
            .map_err(|e| format!("Error loading resource {}: {:?}", name, e))?;

        Shader::from_source(gl, &source, shader_kind)
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}




