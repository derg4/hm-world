extern crate glium;

pub trait View {
	fn set_shaders(&mut self, &str, &str);
	fn set_title(&self, &str);
	fn draw(&mut self);
	fn poll_events(&mut self) -> Vec<glium::glutin::WindowEvent>;
}
