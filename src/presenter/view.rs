use cgmath::Matrix4;
use glium::texture::texture2d_array::Texture2dArray;

use super::{AmbientLight, MeshObject, WorldLight};

pub trait View {
	fn set_shaders(&mut self, &str, &str);
	fn set_texture_array(
		&mut self,
		Vec<glium::texture::RawImage2d<'static, u8>>,
	) -> Option<&Texture2dArray>;
	fn set_title(&self, &str);
	fn draw(
		&self,
		view_mat: Matrix4<f64>,
		proj_mat: Matrix4<f64>,
		ambient_light: &AmbientLight,
		world_light: &WorldLight,
		objects: &[MeshObject],
	);
	fn poll_events(&mut self) -> Vec<glium::glutin::WindowEvent>;
	fn get_aspect_ratio(&self) -> f64;
	fn get_facade(&self) -> &glium::Display;
}
