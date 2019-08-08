use super::Mesh;
use crate::entities::LatLong;

use cgmath::{Angle, Matrix3, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4};

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
	fn new(mesh: Mesh) -> MeshObject {
		MeshObject {
			mesh: mesh,
			scale_mat: Matrix4::identity(),
			rotation_mat: Matrix4::identity(),
			translation_mat: Matrix4::identity(),
		}
	}
	fn get_transformation(&self) -> Matrix4<f64> {
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
	pub position: Point3<f64>,
	pub color: Vector4<f64>,
}
impl Translatable for WorldLight {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.position = self.position + v;
		self
	}
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.position = Point3::new(v.x, v.y, v.z);
		self
	}
}

/// The representation of a camera: translate moves the eye, rotate changes the look direction
#[derive(Debug)]
pub struct FreeCamera {
	pub position: Point3<f64>,
	initial_direction: Vector3<f64>,
	pub direction: Vector3<f64>,
	pub up: Vector3<f64>,
}
impl Rotatable for FreeCamera {
	fn incr_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.direction = Matrix3::from_axis_angle(axis, angle) * self.direction;
		self
	}
	fn set_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.direction = Matrix3::from_axis_angle(axis, angle) * self.initial_direction;
		self
	}
}
impl Translatable for FreeCamera {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.position = self.position + v;
		self
	}
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.position = Point3::new(v.x, v.y, v.z);
		self
	}
}

#[derive(Debug)]
pub struct LockedCamera {
	pub coords: LatLong,
	pub up: Vector3<f64>,
	pub dist_from_center: f64,
	globe_center: Point3<f64>,
	globe_radius: f64,
	min_dist_from_surface: f64,
	max_latitude: Rad<f64>,
}
impl LockedCamera {
	// TODO build camera based on a reference to a world or something?
	fn new(coords: LatLong, up: Vector3<f64>, dist_from_center: f64, globe_center: Point3<f64>, globe_radius: f64, min_dist_from_surface: f64, max_latitude: Rad<f64>) -> LockedCamera {
		LockedCamera {
			coords: coords,
			up: up,
			dist_from_center: dist_from_center,
			globe_center: globe_center,
			globe_radius: globe_radius,
			min_dist_from_surface: min_dist_from_surface,
			max_latitude: max_latitude,
		}
	}
	fn add_latitude<A: Into<Rad<f64>>>(&mut self, add_lat: A) {
		let sum_lat = self.coords.lat + add_lat.into();
		if sum_lat > self.max_latitude {
			self.coords.lat = self.max_latitude;
		}
		else if sum_lat < -self.max_latitude {
			self.coords.lat = -self.max_latitude;
		}
		else {
			self.coords.lat = sum_lat;
		}
	}
	fn add_longitude<A: Into<Rad<f64>>>(&mut self, add_long: A) {
		self.coords.long = (self.coords.long + add_long.into()).normalize_signed();
	}
	fn zoom(&mut self, factor: f64) {
		self.dist_from_center = ((self.dist_from_center - self.globe_radius) / factor)
			.min(self.min_dist_from_surface) + self.globe_radius;
	}
	//fn config_free_camera(&self, free_camera: &mut FreeCamera) { }
}
