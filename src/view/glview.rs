extern crate glium;
use glium::glutin;

extern crate log;

use crate::presenter::View;

pub struct GLView{
	display: glium::Display,
	program: Option<glium::Program>,
}
impl GLView {
	pub fn new() -> Result<GLView, String> {
		let el = glutin::EventsLoop::new();
		let wb = glutin::WindowBuilder::new()
			.with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
		let cb = glutin::ContextBuilder::new();
		let display = match glium::Display::new(wb, cb, &el) {
			Ok(display) => display,
			Err(err) => {
				error!("glview::init_display: {}", err);
				return Err(format!("{}", err));
			},
		};

		Ok(GLView {
			display: display,
			program: None,
		})
	}
}
impl View for GLView {
	fn set_shaders(&mut self, vert_shader: &str, frag_shader: &str) {
		match glium::Program::from_source(&self.display, vert_shader, frag_shader, None) {
			Ok(program) => self.program = Some(program),
			Err(err) => {
				self.program = None;
				error!("glview::init_program: {}", err);
			},
		}
	}

	fn set_title(&self, title: &str) {
		info!("Setting view title to {}", title);
		self.display.gl_window().window().set_title(title);
	}
	fn draw(&mut self) {}
	fn poll_events(&mut self) -> Vec<glium::glutin::WindowEvent> {
		vec![]
	}
}
