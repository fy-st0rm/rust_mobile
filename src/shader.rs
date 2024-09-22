use std::ffi::{ CStr, CString };
use std::ptr;
use gl;
use gl::types::*;

pub struct Shader {
	pub id: GLuint
}

impl Shader {
	pub fn from_src(v_src: &str, f_src: &str) -> Result<Shader, String> {
		let v_shader = Self::compile_shader(v_src, gl::VERTEX_SHADER)?;
		let f_shader = Self::compile_shader(f_src, gl::FRAGMENT_SHADER)?;
		let shader = Self::link_program(v_shader, f_shader)?;

		// Cleaning the shader
		unsafe {
			gl::DeleteShader(v_shader);
			gl::DeleteShader(f_shader);
		}

		Ok(Shader {
			id: shader
		})
	}

	pub fn bind(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}

	pub fn delete(&self) {
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}

	fn compile_shader(src: &str, shader_type: GLenum) -> Result<GLuint, String> {
		let shader;
		unsafe {
			shader = gl::CreateShader(shader_type);

			// Converting to c string for opengl functions
			let c_str = CString::new(src.as_bytes()).unwrap();

			// Compiling the source
			gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
			gl::CompileShader(shader);

			// Check for error
			let mut sucess = gl::FALSE as GLint;
			let mut info_log = vec![0; 512];

			// Getting the compiler status
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut sucess);

			if sucess != gl::TRUE as GLint {
				// Getting the error log
				gl::GetShaderInfoLog(
					shader, 512, ptr::null_mut(),
					info_log.as_mut_ptr() as *mut GLchar
				);
				let err = format!(
					"ERROR::SHADER::COMPILATION_FAILED\n{}",
					CStr::from_ptr(info_log.as_ptr() as *const u8).to_str().unwrap()
				);
				return Err(err);
			}
		}
		Ok(shader)
	}

	fn link_program(v_shader: u32, f_shader: u32) -> Result<GLuint, String> {
		let program;
		unsafe {
			// Attaching both of the shader in single program
			program = gl::CreateProgram();
			gl::AttachShader(program, v_shader);
			gl::AttachShader(program, f_shader);
			gl::LinkProgram(program);

			// Check for linking errors
			let mut success = gl::FALSE as GLint;
			let mut info_log = vec![0; 512];
			gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
			if success != gl::TRUE as GLint {
				gl::GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
				let err = format!(
				"ERROR::PROGRAM::LINKING_FAILED\n{}",
					CStr::from_ptr(info_log.as_ptr() as *const u8).to_str().unwrap()
				);
				return Err(err);
			}
		}
		Ok(program)
	}
}
