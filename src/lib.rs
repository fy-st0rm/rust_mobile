mod math;
mod utils;
mod shader;
mod renderer;

use winit::application::ApplicationHandler;
use winit::window::{
	Window,
	WindowId,
};
use winit::event_loop::{
	ActiveEventLoop,
	ControlFlow,
	EventLoop,
};
use winit::event::WindowEvent;

use egl::{
	EGLDisplay,
	EGLSurface,
	EGLConfig,
	EGLContext,
};
use gl;

use renderer::Renderer;
use math::vec::*;

#[cfg(target_os = "android")]
pub use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid;

#[derive(Default)]
struct App {
	pub android_app: Option<AndroidApp>,
	window: Option<Window>,
	egl_display: Option<EGLDisplay>,
	egl_surface: Option<EGLSurface>,
	egl_config: Option<EGLConfig>,
	egl_context: Option<EGLContext>,
	renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		// Creating window
		self.window = Some(event_loop
			.create_window(Window::default_attributes())
			.expect("Failed to create window"));
		println!("Created window");

		// Creating EGLDisplay
		self.egl_display = Some(
			egl::get_display(egl::EGL_DEFAULT_DISPLAY)
				.expect("Failed to create EGLDisplay")
		);
		println!("Created EGLDisplay");

		// Initializing EGL
		let mut major: i32 = 0;
		let mut minor: i32 = 0;
		if !egl::initialize(self.egl_display.unwrap(), &mut major, &mut minor) {
			panic!("Failed to initialize egl");
		}
		println!("Initialized EGL: {major}.{minor}");

		// Choose an appropriate EGL configuration
		let config_attributes = [
				egl::EGL_SURFACE_TYPE as i32, egl::EGL_WINDOW_BIT as i32,
				egl::EGL_BLUE_SIZE as i32, 8,
				egl::EGL_GREEN_SIZE as i32, 8,
				egl::EGL_RED_SIZE as i32, 8,
				egl::EGL_DEPTH_SIZE as i32, 24,
				egl::EGL_NONE as i32
		];
		self.egl_config = Some(
			egl::choose_config(
				self.egl_display.unwrap(),
				&config_attributes, config_attributes.len() as i32
			).expect("Failed to setup EGLConfig")
		);
		println!("Created EGLConfig");

		// Creating EGL context
		let context_attribs = [
			egl::EGL_CONTEXT_CLIENT_VERSION, 2,  // Request OpenGL ES 2.0
			egl::EGL_NONE,
		];
		self.egl_context = Some(
			egl::create_context(
				self.egl_display.unwrap(),
				self.egl_config.unwrap(),
				std::ptr::null_mut(),
				&context_attribs
			).expect("Failed to create EGLContext")
		);
		println!("Created EGLContext");

		// Creating EGLSurface
		let native_window = self.android_app
			.as_ref()
			.unwrap()
			.native_window()
			.expect("Failed to get NativeWindow");

		self.egl_surface = Some(
			egl::create_window_surface(
				self.egl_display.unwrap(),
				self.egl_config.unwrap(),
				native_window.ptr().as_ptr() as *mut _, &[]
			).expect("Failed to create EGLSurface")
		);
		println!("Created EGLSurface");

		// Establishing Opengl context
		if !egl::make_current(
			self.egl_display.unwrap(),
			self.egl_surface.unwrap(),
			self.egl_surface.unwrap(),
			self.egl_context.unwrap()
		) {
			panic!("Failed to make context");
		}
		println!("Sucessfully established Opengl context");

		// Loaindg opengl functions
		gl::load_with(|name| egl::get_proc_address(name) as *const _);
		println!("Loaded Opengl functions");

		self.renderer = Some(
			Renderer::new().expect("Failed to create renderer")
		);
		println!("Created renderer.");
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			},
			WindowEvent::RedrawRequested => {

				if let Some(renderer) = self.renderer.as_mut() {
					renderer.clear(0.5, 0.5, 0.5, 1.0);
					renderer.begin();

					renderer.push_quad(
						Vec3::new(-0.5, -0.5, 0.0),
						Vec2::new(1.0, 1.0),
						Vec4::new(1.0, 0.0, 0.0, 1.0)
					);

					renderer.end();
				}

				if !egl::swap_buffers(
					self.egl_display.unwrap(),
					self.egl_surface.unwrap()
				) {
					panic!("Failed to swap buffers");
				}
				// Use this to call draw request for every frame
				//self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {

	// Creating eventloop for android
	let event_loop = EventLoop::builder()
		.with_android_app(app.clone())
		.build()
		.expect("Failed to create eventloop");
	event_loop.set_control_flow(ControlFlow::Poll);

	// Creating our app
	let mut t_app = App::default();
	t_app.android_app = Some(app);

	// Running the app
	event_loop.run_app(&mut t_app).unwrap();
}
