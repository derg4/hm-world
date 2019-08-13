use cgmath::prelude::*;
use cgmath::{Deg, Rad};
use cgmath::{Point3, Vector3};

use std::ops::{Add, Sub};

// All conventions used here are from
// https://en.wikipedia.org/wiki/Spherical_coordinate_system#Conventions
//
// radius >= 0
// 0 <= theta <= PI
// 0 <= phi < 2*PI
pub struct SphericalPoint {
	pub radius: f64,
	pub theta: Rad<f64>,
	pub phi: Rad<f64>,
}
impl SphericalPoint {
	pub fn new(radius: f64, theta: Rad<f64>, phi: Rad<f64>) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: theta,
			phi: phi,
		}
	}

	pub fn from_point(point: &Point3<f64>) -> SphericalPoint {
		let radius: f64 = point.to_vec().magnitude();
		SphericalPoint {
			radius: radius,
			theta: Rad::acos(point.y / radius),
			phi: Rad(point.x.atan2(point.z)),
		}
	}

	pub fn from_vec(vec: &Vector3<f64>) -> SphericalPoint {
		let radius: f64 = vec.magnitude();
		SphericalPoint {
			radius: radius,
			theta: Rad::acos(vec.y / radius),
			phi: Rad(vec.x.atan2(vec.z)),
		}
	}

	pub fn to_point(&self) -> Point3<f64> {
		let x: f64 = self.radius * self.theta.sin() * self.phi.sin();
		let y: f64 = self.radius * self.theta.cos();
		let z: f64 = self.radius * self.theta.sin() * self.phi.cos();
		Point3::new(x, y, z)
	}

	pub fn to_vec(&self) -> Vector3<f64> {
		let x: f64 = self.radius * self.theta.sin() * self.phi.sin();
		let y: f64 = self.radius * self.theta.cos();
		let z: f64 = self.radius * self.theta.sin() * self.phi.cos();
		Vector3::new(x, y, z)
	}

	// TODO: TESTME
	pub fn from_lat_long(radius: f64, lat: Rad<f64>, long: Rad<f64>) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: (Rad::turn_div_4() - lat),
			phi: (Rad::turn_div_2() - long), // TODO change so 0 phi is 0 longitude
		}
	}
	pub fn as_lat_long(&self) -> LatLong {
		let lat = Rad::turn_div_4() - self.theta;
		let long = Rad::turn_div_2() - self.phi; //TODO change so 0 phi is 0 longitude
		LatLong::new(lat, long)
	}

	pub fn is_ok(&self) -> bool {
		!(self.radius.is_nan() || self.theta.0.is_nan() || self.phi.0.is_nan())
	}
}

// lat is in radians, [-PI/2 (S), PI/2 (N)]
// long is in radians, [-PI (W), PI (E))
#[derive(Clone, Debug)]
pub struct LatLong {
	pub lat: Rad<f64>,
	pub long: Rad<f64>,
}
impl LatLong {
	pub fn new<A: Into<Rad<f64>>>(latitude: A, longitude: A) -> LatLong {
		LatLong {
			lat: latitude.into(),
			long: longitude.into(),
		}
	}

	pub fn normalize(&self) -> LatLong {
		let Rad(lat) = self.lat;
		let new_lat = Rad(lat.min(std::f64::consts::FRAC_PI_2).max(-std::f64::consts::FRAC_PI_2));

		// TODO: doesn't cover cases where long is < -2*pi
		let Rad(long) = self.long;
		let new_long = Rad((long + std::f64::consts::PI) % (2_f64 * std::f64::consts::PI) -
		                   std::f64::consts::PI);

		LatLong::new(new_lat, new_long)
	}

	pub fn as_sph_point(&self, radius: f64) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: (Rad::turn_div_4() - self.lat),
			phi: (Rad::turn_div_2() - self.long),
		}
	}

	pub fn as_rad_floats(&self) -> (f64, f64) {
		let (Rad(lat), Rad(long)) = (self.lat, self.long);
		(lat, long)
	}

	pub fn as_deg_floats(&self) -> (f64, f64) {
		let deg_lat: Deg<f64> = self.lat.into();
		let deg_long: Deg<f64> = self.long.into();
		let (Deg(lat), Deg(long)) = (deg_lat, deg_long);
		(lat, long)
	}

	// Returns the great circle distance in radians between self and other
	//
	// Uses Vincenty formula from https://en.wikipedia.org/wiki/Great-circle_distance
	pub fn great_circle_distance(&self, other: &LatLong) -> Rad<f64> {
		let long_delta = Rad((self.long - other.long).0.abs());

		Rad(((other.lat.cos() * long_delta.sin()).powi(2) +
		     (self.lat.cos() * other.lat.sin() -
		      self.lat.sin() * other.lat.cos() * long_delta.cos())
				.powi(2))
			.sqrt()
			.atan2(self.lat.sin() * other.lat.sin() +
			       self.lat.cos() * other.lat.cos() * long_delta.cos()))
	}
}
impl Add for LatLong {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		LatLong::new(self.lat + other.lat, self.long + other.long).normalize()
	}
}
impl Sub for LatLong {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		LatLong::new(self.lat - other.lat, self.long - other.long).normalize()
	}
}
