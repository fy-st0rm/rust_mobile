use winit::window::WindowAttributes;
use winit::event::Event;
use winit::event::WindowEvent;
use egl;
use std::ffi::*;
use glow;
use glow::HasContext;

#[cfg(target_os = "android")]
pub use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
pub fn build_android(app: AndroidApp) {
	use winit::platform::android::EventLoopBuilderExtAndroid;
	use winit::event_loop::EventLoopBuilder;

	let event_loop = EventLoopBuilder::new()
		.with_android_app(app.clone())
		.build()
		.expect("No event");

	let window = event_loop
		.create_window(WindowAttributes::new())
		.expect("No window");

	// Initialize EGL
	let egl_display = egl::get_display(egl::EGL_DEFAULT_DISPLAY).unwrap();
	let mut major: i32 = 0;
	let mut minor: i32 = 0;
	if !egl::initialize(egl_display, &mut major, &mut minor) {
		panic!("Failed to initialize egl");
	}

	// Choose an appropriate EGL configuration
	let config_attributes = [
			egl::EGL_SURFACE_TYPE as i32, egl::EGL_WINDOW_BIT as i32,
			egl::EGL_BLUE_SIZE as i32, 8,
			egl::EGL_GREEN_SIZE as i32, 8,
			egl::EGL_RED_SIZE as i32, 8,
			egl::EGL_DEPTH_SIZE as i32, 24,
			egl::EGL_NONE as i32
	];
	let egl_config = egl::choose_config(egl_display, &config_attributes, 10).unwrap();

	// Create the EGL context
	let context_attribs = [
		egl::EGL_CONTEXT_CLIENT_VERSION, 2,  // Request OpenGL ES 2.0
		egl::EGL_NONE,
	];
	let egl_context = egl::create_context(egl_display, egl_config, std::ptr::null_mut(), &context_attribs).expect("No context?");

	let mut glow_gl: Option<glow::Context> = None;
	let mut egl_surface: *mut c_void = std::ptr::null_mut() as *mut _;

	// Use loop the proper way
	event_loop.run(move |event: Event<()>, _| {
		match event {
			Event::WindowEvent { event, window_id } => match event {
				WindowEvent::Resized(_) => {
					// Ensure the native window is accessible
					if let Some(native_window) = app.native_window() {
						let native_window = app.native_window().expect("No native window?");
						egl_surface = egl::create_window_surface(egl_display, egl_config, native_window.ptr().as_ptr() as *mut _, &[]).unwrap();

						// Make the EGL context current
						if !egl::make_current(egl_display, egl_surface, egl_surface, egl_context) {
							panic!("Failed to make context");
						}

						unsafe {
							//gl::load_with(|name| egl::get_proc_address(name) as *const _);
							glow_gl = Some(glow::Context::from_loader_function(|s| egl::get_proc_address(s) as *mut _));
						}
					}
				}
				WindowEvent::RedrawRequested => {
					unsafe {
						if let Some(gl) = &glow_gl {
							gl.clear_color(0.0, 0.0, 1.0, 1.0); // RGBA values for blue
							// Clear the color buffer
							gl.clear(gl::COLOR_BUFFER_BIT);
							// Swap buffers to display the cleared color
							if !egl::swap_buffers(egl_display, egl_surface) {
								panic!("Failed to swap buffers");
							}
						}
					}
				}
				_ => (),
			},
			_ => (),
		}
	});
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
		build_android(app);
}
