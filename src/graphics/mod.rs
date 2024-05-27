use std::ffi::{CStr, CString};
use gl::types::GLuint;

pub mod program;
pub mod shader;
pub mod resources;
pub mod vbo;
pub mod vao;
pub mod ebo;

/// Given a source of shader and the type returns the shader ID
fn shader_from_source(gl : &gl::Gl, source : &CStr, kind : gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id : GLuint = unsafe {
        gl.CreateShader(kind)
    };

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // Couldn't create a shader
    if success == 0 {
        // Create the variable that will be the length of the error message
        let mut len: gl::types::GLint = 0;

        // Pass the error length to the 'len' variable
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // Get CString with b' ' values of the size of the error
        let error: CString = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            // Put error of the shader into variable error
            gl.GetShaderInfoLog(
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
