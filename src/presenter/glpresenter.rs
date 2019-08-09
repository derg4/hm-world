extern crate log;

use super::{AmbientLight, Camera, Mesh, MeshObject, View, WorldLight};
use crate::world::{World, WorldState};

use glium::glutin::dpi::LogicalPosition;
use glium::glutin::{MouseButton, VirtualKeyCode, WindowEvent};

use cgmath::{Deg, Point3, Vector3, Vector4};

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

const VERT_SHADER: &str = include_str!["vertex.glsl"];
const FRAG_SHADER: &str = include_str!["fragment.glsl"];

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum InputType {
	Key(VirtualKeyCode),
	Mouse(MouseButton),
}

#[derive(Clone, Copy, Debug)]
enum ContinualAction {
	MoveForward,
	MoveBackward,
	MoveLeft,
	MoveRight,
	MoveUp,
	MoveDown,
	MoveFast,
}

#[derive(Clone, Copy, Debug)]
enum InstantAction {
	Quit,
	//ToggleDebug,
	MoveLight,
	//ToggleCursorGrab,
	//ToggleCameraLock,
	Log,
}

#[derive(Clone, Copy, Debug)]
enum ActionType {
	Continual(ContinualAction),
	Instant(InstantAction),
}

#[derive(Debug)]
struct Settings {
	bindings: HashMap<InputType, ActionType>,
	max_fps: f64,
	move_forward_speed: f64,
	move_backward_speed: f64,
	strafe_speed: f64,
	scroll_speed: f64,
	zoom_speed: f64,
	light_distance: f64,
	fov: Deg<f64>,
	quitting: bool,
}
impl Default for Settings {
	fn default() -> Settings {
		use self::ContinualAction::*;
		use self::InputType::{Key, Mouse};
		use self::InstantAction::*;

		let bindings: HashMap<InputType, ActionType> = [
			(Key(VirtualKeyCode::W), ActionType::Continual(MoveForward)),
			(Key(VirtualKeyCode::S), ActionType::Continual(MoveBackward)),
			(Key(VirtualKeyCode::A), ActionType::Continual(MoveLeft)),
			(Key(VirtualKeyCode::D), ActionType::Continual(MoveRight)),
			(Key(VirtualKeyCode::R), ActionType::Continual(MoveUp)),
			(Key(VirtualKeyCode::F), ActionType::Continual(MoveDown)),
			(Key(VirtualKeyCode::LShift), ActionType::Continual(MoveFast)),
			(Key(VirtualKeyCode::Escape), ActionType::Instant(Quit)),
			//(Key(VirtualKeyCode::Q), ActionType::Instant(ToggleDebug)),
			(Key(VirtualKeyCode::E), ActionType::Instant(MoveLight)),
			//(Key(VirtualKeyCode::G), ActionType::Instant(ToggleCursorGrab)),
			//(Key(VirtualKeyCode::C), ActionType::Instant(ToggleCameraLock)),
			(Mouse(MouseButton::Left), ActionType::Instant(Log)),
			(Mouse(MouseButton::Middle), ActionType::Instant(Log)),
			(Mouse(MouseButton::Right), ActionType::Instant(Log)),
		]
		.iter()
		.cloned()
		.collect();

		Settings {
			bindings: bindings,
			max_fps: 60_f64,
			move_forward_speed: 1_f64,
			move_backward_speed: 1_f64,
			strafe_speed: 1_f64,
			scroll_speed: 1_f64,
			zoom_speed: 2_f64,
			light_distance: 10_000_f64,
			fov: Deg(90_f64),
			quitting: false,
		}
	}
}

fn duration_to_secs(duration: &Duration) -> f64 {
	let secs: f64 = duration.as_secs() as f64;
	let subsec: f64 = duration.subsec_nanos() as f64 / 1_000_000_000_f64;
	secs + subsec
}

