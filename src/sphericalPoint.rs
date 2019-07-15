extern crate cgmath;
use std;
use cgmath::{EuclideanSpace, InnerSpace, MetricSpace};
use cgmath::{Point3, Vector3};

// All conventions used here are from
// https://en.wikipedia.org/wiki/Spherical_coordinate_system#Conventions
//
// radius >= 0
// 0 <= theta <= PI
// 0 <= phi < 2*PI
pub struct SphericalPoint {
	pub radius: f32,
	pub theta: f32,
	pub phi: f32,
}
impl SphericalPoint {
	pub fn new(radius: &f32, theta: &f32, phi: &f32) -> SphericalPoint {
		SphericalPoint {
			radius: *radius,
			theta: *theta,
			phi: *phi,
		}
	}

	pub fn from_point(point: &Point3<f32>) -> SphericalPoint {
		let radius: f32 = point.to_vec().distance(Vector3::new(0f32, 0f32, 0f32));
		SphericalPoint {
			radius: radius,
			theta: (point.y / radius).acos(),
			phi: point.x.atan2(point.z),
		}
	}

	pub fn to_point(&self) -> Point3<f32> {
		let x: f32 = self.radius * self.theta.sin() * self.phi.sin();
		let y: f32 = self.radius * self.theta.cos();
		let z: f32 = self.radius * self.theta.sin() * self.phi.cos();
		Point3::new(x, y, z)
	}

	// lat is in radians, -PI/2 (S) to PI/2 (N), inclusive
	// long is in radians, -PI (W) to PI (E), inclusive
	// TODO: TESTME
	pub fn from_lat_long(radius: &f32, lat: &f32, long: &f32) -> SphericalPoint {
		SphericalPoint {
			radius: *radius,
			theta: (std::f32::consts::FRAC_PI_2 - lat),
			phi: (std::f32::consts::PI - long),
		}
	}
	pub fn to_lat_long(&self) -> (f32, f32) {
		let lat = std::f32::consts::FRAC_PI_2 - self.theta;
		let long = std::f32::consts::PI - self.phi;
		(lat, long)
	}

	pub fn is_ok(&self) -> bool {
		!(self.radius.is_nan() || self.theta.is_nan() || self.phi.is_nan())
	}

	// Computes the great circle distance between two spherical points:
	// 1. self
	// 2. another point with the same radius as self, and o_theta, and o_phi
	//
	// Uses Vincenty formula from https://en.wikipedia.org/wiki/Great-circle_distance
	// TODO: TESTME
	// TODO: Great ellipse instead of great circle distance?
	// See: https://en.wikipedia.org/wiki/Vincenty's_formulae
	pub fn great_circle_distance(&self, o_theta: &f32, o_phi: &f32) -> f32 {
		let other = SphericalPoint::new(&self.radius, o_theta, o_phi);
		let (lat_s, long_s) = self.to_lat_long();
		let (lat_o, long_o) = other.to_lat_long();
		let long_delta = (long_s - long_o).abs();

		self.radius *
		((lat_o.cos() * long_delta.sin()).powi(2) +
		 (lat_s.cos() * lat_o.sin() - lat_s.sin() * lat_o.cos() * long_delta.cos()).powi(2))
			.sqrt()
			.atan2(lat_s.sin() * lat_o.sin() + lat_s.cos() * lat_o.cos() * long_delta.cos())
	}
}
