#[macro_use]
use std;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::error::Error;

use sphericalPoint::SphericalPoint;

pub struct City {
	pub name: String,
	pub population: u32,
	pub country: String,
	pub latitude: f32,
	pub longitude: f32,
}
impl std::fmt::Debug for City {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,
		       "{}: pop {}, country {}, lat {}, long {}",
		       self.name,
		       self.population,
		       self.country,
		       self.latitude,
		       self.longitude)?;
		Ok(())
	}
}

pub fn read_cities_file(file_name: &str) -> Vec<City> {
	let mut cities: Vec<City> = Vec::new();

	let f = File::open(file_name).unwrap();
	let f = BufReader::new(f);

	for line in f.lines() {
		let line = line.unwrap();
		let columns: Vec<&str> = line.split(',').collect();
		let city = City {
			name: String::from(columns[0]),
			population: columns[1].parse::<u32>().unwrap(),
			country: String::from(columns[2]),
			latitude: columns[3].parse::<f32>().unwrap(),
			longitude: columns[4].parse::<f32>().unwrap(),
		};
		cities.push(city);
	}
	println!("{:?}", cities);
	cities
}

pub fn closest_city_to<'a>(latitude: &f32,
                           longitude: &f32,
                           cities: &'a Vec<City>)
                           -> Option<&'a City> {
	let sphPoint = SphericalPoint::from_lat_long(&1.0f32, latitude, longitude);
	let cmp_cities = |city1: &&City, city2: &&City| {
		let city1_loc = SphericalPoint::from_lat_long(&1.0f32, &city1.latitude, &city1.longitude);
		let dist_to_city1 = sphPoint.great_circle_distance(&city1_loc.theta, &city1_loc.phi);

		let city2_loc = SphericalPoint::from_lat_long(&1.0f32, &city2.latitude, &city2.longitude);
		let dist_to_city2 = sphPoint.great_circle_distance(&city2_loc.theta, &city2_loc.phi);

		dist_to_city1.partial_cmp(&dist_to_city2).unwrap()
	};
	cities.into_iter().min_by(cmp_cities)
}
