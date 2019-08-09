use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder, WindowEvent};
use glium::texture::texture2d_array::Texture2dArray;
use glium::{Display, Program, Surface};

use cgmath::{Matrix4, Point3, Vector4};

use crate::presenter::{AmbientLight, MeshObject, View, WorldLight};

pub struct GLView {
	display: Display,
	events_loop: EventsLoop,
	program: Option<Program>,
	texture_array: Option<Texture2dArray>,
}
impl GLView {
	pub fn new() -> Result<GLView, String> {
		let el = EventsLoop::new();
		let wb = WindowBuilder::new().with_dimensions(LogicalSize::new(1024.0, 768.0));
		let cb = ContextBuilder::new();
		let display = match Display::new(wb, cb, &el) {
			Ok(display) => display,
			Err(err) => {
				error!("glview::init_display: {}", err);
				return Err(format!("{}", err));
			}
		};

		Ok(GLView {
			display: display,
			events_loop: el,
			program: None,
			texture_array: None,
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
			}
		}
	}

	fn set_texture_array(
		&mut self,
		images: Vec<glium::texture::RawImage2d<'static, u8>>,
	) -> Option<&Texture2dArray> {
		self.texture_array = Texture2dArray::new(&self.display, images).ok();
		self.texture_array.as_ref()
	}

	fn set_title(&self, title: &str) {
		info!("Setting view title to {}", title);
		self.display.gl_window().window().set_title(title);
	}
	fn draw(
		&self,
		view_mat: Matrix4<f64>,
		proj_mat: Matrix4<f64>,
		ambient_light: &AmbientLight,
		world_light: &WorldLight,
		objects: &[MeshObject],
	) {
		let textures = self.texture_array.as_ref().unwrap();

		let mut target = self.display.draw();
		let draw_params = glium::DrawParameters {
			backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
			..Default::default()
		};
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		for object in objects {
			let uniforms = glium::uniform! {
				u_view_mat: <Matrix4<f64> as Into<[[f64; 4]; 4]>>::into(view_mat),
				u_proj_mat: <Matrix4<f64> as Into<[[f64; 4]; 4]>>::into(proj_mat),
				u_model_mat: <Matrix4<f64> as Into<[[f64; 4]; 4]>>::into(object.model_mat()),
				u_light_pos: <Point3<f64> as Into<[f64; 3]>>::into(world_light.pos),
				u_light_color: <Vector4<f64> as Into<[f64; 4]>>::into(world_light.color),
				u_light_ambient: <Vector4<f64> as Into<[f64; 4]>>::into(ambient_light.color),
				u_tex: textures,
			};

			target
				.draw(
					&object.mesh.vertex_buffer,
					&object.mesh.index_buffer,
					self.program.as_ref().unwrap(),
					&uniforms,
					&draw_params,
				)
				.unwrap();
		}

		target.finish().unwrap();
	}
	fn poll_events(&mut self) -> Vec<WindowEvent> {
		let mut events = Vec::new();
		self.events_loop.poll_events(|event| {
			if let glium::glutin::Event::WindowEvent {
				event: window_event,
				..
			} = event
			{
				events.push(window_event)
			}
		});
		events
	}
	fn get_aspect_ratio(&self) -> f64 {
		let (win_width, win_height) = self.display.get_framebuffer_dimensions();
		(win_width as f64) / (win_height as f64)
	}
	fn get_facade(&self) -> &glium::Display {
		&self.display
	}
}
