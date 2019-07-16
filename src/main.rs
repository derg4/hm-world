#[macro_use]
extern crate glium;
use glium::{Display, Surface};
use glium::texture::Texture2dArray;

//extern crate glutin;
use glium::glutin;
use glutin::{ElementState, VirtualKeyCode};
//use glutin::{Event, WindowEvent};
/*use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, VirtualKeyCode};*/
// use glutin::{ElementState, VirtualKeyCode};

use std::fs::File;
use std::io::{Read, BufReader};
use std::time::{Duration, Instant};
use std::collections::HashSet;

extern crate cgmath;
use cgmath::{Matrix4, Point3, Vector3, Deg};
use cgmath::{EuclideanSpace, InnerSpace};

extern crate image;

extern crate rand;
use rand::prelude::*;

mod city;

mod sphericalPoint;
use sphericalPoint::SphericalPoint;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 4],
	color: [f32; 3],
	normal: [f32; 3],
	tex_coords: [f32; 3],
}
impl Default for Vertex {
	fn default() -> Vertex {
		Vertex {
			position: [0f32, 0f32, 0f32, 1f32],
			color: [1f32, 1f32, 1f32],
			normal: [0f32, 1f32, 0f32],
			tex_coords: [0f32, 0f32, 0f32],
		}
	}
}
impl std::fmt::Debug for Vertex {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,
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
		       self.color[2])?;
		Ok(())
	}
}
implement_vertex!(Vertex, position, color, normal, tex_coords);

struct Model {
	vertices: Vec<Vertex>,
	triangles_list: Vec<u32>,
}
impl std::fmt::Debug for Model {
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

#[derive(Debug)]
struct Camera {
	position: Point3<f32>,
	direction: Vector3<f32>,
	up: Vector3<f32>,
	attached: bool,
}

#[derive(Debug)]
struct Light {
	position: [f32; 3],
	color: [f32; 4],
	ambient: [f32; 4],
}

#[derive(Debug)]
struct KeyBindings {
	move_forward: VirtualKeyCode,
	move_backward: VirtualKeyCode,
	move_right: VirtualKeyCode,
	move_left: VirtualKeyCode,
	move_up: VirtualKeyCode,
	move_down: VirtualKeyCode,
	move_fast: VirtualKeyCode,
	quit: VirtualKeyCode,
	debug_output: VirtualKeyCode,
	move_light: VirtualKeyCode,
	toggle_cursor_grab: VirtualKeyCode,
	toggle_camera_attached: VirtualKeyCode,
}
impl Default for KeyBindings {
	fn default() -> KeyBindings {
		KeyBindings {
			move_forward: VirtualKeyCode::W,
			move_backward: VirtualKeyCode::S,
			move_right: VirtualKeyCode::D,
			move_left: VirtualKeyCode::A,
			move_up: VirtualKeyCode::R,
			move_down: VirtualKeyCode::F,
			move_fast: VirtualKeyCode::LShift,
			quit: VirtualKeyCode::Escape,
			debug_output: VirtualKeyCode::P,
			move_light: VirtualKeyCode::L,
			toggle_cursor_grab: VirtualKeyCode::H,
			toggle_camera_attached: VirtualKeyCode::C,
		}
	}
}

#[derive(Debug)]
struct GameSettings {
	key_bindings: KeyBindings,
	max_fps: f32,
	move_forward_speed: f32,
	move_backward_speed: f32,
	strafe_speed: f32,
	scroll_speed: f32,
	zoom_speed: f32,
	min_theta: f32,
	light_distance: f32,
}
impl Default for GameSettings {
	fn default() -> GameSettings {
		GameSettings {
			key_bindings: Default::default(),
			max_fps: 60f32,
			move_forward_speed: 1.0f32,
			move_backward_speed: 1.0f32,
			strafe_speed: 1.0f32,
			scroll_speed: 1.0f32,
			zoom_speed: 0.5f32,
			min_theta: 0.001f32,
			light_distance: 10000f32,
		}
	}
}

#[derive(Debug)]
struct GameState {
	camera: Camera,
	light: Light,
	settings: GameSettings,
	pressed_keys: HashSet<VirtualKeyCode>,
}
impl Default for GameState {
	fn default() -> GameState {
		GameState {
			camera: Camera {
				position: Point3::new(0.0f32, 0.0f32, 2.0f32),
				direction: Vector3::new(0.0f32, 0.0f32, -1.0f32),
				up: Vector3::new(0.0f32, 1.0f32, 0.0f32),
				attached: true,
			},
			light: Light {
				position: [0.0f32, 0.0f32, 10000f32],
				color: [1.0f32, 1.0f32, 1.0f32, 1.0f32],
				ambient: [0.05f32, 0.05f32, 0.05f32, 1.0f32],
			},
			settings: Default::default(),
			pressed_keys: HashSet::new(),
		}
	}
}

// ============================================================================
// UTILITIES
// ============================================================================

fn duration_to_secs(duration: &Duration) -> f32 {
	let secs: f32 = duration.as_secs() as f32;
	let subsec: f32 = duration.subsec_nanos() as f32 / 1_000_000_000_f32;
	secs + subsec
}

fn read_shader_file(file_name: &str) -> Result<String, std::io::Error> {

	let mut f = File::open(file_name)?;

	let mut s = String::new();
	f.read_to_string(&mut s)?;
	return Ok(s);
}

fn get_image_from_file(file_name: String)
                       -> Result<glium::texture::RawImage2d<'static, u8>, std::io::Error> {
	let image = image::load(BufReader::with_capacity(8192, File::open(file_name)?),
	                        image::PNG)
		.unwrap()
		.to_rgba();

