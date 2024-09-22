use crate::math::vec::*;
use crate::shader::Shader;
use crate::static_assert;

use gl;
use gl::types::*;
use std::ptr;
use std::mem::{ size_of, offset_of };
use std::ffi::CString;

/*
const V_DEF_SHADER_SRC: &str = r#"
	#version 440 core
	layout (location = 0) in vec3 pos;
	layout (location = 1) in vec4 color;

	out vec4 o_color;

	void main() {
		o_color = color;
		gl_Position = vec4(pos, 1.0);
	}
"#;

const F_DEF_SHADER_SRC: &str = r#"
	#version 440 core
	layout (location = 0) out vec4 color;

	in vec4 o_color;

	void main() {
		color = o_color;
	}
"#;
*/

const V_DEF_SHADER_SRC: &str = r#"
    #version 100
    attribute vec3 pos;
    attribute vec4 color;

    varying vec4 o_color;

    void main() {
        o_color = color;
        gl_Position = vec4(pos, 1.0);
    }
"#;

const F_DEF_SHADER_SRC: &str = r#"
    #version 100
    precision mediump float;

    varying vec4 o_color;

    void main() {
        gl_FragColor = o_color;
    }
"#;


#[derive(Debug)]
pub struct Vertex {
	pub pos: Vec3,
	pub color: Vec4,
}

pub struct Renderer {
	vao: GLuint,
	vbo: GLuint,
	buffer: Vec<Vertex>,
	buff_idx: i32,
	shader: Shader,
}

// Renderer configs
const TEXTURE_SAMPLE_AMT: i32 =  32;
const VERTEX_SIZE : i32       = 7;
const MAX_VERT_CNT: i32       = 10000;
const MAX_BUFF_CAP: i32       = MAX_VERT_CNT  * VERTEX_SIZE;
const MAX_VBO_SIZE: usize     = MAX_BUFF_CAP as usize * size_of::<f32>();

static_assert!(
	VERTEX_SIZE as usize == size_of::<Vertex>() / size_of::<f32>(),
	"Size of vertex missmatched"
);

impl Renderer {
	pub fn new() -> Result<Renderer, String> {
		let mut vao = 0;
		let mut vbo = 0;
		let buffer: Vec<Vertex> = Vec::new();
		let shader = Shader::from_src(V_DEF_SHADER_SRC, F_DEF_SHADER_SRC)?;

		unsafe {
			// gl::Enable(gl::DEPTH_TEST);

			// VAO
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);

			// VBO
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				MAX_VBO_SIZE as GLsizeiptr,
				ptr::null(),
				gl::DYNAMIC_DRAW
			);

			// Setting up vertex format
			let pos_attr = gl::GetAttribLocation(shader.id, CString::new("pos").unwrap().as_ptr());
			gl::VertexAttribPointer(
				pos_attr as GLuint, 3, gl::FLOAT, gl::FALSE,
				size_of::<Vertex>() as GLsizei,
				offset_of!(Vertex, pos) as *const GLvoid
			);
			gl::EnableVertexAttribArray(pos_attr as GLuint);

			let color_attr = gl::GetAttribLocation(shader.id, CString::new("color").unwrap().as_ptr());
			gl::VertexAttribPointer(
				color_attr as GLuint, 4, gl::FLOAT, gl::FALSE,
				size_of::<Vertex>() as GLsizei,
				offset_of!(Vertex, color) as *const GLvoid
			);
			gl::EnableVertexAttribArray(color_attr as GLuint);

			// Unbinding
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		Ok(Renderer {
			vao: vao,
			vbo: vbo,
			buffer: buffer,
			buff_idx: 0,
			shader: shader,
		})
	}

	pub fn delete(&self) {
		self.shader.delete();
		unsafe {
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}

	pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
		unsafe {
			gl::ClearColor(r, g, b, a);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn begin(&mut self) {
		self.shader.bind();
		self.buff_idx = 0;
		self.buffer.clear();
	}

	pub fn end(&mut self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::BufferSubData(
				gl::ARRAY_BUFFER,
				0,
				(self.buffer.len() * size_of::<Vertex>()) as GLsizeiptr,
				self.buffer.as_ptr() as *const GLvoid,
			);

			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, self.buff_idx);
		}
	}

	pub fn push_vertex(&mut self, v: Vertex) {
		self.buffer.push(v);
		self.buff_idx += VERTEX_SIZE;
	}

	pub fn push_quad(&mut self, pos: Vec3, size: Vec2, color: Vec4) {
		if (self.buff_idx + 6 * VERTEX_SIZE) / VERTEX_SIZE >= MAX_VERT_CNT {
			self.end();
			self.begin();
		}

		let p1 = Vertex {
			pos: pos.clone(),
			color: color.clone()
		};
		let p2 = Vertex {
			pos: Vec3::new(pos.x + size.x, pos.y, pos.z),
			color: color.clone()
		};
		let p3 = Vertex {
			pos: Vec3::new(pos.x + size.x, pos.y + size.y, pos.z),
			color: color.clone()
		};
		let p4 = Vertex {
			pos: Vec3::new(pos.x + size.x, pos.y + size.y, pos.z),
			color: color.clone()
		};
		let p5 = Vertex {
			pos: Vec3::new(pos.x, pos.y + size.y, pos.z),
			color: color.clone()
		};
		let p6 = Vertex {
			pos: pos.clone(),
			color: color.clone()
		};

		self.push_vertex(p1);
		self.push_vertex(p2);
		self.push_vertex(p3);
		self.push_vertex(p4);
		self.push_vertex(p5);
		self.push_vertex(p6);
	}
}
