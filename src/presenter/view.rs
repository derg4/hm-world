extern crate glium;
// For sending input data from View to Presenter
// #[derive(Debug)]
// pub enum ViewInputMsg {
// ViewInitialized,
// WindowEvents(Vec<glium::glutin::WindowEvent>),
// _Unused,
// }
//
// For sending output data from Presenter to View
// #[derive(Debug)]
// pub enum ViewOutputMsg {
// SetShaders(String, String),
// InitializeDisplay,
// RequestEvents,
// _Unused,
// }

pub trait View {
	fn set_shaders(&mut self, &str, &str);
	fn set_title(&self, &str);
	fn draw(&mut self);
	fn poll_events(&mut self) -> Vec<glium::glutin::WindowEvent>;
}