	let image_dimensions = image.dimensions();

	Ok(glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions))
}

fn get_texture_array(display: &glium::backend::Facade) -> Texture2dArray {
	let texture_dir = "textures/tellene_1/";
	let texture_file_prefix = "tellene_";

	let missing_tex_file = format!("{}{}missing.png", texture_dir, texture_file_prefix);
	let mut images = Vec::new();
	images.push(get_image_from_file(missing_tex_file).unwrap());

	for lat in 23..54 {
		for long in -26..13 {
			let tex_file = format!("{}{}{}_{}.png", texture_dir, texture_file_prefix, lat, long);
			images.push(get_image_from_file(tex_file).unwrap());
		}
	}

	Texture2dArray::new(display, images).unwrap()
}

fn gen_sphere_model(deg_resolution: u32, radius: f32) -> Model {
	let lat_divs = 180u32 / deg_resolution;
	let lat_inc = std::f32::consts::PI / (lat_divs as f32);

	let lon_divs = 360u32 / deg_resolution;
	let lon_inc = 2f32 * std::f32::consts::PI / (lon_divs as f32);
	// TODO Only generates texture coords for the missing texture atm

	let mut model = Model {
		vertices: Vec::new(),
		triangles_list: Vec::new(),
	};
	let mut rng = rand::thread_rng();

	fn add_vertex_fn(lat: f32,
	                 lon: f32,
	                 tex: [f32; 3],
	                 r: &f32,
	                 rng: &mut ThreadRng,
	                 verts: &mut Vec<Vertex>) {
		let pos = SphericalPoint::new(r, &lat, &lon).to_point();
		let &(x, y, z) = pos.as_ref();

		let vert = Vertex {
			position: [x, y, z, 1f32],
			color: [rng.gen(), rng.gen(), rng.gen()],
			normal: pos.to_vec().normalize().into(),
			tex_coords: tex,
		};

		verts.push(vert);

	}
	let mut add_vertex = |lat: u32, lon: u32, tex: [f32; 3], verts: &mut Vec<Vertex>| {
		add_vertex_fn((lat as f32) * lat_inc,
		              (lon as f32) * lon_inc,
		              tex,
		              &radius,
		              &mut rng,
		              verts);
	};

	let add_triangle = |base, offset_a, offset_b, offset_c, tris: &mut Vec<u32>| {
		tris.push(base + offset_a);
		tris.push(base + offset_b);
		tris.push(base + offset_c);
	};

	for lat in 0..lat_divs {
		for lon in 0..lon_divs {
			let verts_len = model.vertices.len() as u32;
			let lat_deg: i32 = 90i32 - (deg_resolution as i32 * lat as i32);
			let lon_deg: i32 = (deg_resolution as i32 * lon as i32) - 180i32;
			let (min_lat, rows, min_lon, cols) = (23i32, 31i32, -26i32, 39i32);

			// TODO support resolutions > 1?
			let tex_id: f32 = if lat_deg >= min_lat && lat_deg < min_lat + rows &&
			                     lon_deg >= min_lon && lon_deg < min_lon + cols {
				let row = lat_deg - min_lat;
				let col = lon_deg - min_lon;
				1f32 + (col as f32) + ((row * cols) as f32)
			} else {
				0f32
			};

			// TODO: Make this more readable? It's a nightmare
			if lat == 0 {
				// First ring, 3 verts, one triangle for each lon
				add_vertex(lat, lon, [0f32, 1f32, tex_id], &mut model.vertices);
				add_vertex(lat + 1,
				           (lon + 1) % lon_divs,
				           [1f32, 0f32, tex_id],
				           &mut model.vertices);
				add_vertex(lat + 1, lon, [0f32, 0f32, tex_id], &mut model.vertices);
				add_triangle(verts_len, 0, 1, 2, &mut model.triangles_list);
			} else if lat == lat_divs - 1 {
				// Last ring, 3 verts, one triangle for each lon
				add_vertex(lat, lon, [0f32, 1f32, tex_id], &mut model.vertices);
				add_vertex(lat,
				           (lon + 1) % lon_divs,
				           [1f32, 1f32, tex_id],
				           &mut model.vertices);
				add_vertex(lat + 1, lon, [0f32, 0f32, tex_id], &mut model.vertices);
				add_triangle(verts_len, 0, 1, 2, &mut model.triangles_list);
			} else {
				// Middle rings, 4 verts, two triangles for each lon
				add_vertex(lat, lon, [0f32, 1f32, tex_id], &mut model.vertices);
				add_vertex(lat,
				           (lon + 1) % lon_divs,
				           [1f32, 1f32, tex_id],
				           &mut model.vertices);
				add_vertex(lat + 1, lon, [0f32, 0f32, tex_id], &mut model.vertices);
				add_vertex(lat + 1,
				           (lon + 1) % lon_divs,
				           [1f32, 0f32, tex_id],
				           &mut model.vertices);
				add_triangle(verts_len, 0, 1, 2, &mut model.triangles_list);
				add_triangle(verts_len, 1, 3, 2, &mut model.triangles_list);
			}
		}
	}

	// Clean up and give our caller a beautiful model!
	model.vertices.shrink_to_fit();
	model.triangles_list.shrink_to_fit();
	model
}

