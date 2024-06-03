use std::ffi::CString;
use gl;
use gl::types::{GLint, GLuint};

use crate::graphics::{
    create_whitespace_cstring_with_len,
};

use crate::Shader;

pub struct Program {
    gl : gl::Gl,
    id: GLuint,
}

impl Program {
    pub fn from_shaders(gl : &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        // Create program
        let program_id = unsafe { gl.CreateProgram() };

        // For every shader in list given attach to the program
        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()); }
        }

        // Link the program to GL
        unsafe { gl.LinkProgram(program_id); }

        // Get status of linking the shader
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        // If it wasn't successful do error stuff
        if success == 0 {

            // Get length of error message
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            // Make error variable made of b' ' of length of error
            let error = create_whitespace_cstring_with_len(len as usize);

            // Put message into error variable
            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            // Return error
            return Err(error.to_string_lossy().into_owned());
        }

        // After linking the program without errors, detach from the program
        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()); }
        }

        // If everything went well returns the Program
        Ok(Program { gl : gl.clone(),  id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn activate(&self){
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn get_uniform_id(&self, name : &str) -> GLint{
        let mut uni_id : GLint;
        unsafe {
            let name = CString::new(String::from(name)).expect("CString::new failed");
            uni_id = self.gl.GetUniformLocation(self.id, name.as_ptr());
        }
        uni_id
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}