fn conv_image_to_raw_image(image: &image::DynamicImage) -> glium::texture::RawImage2d<'static, u8> {
	let image_rgba = image.to_rgba();
	let image_dimensions = image_rgba.dimensions();
	glium::texture::RawImage2d::from_raw_rgba_reversed(&image_rgba.into_raw(), image_dimensions)
}

pub struct GLPresenter {
	view: Box<View>,
	world: Box<World>,
	inputs_held: HashSet<InputType>,
	settings: Settings,
	objects: Vec<MeshObject>,
	ambient_light: AmbientLight,
	world_light: WorldLight,
	camera: Camera,
}

impl GLPresenter {
	pub fn new(view: Box<View>, world: Box<World>) -> GLPresenter {
		let frac_ambient = 0.05_f64;
		let frac_world = 1_f64 - frac_ambient;

		GLPresenter {
			view: view,
			world: world,
			inputs_held: HashSet::new(),
			settings: Default::default(),
			objects: Vec::new(),
			ambient_light: AmbientLight {
				color: Vector4::new(frac_ambient, frac_ambient, frac_ambient, 1_f64),
			},
			world_light: WorldLight {
				pos: Point3::new(1000_f64, 0_f64, 0_f64),
				color: Vector4::new(frac_world, frac_world, frac_world, 1_f64),
			},
			camera: Camera::new(
				Point3::new(2_f64, 0_f64, 0_f64),
				-Vector3::unit_x(),
				Vector3::unit_y(),
			),
		}
	}

	pub fn event_loop(&mut self) {
		let world_state = self.update_from_world().clone();
		self.init_view(&world_state);

		self.objects.push(MeshObject::new(Mesh::gen_sphere_mesh(
			self.view.get_facade(),
			1_u32,
			1_f64,
		)));

		let mut fps_track_start = Instant::now();
		let mut frame_count = 0_u32;

		let mut last_frame_start = Instant::now();
		loop {
			let this_frame_start = Instant::now();
			let secs_since_last_frame =
				duration_to_secs(&this_frame_start.duration_since(last_frame_start));

			self.update_from_view();
			if self.settings.quitting {
				self.world.save();
				break;
			}
			self.process_held_inputs(secs_since_last_frame);

			self.draw();

			// FPS tracker
			frame_count += 1;
			let fps_track_secs = duration_to_secs(&fps_track_start.elapsed());
			if fps_track_secs > 1_f64 {
				info!("FPS: {}", frame_count as f64 / fps_track_secs);
				fps_track_start = Instant::now();
				frame_count = 0;
			}

			// Maybe sleep for some time to match max fps
			let min_frame_duration =
				Duration::new(0u64, (1_000_000_000_f64 / self.settings.max_fps) as u32);
			if let Some(time_to_sleep) = min_frame_duration.checked_sub(this_frame_start.elapsed())
			{
				std::thread::sleep(time_to_sleep);
			}

			last_frame_start = this_frame_start;
		}
		info!("exiting presenter event loop");
	}

	fn update_from_world(&mut self) -> &WorldState {
		self.world.get_state()
	}

	/// Updates presenter from the events view has accumulated since this was last called
	fn update_from_view(&mut self) {
		for window_event in self.view.poll_events() {
			self.process_view_event(window_event);
		}
	}

