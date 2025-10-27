use std::ffi::CString;
use std::fs;
use std::ptr;
use nalgebra_glm as glm;

/// Manages a compiled and linked OpenGL shader program
pub struct Shader {
    pub id: u32,  // OpenGL program ID
}

impl Shader {
    /// Creates a new shader program from vertex and fragment shader files
    ///
    /// # Arguments
    /// * `vertex_path` - Path to vertex shader file (e.g., "shaders/basic.vert")
    /// * `fragment_path` - Path to fragment shader file (e.g., "shaders/basic.frag")
    ///
    /// # Panics
    /// Panics if shader files can't be read or shaders fail to compile/link
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        // Read shader source files
        let vertex_src = fs::read_to_string(vertex_path)
            .expect(&format!("Failed to read vertex shader: {}", vertex_path));

        let fragment_src = fs::read_to_string(fragment_path)
            .expect(&format!("Failed to read fragment shader: {}", fragment_path));

        unsafe {
            // Compile shaders
            let vertex_shader = Self::compile_shader(&vertex_src, gl::VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(&fragment_src, gl::FRAGMENT_SHADER);

            // Link program
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // Check for linking errors
            Self::check_link_errors(program);

            // Clean up individual shaders (no longer needed after linking)
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader { id: program }
        }
    }

    /// Activates this shader program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, matrix: &glm::Mat4) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                matrix.as_ptr(),
            );
        }
    }

    pub fn set_vec3(&self, name: &str, value: &glm::Vec3) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    /// Compiles a shader from source code
    ///
    /// Private helper function (no `pub` keyword)
    unsafe fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> u32 {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        Self::check_compile_errors(shader, shader_type);

        shader
    }

    /// Checks for shader compilation errors
    unsafe fn check_compile_errors(shader: u32, shader_type: gl::types::GLenum) {
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );

            let shader_type_str = if shader_type == gl::VERTEX_SHADER {
                "VERTEX"
            } else {
                "FRAGMENT"
            };

            panic!(
                "{} shader compilation failed:\n{}",
                shader_type_str,
                String::from_utf8_lossy(&buffer)
            );
        }
    }

    /// Checks for program linking errors
    unsafe fn check_link_errors(program: u32) {
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );

            panic!(
                "Shader program linking failed:\n{}",
                String::from_utf8_lossy(&buffer)
            );
        }
    }
}

// Cleanup when Shader is dropped (goes out of scope)
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

