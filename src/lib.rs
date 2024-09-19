use winit::window::WindowAttributes;

#[cfg(target_os = "android")]
pub use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
pub fn build_android(app: AndroidApp) {
	use winit::platform::android::EventLoopBuilderExtAndroid;
	use winit::event_loop::EventLoopBuilder;

	let event_loop = EventLoopBuilder::new()
		.with_android_app(app)
		.build()
		.expect("No event");

	let window = event_loop
		.create_window(WindowAttributes::new())
		.expect("No window");

	// Use loop the proper way
	event_loop.run(move |_, _| {
	});
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
		build_android(app);
}