fn process_key_press(key_code: &VirtualKeyCode,
                     game_state: &mut GameState,
                     display: &Display) {
	let key_bindings = &game_state.settings.key_bindings;
	if *key_code == key_bindings.quit {
		println!("User hit the quit key, exiting.");
		std::process::exit(0);
	}
	/* if *key_code == key_bindings.toggle_cursor_grab {
	 * game_state.cursor_state = match game_state.cursor_state {
	 * CursorState::Normal => CursorState::Grab,
	 * CursorState::Grab => CursorState::Normal,
	 * _ => CursorState::Normal,
	 * };
	 * if let Some(win) = display.get_window() {
	 * win.set_cursor_state(game_state.cursor_state).unwrap();
	 * win.set_cursor(match game_state.cursor_state {
	 * CursorState::Normal => MouseCursor::Default,
	 * CursorState::Grab => MouseCursor::NoneCursor,
	 * _ => MouseCursor::Default,
	 * });
	 * }
	 * } */
	if *key_code == key_bindings.toggle_camera_attached {
		game_state.camera.attached = !game_state.camera.attached;
	}
}

fn process_keys_held(game_state: &mut GameState, time_since_last_frame: f32) {
	let key_bindings = &game_state.settings.key_bindings;
	let camera = &mut game_state.camera;

	// Do processing for currently held keys
	for key_code in &game_state.pressed_keys {
		let move_fast_held: bool = game_state.pressed_keys.contains(&key_bindings.move_fast);
		let move_multiplier = if move_fast_held { 2f32 } else { 1f32 };

		if !camera.attached {
			// Camera is not attached to globe, player can move camera freely
			if *key_code == key_bindings.move_forward {
				camera.position = camera.position +
				                  camera.direction * game_state.settings.move_forward_speed *
				                  move_multiplier * time_since_last_frame;
			}
			if *key_code == key_bindings.move_backward {
				camera.position = camera.position +
				                  -camera.direction * game_state.settings.move_backward_speed *
				                  move_multiplier * time_since_last_frame;
			}
			if *key_code == key_bindings.move_left {
				camera.position = camera.position +
				                  -camera.direction.cross(camera.up) *
				                  game_state.settings.strafe_speed *
				                  move_multiplier * time_since_last_frame;
			}
			if *key_code == key_bindings.move_right {
				camera.position = camera.position +
				                  camera.direction.cross(camera.up) *
				                  game_state.settings.strafe_speed *
				                  move_multiplier * time_since_last_frame;
			}
			if *key_code == key_bindings.move_up {
				// TODO move as the "camera's" up?
				camera.position = camera.position +
				                  camera.up * game_state.settings.strafe_speed * move_multiplier *
				                  time_since_last_frame;
			}
			if *key_code == key_bindings.move_down {
				// TODO move as the "camera's" down?
				camera.position = camera.position +
				                  -camera.up * game_state.settings.strafe_speed * move_multiplier *
				                  time_since_last_frame;
			}
		} else {
			// Camera is attached to globe, player can only rotate around globe
			// https://en.wikipedia.org/wiki/Spherical_coordinate_system
			// TODO when camera is locked, store r/theta/phi values instead of recalculating all the time, less jittery at extremes (ex. when really zoomed in on poles)
			let mut sph = SphericalPoint::from_point(&camera.position);

			let zoom = sph.radius - 1.0f32;
			let frac_zoom = (game_state.settings.zoom_speed / move_multiplier)
				.powf(time_since_last_frame);
			let frac_scroll = game_state.settings.scroll_speed * time_since_last_frame *
			                  zoom.min(2f32) * move_multiplier;
			if *key_code == key_bindings.move_forward {
				sph.theta = (sph.theta - frac_scroll).max(game_state.settings.min_theta);
			}
			if *key_code == key_bindings.move_backward {
				sph.theta = (sph.theta + frac_scroll)
					.min(std::f32::consts::PI - game_state.settings.min_theta);
			}
			if *key_code == key_bindings.move_left {
				sph.phi = sph.phi - frac_scroll / sph.theta.sin().max(0.25f32); //TODO hardcoded max spin speedup value
			}
			if *key_code == key_bindings.move_right {
				sph.phi = sph.phi + frac_scroll / sph.theta.sin().max(0.25f32);
			}
			if *key_code == key_bindings.move_up {
				sph.radius = (sph.radius - 1.0f32) * frac_zoom + 1.0f32; //TODO hardcoded globe radius
			}
			if *key_code == key_bindings.move_down {
				sph.radius = (sph.radius - 1.0f32) / frac_zoom + 1.0f32;
			}

			if game_state.pressed_keys.contains(&VirtualKeyCode::T) {
				println!("r {} | theta {} | phi {}", sph.radius, sph.theta, sph.phi);
			}

			if !sph.is_ok() {
				println!("Resetting camera, bad spherical coord values: r {} | theta {} | phi {}",
				         sph.radius,
				         sph.theta,
				         sph.phi);
				sph.radius = 2.0f32;
				sph.theta = std::f32::consts::PI / 2.0f32;
				sph.phi = 0.0f32;
			}

			let new_pos = sph.to_point();
			camera.direction = Vector3::new(-new_pos.x, -new_pos.y, -new_pos.z).normalize();
			camera.position = new_pos;
		}
		if *key_code == key_bindings.debug_output {
			println!("Time since last frame: {}", time_since_last_frame);
			println!("camera: {:?}, {:?}, {:?}",
			         camera.position,
			         camera.direction,
			         camera.up);
		}
		if *key_code == key_bindings.move_light {
			game_state.light.position =
				Vector3::new(camera.position.x, camera.position.y, camera.position.z)
					.normalize_to(game_state.settings.light_distance)
					.into();
		}
	}
}

