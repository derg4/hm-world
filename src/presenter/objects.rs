use super::Mesh;

use cgmath::{Matrix3, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4};

pub trait Scalable {
	fn incr_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self;
	fn set_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self;
}
pub trait Rotatable {
	fn incr_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self;
	fn set_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self;
}
pub trait Translatable {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self;
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self;
}

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

#[derive(Debug)]
pub struct AmbientLight {
	pub color: Vector4<f64>,
}

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

#[derive(Debug)]
pub struct Camera {
	pub position: Point3<f64>,
	initial_direction: Vector3<f64>,
	pub direction: Vector3<f64>,
	pub up: Vector3<f64>,
}
impl Rotatable for Camera {
	fn incr_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.direction = Matrix3::from_axis_angle(axis, angle) * self.direction;
		self
	}
	fn set_rotate<A: Into<Rad<f64>>>(&mut self, axis: Vector3<f64>, angle: A) -> &mut Self {
		self.direction = Matrix3::from_axis_angle(axis, angle) * self.initial_direction;
		self
	}
}
impl Translatable for Camera {
	fn incr_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.position = self.position + v;
		self
	}
	fn set_translate(&mut self, v: Vector3<f64>) -> &mut Self {
		self.position = Point3::new(v.x, v.y, v.z);
		self
	}
}
