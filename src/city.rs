#[macro_use]
use std::fs::File;
use std::io::{BufRead, BufReader};

use coords::LatLong;

pub struct City {
	pub name: String,
	pub population: u32,
	pub country: String,
	pub coords: LatLong,
}
impl std::fmt::Debug for City {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,
		       "{}: pop {}, country {}, lat {}, long {}",
		       self.name,
		       self.population,
		       self.country,
		       self.coords.lat,
		       self.coords.long)?;
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
			coords: LatLong::new(columns[3].parse::<f32>().unwrap(),
			                     columns[4].parse::<f32>().unwrap()),
		};
		cities.push(city);
	}
	println!("{:?}", cities);
	cities
}

pub fn closest_city_to<'a>(coords: &LatLong,
                           cities: &'a Vec<City>)
                           -> Option<&'a City> {
	let cmp_cities = |city1: &&City, city2: &&City| {
		let city1_dist = coords.great_circle_distance(&city1.coords);
		let city2_dist = coords.great_circle_distance(&city2.coords);
		city1_dist.partial_cmp(&city2_dist).unwrap()
	};
	cities.into_iter().min_by(cmp_cities)
}
