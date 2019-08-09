use super::Mesh;
use crate::entities::SphericalPoint;

use cgmath::{Angle, InnerSpace, Matrix4, Point2, Point3, Rad, SquareMatrix, Vector3, Vector4};

/// Representing any object in the game world which it makes sense to be able to scale
pub trait Scalable {
	fn incr_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self;
	fn set_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self;
}
/// Representing any object in the game world which it makes sense to be able to rotate
pub trait Rotatable {
	fn incr_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self;
	fn set_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self;
}
/// Representing any object in the game world which it makes sense to be able to translate
pub trait Translatable {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self;
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self;
}

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
	pub fn model_mat(&self) -> Matrix4<f64> {
		self.scale_mat * self.rotation_mat * self.translation_mat
	}
}
impl Scalable for MeshObject {
	fn incr_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
		self.scale_mat = self.scale_mat * Matrix4::from_nonuniform_scale(x, y, z);
		self
	}
	fn set_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
		self.scale_mat = Matrix4::from_nonuniform_scale(x, y, z);
		self
	}
}
impl Rotatable for MeshObject {
	fn incr_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.rotation_mat = self.rotation_mat * Matrix4::from_axis_angle(axis, angle);
		self
	}
	fn set_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.rotation_mat = Matrix4::from_axis_angle(axis, angle);
		self
	}
}
impl Translatable for MeshObject {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.translation_mat = self.translation_mat * Matrix4::from_translation(v);
		self
	}
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.translation_mat = Matrix4::from_translation(v);
		self
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
impl Translatable for WorldLight {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.pos = self.pos + v;
		self
	}
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.pos = Point3::new(v.x, v.y, v.z);
		self
	}
}

#[derive(Debug)]
pub struct Camera {
	pos: Point3<f64>,
	dir: Vector3<f64>,
	up: Vector3<f64>,
}
impl Camera {
	pub fn new(pos: Point3<f64>, dir: Vector3<f64>, up: Vector3<f64>) -> Camera {
		Camera {
			pos: pos,
			dir: dir,
			up: up,
		}
	}
	pub fn view_mat(&self) -> Matrix4<f64> {
		Matrix4::look_at(self.pos, self.pos + self.dir, self.up)
	}

	/*fn move(&mut self, vec: Vector3<f64>) {
		self.pos += vec;
	}
	fn rotate(&mut self, angles: Point2<Rad<f64>>) {
		// TODO self.dir = Matrix3::from_axis_angle(axis, angle) * self.dir;
	}*/
	pub fn rotate_about_point(&mut self, about: Point3<f64>, angles: Point2<Rad<f64>>) {
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
	pub fn zoom(&mut self, to_point: Point3<f64>, to_dist: f64, factor: f64) {
		let zoom_target = to_point + to_dist * (self.pos - to_point).normalize();
		self.pos = zoom_target + (self.pos - zoom_target) / factor;
	}
}
