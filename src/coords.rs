extern crate cgmath;
use std;
use cgmath::{EuclideanSpace, MetricSpace};
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
	pub fn new(radius: f32, theta: f32, phi: f32) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: theta,
			phi: phi,
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
	pub fn from_lat_long(radius: f32, lat: f32, long: f32) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: (std::f32::consts::FRAC_PI_2 - lat),
			phi: (std::f32::consts::PI - long),
		}
	}
	pub fn as_lat_long(&self) -> LatLong {
		let lat = std::f32::consts::FRAC_PI_2 - self.theta;
		let long = std::f32::consts::PI - self.phi;
		LatLong::new(lat, long)
	}

	pub fn is_ok(&self) -> bool {
		!(self.radius.is_nan() || self.theta.is_nan() || self.phi.is_nan())
	}
}

pub struct LatLong {
	pub lat: f32,
	pub long: f32,
}
impl LatLong {
	pub fn new(latitude: f32, longitude: f32) -> LatLong {
		LatLong {
			lat: latitude,
			long: longitude,
		}
	}

	pub fn as_sph_point(&self, radius: f32) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: (std::f32::consts::FRAC_PI_2 - self.lat),
			phi: (std::f32::consts::PI - self.long),
		}
	}

	// Returns the great circle distance in radians between self and other
	//
	// Uses Vincenty formula from https://en.wikipedia.org/wiki/Great-circle_distance
	pub fn great_circle_distance(&self, other: &LatLong) -> f32 {
		let long_delta = (self.long - other.long).abs();

		((other.lat.cos() * long_delta.sin()).powi(2) +
		 (self.lat.cos() * other.lat.sin() -
		  self.lat.sin() * other.lat.cos() * long_delta.cos()).powi(2))
			.sqrt()
			.atan2(self.lat.sin() * other.lat.sin() +
			       self.lat.cos() * other.lat.cos() * long_delta.cos())
	}
}
