use cgmath::prelude::*;
use cgmath::Rad;
use cgmath::{Point3, Vector3};

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
			phi: (Rad::turn_div_2() - long),
		}
	}
	pub fn as_lat_long(&self) -> LatLong {
		let lat = Rad::turn_div_4() - self.theta;
		let long = Rad::turn_div_2() - self.phi;
		LatLong::new(lat, long)
	}

	pub fn is_ok(&self) -> bool {
		!(self.radius.is_nan() || self.theta.0.is_nan() || self.phi.0.is_nan())
	}
}

// lat is in radians, -PI/2 (S) to PI/2 (N), inclusive
// long is in radians, -PI (W) to PI (E), inclusive
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

	// pub fn normalize(&mut self) { //TODO
	// match self.lat {
	// Rad(f64::MIN)...-Rad::turn_div_4() =>
	// }
	// }

	// pub fn add_lat<A: Into<Rad<f64>>>(&mut self, latitude: A) {
	// self.lat += latitude.into();
	// }

	pub fn as_sph_point(&self, radius: f64) -> SphericalPoint {
		SphericalPoint {
			radius: radius,
			theta: (Rad::turn_div_4() - self.lat),
			phi: (Rad::turn_div_2() - self.long),
		}
	}

	// Returns the great circle distance in radians between self and other
	//
	// Uses Vincenty formula from https://en.wikipedia.org/wiki/Great-circle_distance
	pub fn great_circle_distance(&self, other: &LatLong) -> Rad<f64> {
		let long_delta = Rad((self.long - other.long).0.abs());

		Rad(((other.lat.cos() * long_delta.sin()).powi(2)
			+ (self.lat.cos() * other.lat.sin()
				- self.lat.sin() * other.lat.cos() * long_delta.cos())
			.powi(2))
		.sqrt()
		.atan2(
			self.lat.sin() * other.lat.sin() + self.lat.cos() * other.lat.cos() * long_delta.cos(),
		))
	}
}
