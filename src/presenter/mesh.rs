use cgmath::{InnerSpace, Rad, Vector3};
use glium::backend::Facade;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::entities::SphericalPoint;

#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: [f64; 4],
	pub color: [f64; 3],
	pub normal: [f64; 3],
	pub tex_coords: [f64; 3],
}
impl Default for Vertex {
	fn default() -> Vertex {
		Vertex {
			position: [0_f64, 0_f64, 0_f64, 1_f64],
			color: [1_f64, 1_f64, 1_f64],
			normal: [0_f64, 1_f64, 0_f64],
			tex_coords: [0_f64, 0_f64, 0_f64],
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
	//pub vertices: Vec<Vertex>,
	//pub triangles_list: Vec<u32>,
	pub vertex_buffer: glium::VertexBuffer<Vertex>,
	pub index_buffer: glium::index::IndexBuffer<u32>,
}
impl Mesh {
	pub fn new<F: ?Sized>(facade: &F, vertices: &[Vertex], triangles: &[u32]) -> Mesh
	where
		F: Facade,
	{
		Mesh {
			vertex_buffer: glium::VertexBuffer::new(facade, vertices).unwrap(),
			index_buffer: glium::index::IndexBuffer::new(
				facade,
				glium::index::PrimitiveType::TrianglesList,
				triangles,
			)
			.unwrap(),
		}
	}
	pub fn gen_sphere_mesh<F: ?Sized>(facade: &F, deg_resolution: u32, radius: f64) -> Mesh
	where
		F: Facade,
	{
		let lat_divs = 180u32 / deg_resolution;
		let lat_inc = std::f64::consts::PI / (lat_divs as f64);

		let lon_divs = 360u32 / deg_resolution;
		let lon_inc = 2f64 * std::f64::consts::PI / (lon_divs as f64);
		// TODO Only generates texture coords for the missing texture atm

		let mut vertices = Vec::new();
		let mut triangles_list = Vec::new();
		let mut rng = rand::thread_rng();

		fn add_vertex_fn(
			lat: f64,
			lon: f64,
			tex: [f64; 3],
			r: f64,
			rng: &mut ThreadRng,
			verts: &mut Vec<Vertex>,
		) {
			let pos = SphericalPoint::new(r, Rad(lat), Rad(lon)).to_point();
			let &(x, y, z) = pos.as_ref();

			let vert = Vertex {
				position: [x, y, z, 1f64],
				color: [rng.gen(), rng.gen(), rng.gen()],
				normal: Vector3::new(pos.x, pos.y, pos.z).normalize().into(),
				tex_coords: tex,
			};

			verts.push(vert);
		}
		let mut add_vertex = |lat: u32, lon: u32, tex: [f64; 3], verts: &mut Vec<Vertex>| {
			add_vertex_fn(
				(lat as f64) * lat_inc,
				(lon as f64) * lon_inc,
				tex,
				radius,
				&mut rng,
				verts,
			);
		};

		let add_triangle = |base, offset_a, offset_b, offset_c, tris: &mut Vec<u32>| {
			tris.push(base + offset_a);
			tris.push(base + offset_b);
			tris.push(base + offset_c);
		};

		for lat in 0..lat_divs {
			for lon in 0..lon_divs {
				let verts_len = vertices.len() as u32;
				let lat_deg: i32 = 90i32 - (deg_resolution as i32 * lat as i32);
				let lon_deg: i32 = (deg_resolution as i32 * lon as i32) - 180i32;
				let (min_lat, rows, min_lon, cols) = (23i32, 31i32, -26i32, 39i32);

				// TODO support resolutions > 1?
				let tex_id: f64 = if lat_deg >= min_lat
					&& lat_deg < min_lat + rows
					&& lon_deg >= min_lon
					&& lon_deg < min_lon + cols
				{
					let row = lat_deg - min_lat;
					let col = lon_deg - min_lon;
					1f64 + (col as f64) + ((row * cols) as f64)
				} else {
					0f64
				};

				// TODO: Make this more readable? It's a nightmare
				if lat == 0 {
					// First ring, 3 verts, one triangle for each lon
					add_vertex(lat, lon, [0f64, 1f64, tex_id], &mut vertices);
					add_vertex(
						lat + 1,
						(lon + 1) % lon_divs,
						[1f64, 0f64, tex_id],
						&mut vertices,
					);
					add_vertex(lat + 1, lon, [0f64, 0f64, tex_id], &mut vertices);
					add_triangle(verts_len, 0, 1, 2, &mut triangles_list);
				} else if lat == lat_divs - 1 {
					// Last ring, 3 verts, one triangle for each lon
					add_vertex(lat, lon, [0f64, 1f64, tex_id], &mut vertices);
					add_vertex(
						lat,
						(lon + 1) % lon_divs,
						[1f64, 1f64, tex_id],
						&mut vertices,
					);
					add_vertex(lat + 1, lon, [0f64, 0f64, tex_id], &mut vertices);
					add_triangle(verts_len, 0, 1, 2, &mut triangles_list);
				} else {
					// Middle rings, 4 verts, two triangles for each lon
					add_vertex(lat, lon, [0f64, 1f64, tex_id], &mut vertices);
					add_vertex(
						lat,
						(lon + 1) % lon_divs,
						[1f64, 1f64, tex_id],
						&mut vertices,
					);
					add_vertex(lat + 1, lon, [0f64, 0f64, tex_id], &mut vertices);
					add_vertex(
						lat + 1,
						(lon + 1) % lon_divs,
						[1f64, 0f64, tex_id],
						&mut vertices,
					);
					add_triangle(verts_len, 0, 1, 2, &mut triangles_list);
					add_triangle(verts_len, 1, 3, 2, &mut triangles_list);
				}
			}
		}

		// Clean up and give our caller a beautiful mesh!
		Self::new(facade, &vertices, &triangles_list)
	}
}
impl std::fmt::Debug for Mesh {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Mesh[{:?}, {:?}]", self.vertex_buffer, self.index_buffer)
		/*writeln!(f, "Vertices (position, normal, tex_coords, color):")?;
		for (i, vertex) in self.vertex_buffer.data.iter().enumerate() {
			writeln!(f, "{}: {:?}", i, vertex)?;
		}
		writeln!(f, "Indices:")?;
		for (i, index) in self.index_buffer.data.iter().enumerate() {
			writeln!(f, "{}: {:?}", i, index)?;
		}
		Ok(())*/
	}
}
