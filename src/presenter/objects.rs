use super::Mesh;
use crate::entities::SphericalPoint;

use cgmath::prelude::*;
use cgmath::{Matrix4, Point2, Point3, Rad, Vector3, Vector4};

/// An object in the game world, which has a mesh
#[derive(Debug)]
pub struct MeshObject {
	pub mesh: Mesh,
	scale_mat: Matrix4<f64>,
	rotation_mat: Matrix4<f64>,
	translation_mat: Matrix4<f64>,
}
impl MeshObject {
	pub fn new(mesh: Mesh) -> MeshObject {
		MeshObject {
			mesh: mesh,
			scale_mat: Matrix4::identity(),
			rotation_mat: Matrix4::identity(),
			translation_mat: Matrix4::identity(),
		}
	}

	//TODO: remove these attributes when dynamic game objects are introduced
	#[allow(dead_code)] // scale/rotate/translate aren't used because world is static atm
	fn incr_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
		self.scale_mat = self.scale_mat * Matrix4::from_nonuniform_scale(x, y, z);
		self
	}
	#[allow(dead_code)] // scale/rotate/translate aren't used because world is static atm
	fn set_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
		self.scale_mat = Matrix4::from_nonuniform_scale(x, y, z);
		self
	}

	#[allow(dead_code)] // scale/rotate/translate aren't used because world is static atm
	fn incr_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.rotation_mat = self.rotation_mat * Matrix4::from_axis_angle(axis, angle);
		self
	}
	#[allow(dead_code)] // scale/rotate/translate aren't used because world is static atm
	fn set_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.rotation_mat = Matrix4::from_axis_angle(axis, angle);
		self
	}

	#[allow(dead_code)] // scale/rotate/translate aren't used because world is static atm
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.translation_mat = self.translation_mat * Matrix4::from_translation(v);
		self
	}
	#[allow(dead_code)] // scale/rotate/translate aren't used because world is static atm
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.translation_mat = Matrix4::from_translation(v);
		self
	}

	pub fn model_mat(&self) -> Matrix4<f64> {
		self.scale_mat * self.rotation_mat * self.translation_mat
	}
}

/// An ambient light, which colors every fragment equally
#[derive(Debug)]
pub struct AmbientLight {
	pub color: Vector4<f64>,
}

/// A point light with a location in the world
#[derive(Debug)]
pub struct WorldLight {
	pub pos: Point3<f64>,
	pub color: Vector4<f64>,
}

#[derive(Debug)]
struct LockedData {
	pub rotate_point: Point3<f64>,
	pub zoom_dist: f64,
}

#[derive(Debug)]
pub struct Camera {
	pos: Point3<f64>,
	dir: Vector3<f64>,
	up: Vector3<f64>,
	lock: Option<LockedData>,

	move_speed: f64, // World units per second when moving unlocked
	pan_speed: f64,  // Rad/s to rotate panning locked (at one radius dist at equator)
	zoom_speed: f64, // Zoom factor per second when zooming locked
}
impl Camera {
	pub fn new(pos: Point3<f64>, dir: Vector3<f64>, up: Vector3<f64>, move_speed: f64, pan_speed: f64, zoom_speed: f64) -> Camera {
		info!("Camera initialized, pos {:?}, dir {:?}", pos, dir);
		Camera {
			pos: pos,
			dir: dir,
			up: up,
			lock: None,
			move_speed: move_speed,
			pan_speed: pan_speed,
			zoom_speed: zoom_speed,
		}
	}

	pub fn lock(&mut self, to_point: Point3<f64>, zoom_dist: f64) {
		self.lock = Some(LockedData { rotate_point: to_point, zoom_dist: zoom_dist });
		self.dir = (to_point - self.pos).normalize();
	}
	pub fn unlock(&mut self) {
		self.lock = None;
	}
	pub fn is_locked(&self) -> bool {
		self.lock.is_some()
	}

	pub fn view_mat(&self) -> Matrix4<f64> {
		Matrix4::look_at(self.pos, self.pos + self.dir, self.up)
	}
	pub fn get_pos(&self) -> Point3<f64> {
		self.pos
	}

