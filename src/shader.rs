use std::ffi::CString;
use std::fs;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        let id;
        unsafe {
            let vertex_shader = Self::compile_shader(vertex_shader_path, gl::VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(fragment_shader_path, gl::FRAGMENT_SHADER);
            id = Self::link_program(vertex_shader, fragment_shader);
        }
        Self { id }
    }

    fn compile_shader(path: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &read_file(path).as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
            let mut compiled = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compiled);
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut info_log: Vec<u8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(
                shader,
                log_length,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as _,
            );
            if compiled == 0 {
                panic!(
                    "{} not compiled {}",
                    path,
                    String::from_utf8_unchecked(info_log)
                );
            }
        }
        shader
    }

    fn link_program(
        vertex_shader: gl::types::GLuint,
        fragment_shader: gl::types::GLuint,
    ) -> gl::types::GLuint {
        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            let mut compiled = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut compiled);
            let mut log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut info_log: Vec<u8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(
                program,
                log_length,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as _,
            );
            if compiled == 0 {
                panic!("Not linked {}", String::from_utf8_unchecked(info_log));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
        program
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

fn read_file(path: &str) -> CString {
    let file = fs::read(path).expect(&format!("{} should be present", path));
    let string;
    unsafe {
        string = CString::from_vec_unchecked(file);
    }
    string
}
