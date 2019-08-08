use glium::{Display, Program};
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder, WindowEvent};
use glium::glutin::dpi::LogicalSize;

use crate::presenter::View;

pub struct GLView{
	display: Display,
	events_loop: EventsLoop,
	program: Option<Program>,
}
impl GLView {
	pub fn new() -> Result<GLView, String> {
		let el = EventsLoop::new();
		let wb = WindowBuilder::new()
			.with_dimensions(LogicalSize::new(1024.0, 768.0));
		let cb = ContextBuilder::new();
		let display = match Display::new(wb, cb, &el) {
			Ok(display) => display,
			Err(err) => {
				error!("glview::init_display: {}", err);
				return Err(format!("{}", err));
			},
		};

		Ok(GLView {
			display: display,
			events_loop: el,
			program: None,
		})
	}
}
impl View for GLView {
	fn set_shaders(&mut self, vert_shader: &str, frag_shader: &str) {
		match Program::from_source(&self.display, vert_shader, frag_shader, None) {
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
	fn poll_events(&mut self) -> Vec<WindowEvent> {
		let mut events = Vec::new();
		self.events_loop.poll_events(|event| {
			if let glium::glutin::Event::WindowEvent { event: window_event, .. } = event {
				events.push(window_event)
			}
		});
		events
	}
}
