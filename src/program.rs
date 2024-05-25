use std::ffi::{CString, CStr};

pub struct Program {
    id: gl::types::GLuint,
}


impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        // Create program
        let program_id = unsafe { gl::CreateProgram() };

        // For every shader in list given attach to the program
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        // Link the program to GL
        unsafe { gl::LinkProgram(program_id); }

        // Get status of linking the shader
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        // If it wasn't successful do error stuff
        if success == 0 {

            // Get length of error message
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            // Make error variable made of b' ' of length of error
            let error = create_whitespace_cstring_with_len(len as usize);

            // Put message into error variable
            unsafe {
                gl::GetProgramInfoLog(
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
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        // If everything went well returns the Program
        Ok(Program { id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

// ------------------------------------------

pub struct Shader {
    id: gl::types::GLuint,
}

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


/// Args:
///     source (&CStr)  : Information of the shader
///     Kind (GLuint)   : Type of shader
/// Returns:
///     shader (Result<GLuint, String>) : Return the ID of the shader created or a message of the error when creating the shader
fn shader_from_source(source : &CStr, kind : gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe {
        gl::CreateShader(kind)
    };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // Couldn't create a shader
    if success == 0 {
        // Create the variable that will be the length of the error message
        let mut len: gl::types::GLint = 0;

        // Pass the error length to the 'len' variable
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // Get CString with b' ' values of the size of the error
        let error: CString = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            // Put error of the shader into variable error
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        // Return error message
        return Err(error.to_string_lossy().into_owned());
    }

    // Returns the id of the shader
    return Ok(id)
}

/// Creates a buffer of type Vec<u8> filled with b' ' of the length given
/// Return the buffer as a CString
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Create a vector made of 8 bits unsigned integers
    // With capacity of len + 1
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

    // Put items of b' ' over the length of the error message
    buffer.extend([b' '].iter().cycle().take(len));

    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}