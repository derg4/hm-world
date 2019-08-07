#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: [f32; 4],
	pub color: [f32; 3],
	pub normal: [f32; 3],
	pub tex_coords: [f32; 3],
}
impl Default for Vertex {
	fn default() -> Vertex {
		Vertex {
			position: [0_f32, 0_f32, 0_f32, 1_f32],
			color: [1_f32, 1_f32, 1_f32],
			normal: [0_f32, 1_f32, 0_f32],
			tex_coords: [0_f32, 0_f32, 0_f32],
		}
	}
}
impl std::fmt::Debug for Vertex {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"p ({}, {}, {}, {}), n ({}, {}, {}), t ({}, {}, {}), c ({}, {}, {})",
			self.position[0],
			self.position[1],
			self.position[2],
			self.position[3],
			self.normal[0],
			self.normal[1],
			self.normal[2],
			self.tex_coords[0],
			self.tex_coords[1],
			self.tex_coords[2],
			self.color[0],
			self.color[1],
			self.color[2]
		)
	}
}
glium::implement_vertex!(Vertex, position, color, normal, tex_coords);

pub struct Mesh {
	vertices: Vec<Vertex>,
	triangles_list: Vec<u32>,
}
impl std::fmt::Debug for Mesh {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "Vertices (position, normal, tex_coords, color):")?;
		for (i, vertex) in self.vertices.iter().enumerate() {
			writeln!(f, "{}: {:?}", i, vertex)?;
		}
		writeln!(f, "Indices:")?;
		for (i, index) in self.triangles_list.iter().enumerate() {
			writeln!(f, "{}: {:?}", i, index)?;
		}
		Ok(())
	}
}