fn main() {
	let cities = city::read_cities_file("src/cities.csv");

	let mut el = glutin::EventsLoop::new();
	let wb = glutin::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &el).unwrap();

	let vert_shader_src = read_shader_file("src/vertex.glsl").expect("Failed to read vert shader");
	let frag_shader_src = read_shader_file("src/fragment.glsl")
		.expect("Failed to read frag shader");
	let program = glium::Program::from_source(&display,
	                                          vert_shader_src.as_str(),
	                                          frag_shader_src.as_str(),
	                                          None)
		.unwrap();

	let sphere = gen_sphere_model(1u32, 1f32);
	// println!("{}", sphere);

	let vertex_buffer = glium::VertexBuffer::new(&display, &sphere.vertices).unwrap();
	let indices = glium::index::IndexBuffer::new(&display,
	                                             glium::index::PrimitiveType::TrianglesList,
	                                             &sphere.triangles_list)
		.unwrap();

	let mut fps_track_start = Instant::now();
	let mut frame_count = 0u32;

	let mut last_frame = Instant::now();
	let mut game_state: GameState = Default::default();

	let textures = get_texture_array(&display);

	loop {
		let this_frame = Instant::now();
		let time_since_last_frame: f32 = duration_to_secs(&this_frame.duration_since(last_frame));

		let (win_width, win_height): (u32, u32) = display.get_framebuffer_dimensions();
		let aspect_ratio = (win_width as f32) / (win_height as f32);

		let mat_s: Matrix4<f32> = Matrix4::from_scale(1.0f32);
		let mat_r: Matrix4<f32> = Matrix4::from_axis_angle(Vector3::new(0.0f32, 1.0f32, 0.0f32),
		                                                   Deg(0.0f32));
		let mat_t: Matrix4<f32> = Matrix4::from_translation(Vector3::new(0.0f32, 0.0f32, 0.0f32));
		let model_mat: [[f32; 4]; 4] = (mat_t * mat_r * mat_s).into();

		let view_mat: [[f32; 4]; 4] = Matrix4::look_at(game_state.camera.position,
		                                               game_state.camera.position +
		                                               game_state.camera.direction,
		                                               game_state.camera.up)
			.into();

		// TODO FOV is fov for y, and adjust near/far plane
		let proj_mat: [[f32; 4]; 4] =
			cgmath::perspective(Deg(90f32), aspect_ratio, 0.001f32, 1000f32).into();

		let uniforms = uniform! {
			u_model_mat: model_mat,
			u_view_mat: view_mat,
			u_proj_mat: proj_mat,
			u_light_pos: game_state.light.position,
			u_light_color: game_state.light.color,
			u_light_ambient: game_state.light.ambient,
			u_tex: &textures,
		};

		let mut target = display.draw();
		let draw_params = glium::DrawParameters {
			backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
			..Default::default()
		};
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_params)
			.unwrap();
		target.finish().unwrap();

		// Do processing for new events (key presses, etc)
		el.poll_events(|ev| {
			match ev {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::CloseRequested => return,
					glutin::WindowEvent::KeyboardInput { input, .. } => {
						match input.virtual_keycode {
							Some(key_code) => {
								match input.state {
									ElementState::Pressed => {
										game_state.pressed_keys.insert(key_code);
										process_key_press(&key_code, &mut game_state, &display);
									}
									ElementState::Released => {
										game_state.pressed_keys.remove(&key_code);
									}
								};
							},
							None => (),
						}
					//elem_state, _, Some(virt_key_code)) => {
					},
					_ => (),
				},
				_ => (),
			}
		});

		process_keys_held(&mut game_state, time_since_last_frame);

		// Cleanup code for the current frame
		// FPS tracker
		frame_count += 1;
		if duration_to_secs(&fps_track_start.elapsed()) > 1f32 {
			println!("FPS: {}", frame_count);
			fps_track_start = Instant::now();
			frame_count = 0;
		}

		// Sleep for some time if we were to exceed max fps
		let min_frame_duration =
			Duration::new(0u64, (1_000_000_000f32 / game_state.settings.max_fps) as u32);
		if let Some(time_to_sleep) = min_frame_duration.checked_sub(this_frame.elapsed()) {
			std::thread::sleep(time_to_sleep);
		}
		last_frame = this_frame;
	}
}