	/// Processes a single event from view
	fn process_view_event(&mut self, window_event: WindowEvent) {
		use glium::glutin::{ElementState, MouseScrollDelta};

		match window_event {
			WindowEvent::CloseRequested => self.settings.quitting = true,
			WindowEvent::KeyboardInput {
				input:
					glium::glutin::KeyboardInput {
						state,
						virtual_keycode: Some(key),
						..
					},
				..
			} => {
				let input = InputType::Key(key);
				info!("KeyboardInput: {:?} {:?}", key, state);
				match state {
					ElementState::Pressed => {
						self.process_input(&input);
						self.inputs_held.insert(input);
					}
					ElementState::Released => {
						self.inputs_held.remove(&input);
					}
				}
			}
			WindowEvent::MouseInput { state, button, .. } => {
				let input = InputType::Mouse(button);
				info!("MouseInput: {:?} {:?}", button, state);
				match state {
					ElementState::Pressed => {
						self.process_input(&input);
						self.inputs_held.insert(input);
					}
					ElementState::Released => {
						self.inputs_held.remove(&input);
					}
				}
			}
			// TODO: Shouldn't skip over the process_input pipeline but...
			//       Need a more generic way to implement zoom for scroll wheel.
			//       No other input has an "amount"... and I want to preserve the pixel delta
			WindowEvent::MouseWheel { delta, .. } => {
				let dist = match delta {
					MouseScrollDelta::LineDelta(_, vert) => vert as f64 * 15_f64,
					MouseScrollDelta::PixelDelta(LogicalPosition { y, .. }) => y,
				};
				let zoom_factor = 1_f64 - dist / 300_f64; // TODO calibrate, add config
				info!("MouseWheel: {:?} ({}, {})", delta, dist, zoom_factor);
				self.zoom(zoom_factor);
			}
			//TODO WindowEvent::CursorMoved { position, .. } => {
			//...
			//}
			_ => (),
		}
	}

	// Fires when an input is first pressed. Used for actions that don't happen continually,
	// like switching camera modes.
	fn process_input(&mut self, input: &InputType) {
		if let Some(&ActionType::Instant(action)) = self.settings.bindings.get(input) {
			self.process_instant_action(&action);
		}
	}
	fn process_instant_action(&mut self, action: &InstantAction) {
		info!("Instant action fired: {:?}", action);
		match action {
			InstantAction::Quit => self.settings.quitting = true,
			InstantAction::MoveLight => (),
			InstantAction::Log => (),
		}
	}

	// Fires every frame an input is held (incl the first!), with the time since the last
	// frame. Used for actions that happen continually, like moving the camera.
	fn process_held_inputs(&mut self, frame_secs: f64) {
		let inputs_vec: Vec<InputType> = self.inputs_held.iter().map(|&it| it).collect();
		for &input in inputs_vec.iter() {
			if let Some(&ActionType::Continual(action)) = self.settings.bindings.get(&input) {
				self.process_continual_action(&action, frame_secs);
			}
		}
	}
	fn process_continual_action(&mut self, action: &ContinualAction, frame_secs: f64) {
		info!("Continual action fired: {:?} ({:?}s)", action, frame_secs);
		match action {
			ContinualAction::MoveForward => (),
			ContinualAction::MoveBackward => (),
			ContinualAction::MoveLeft => (),
			ContinualAction::MoveRight => (),
			ContinualAction::MoveUp => (),
			ContinualAction::MoveDown => (),
			ContinualAction::MoveFast => (), // TODO:Continual modifiers?
		};
	}

	// Zooms in on the globe by a given factor (will appear as 'factor' times larger)
	fn zoom(&mut self, factor: f64) {
		info!("Zoom! {}", factor); //TODO
	}

	fn draw(&self) {
		let view_mat = self.camera.view_mat();

		let aspect_ratio = self.view.get_aspect_ratio();
		let proj_mat =
			cgmath::perspective(self.settings.fov, aspect_ratio, 0.00001_f64, 100000_f64);

		self.view.draw(
			view_mat,
			proj_mat,
			&self.ambient_light,
			&self.world_light,
			&self.objects,
		);
	}

	// For setting the view up from scratch
	fn init_view(&mut self, state: &WorldState) {
		self.view.set_shaders(VERT_SHADER, FRAG_SHADER);
		self.view
			.set_title(&format!("Viewing the world of {}", state.name));
		self.view
			.set_texture_array(vec![conv_image_to_raw_image(&state.map.missing_image)]);
	}
}