	/// If the camera is unlocked, will 'move' vec distance in world-space relative to the
	/// direction the camera is facing.
	///
	/// If the camera is locked, will 'pan' over and 'zoom' to the locked point.
	///
	/// In either case,
	/// +vec.x = right, +vec.y = up, +vec.z = backwards XXX
	///
	/// The camera will move using the appropriate speed for the type of movement,
	/// using the amount of time elapsed supplied.
	pub fn move_cam(&mut self, vec: Vector3<f64>, elapsed: f64) {
		match self.lock {
			Some(LockedData {rotate_point, zoom_dist}) => {
				// "Pan" movement
				let angles = self.correct_rot(rotate_point, zoom_dist,
					Point2::new(Rad(elapsed * self.pan_speed * vec.x),
					            Rad(elapsed * self.pan_speed * vec.z)));
				debug!("Rotating phi={:?}, theta={:?} over {}s", angles.x, angles.y, elapsed);
				self.rotate_locked(rotate_point, angles);

				// "Zoom" movement (avoid div by 0)
				if vec.y.abs() > 0.01_f64 {
					let zoom_factor = if vec.y < 0_f64 {
						1_f64 / (self.zoom_speed * -vec.y).powf(elapsed)
					}
					else {
						(self.zoom_speed * vec.y).powf(elapsed)
					};
					debug!("Zooming a factor of {} over {}s", zoom_factor, elapsed);
					self.zoom(rotate_point, zoom_dist, zoom_factor);
				}
			},
			None => {
				// "Move" movement
				let right = self.dir.cross(self.up).normalize();
				self.pos += self.move_speed * elapsed *
					(right * vec.x + self.up * vec.y - self.dir * vec.z)
			},
		}
	}

	// Sets the (longitudinal) rotation multiplier due to latitude.
	// Basically, rotating x radians at the equator is faster compared to rotating the
	// same angle near the poles, more ground moves beneath you
	fn correct_rot(&self, about: Point3<f64>, zoom_dist: f64, angles: Point2<Rad<f64>>) -> Point2<Rad<f64>> {
		let sph_disp = SphericalPoint::from_vec(&(self.pos - about));
		let long_rot_mult = 1_f64 / sph_disp.theta.sin().max(0.25f64); // XXX Hardcoded max speedup
		// Slows down rotation closer the camera is to the sphere
		let dist_mult = sph_disp.radius - zoom_dist;
		Point2::new(angles.x * long_rot_mult * dist_mult, angles.y * dist_mult)
	}

	// Rotates around "about" by the given angles (raw, no corrections except stopping at
	// min/max latitude)
	fn rotate_locked(&mut self, about: Point3<f64>, angles: Point2<Rad<f64>>) {
		let mut sph_disp = SphericalPoint::from_vec(&(self.pos - about));

		sph_disp.theta += angles.y;
		let min_theta = Rad(0.00001_f64); // magic number
		let max_theta = Rad::turn_div_2() - min_theta;
		if sph_disp.theta < min_theta {
			sph_disp.theta = min_theta;
		} else if sph_disp.theta > max_theta {
			sph_disp.theta = max_theta;
		}

		sph_disp.phi = (sph_disp.phi + angles.x).normalize();

		if !sph_disp.is_ok() {
			// TODO set to a normal default
			error!(
				"Resetting camera, bad spherical coord values: r {} | theta {:?} | phi {:?}",
				sph_disp.radius, sph_disp.theta, sph_disp.phi
			);
			sph_disp.radius = 2_f64;
			sph_disp.theta = Rad::turn_div_4();
			sph_disp.phi = Rad(0_f64);
		}

		let new_disp = sph_disp.to_vec();

		self.pos = about + new_disp;
		self.dir = -new_disp.normalize();
	}

	fn zoom(&mut self, to_point: Point3<f64>, to_dist: f64, factor: f64) {
		let zoom_target = to_point + to_dist * (self.pos - to_point).normalize();
		self.pos = zoom_target + (self.pos - zoom_target) / factor;
	}
}
