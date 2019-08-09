use image::DynamicImage;

use std::fmt;
use std::fs::File;
use std::io::BufReader;

use super::LatLong;

#[derive(Clone, Debug)]
pub struct MapBounds {
	pub min: LatLong,
	pub max: LatLong,
}
/*impl MapBounds {
	pub fn new(min_lat_deg: f64, max_lat_deg: f64, min_long_deg: f64, max_long_deg: f64) -> MapBounds {
		MapBounds {
			// TODO: double-check degrees/radians
			min: LatLong::new(min_lat_deg, min_long_deg),
			max: LatLong::new(max_lat_deg, max_long_deg),
		}
	}
}*/

#[derive(Debug)]
pub enum MapError {
	IOError(std::io::Error),
	ImageError(image::ImageError),
}
impl fmt::Display for MapError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				MapError::IOError(e) => e as &fmt::Display,
				MapError::ImageError(e) => e as &fmt::Display,
			}
		)
	}
}

#[derive(Clone)]
pub struct Map {
	file_name: String,
	pub image: DynamicImage,
	pub missing_image: DynamicImage,
	pub bounds: MapBounds,
}
impl Map {
	pub fn new(file_name: &str, missing_file: &str, bounds: MapBounds) -> Result<Map, MapError> {
		let file = File::open(file_name).map_err(|e| MapError::IOError(e))?;
		let image = image::load(BufReader::with_capacity(8192, file), image::PNG)
			.map_err(|e| MapError::ImageError(e))?;

		let missing_file = File::open(missing_file).map_err(|e| MapError::IOError(e))?;
		let missing_image = image::load(BufReader::with_capacity(8192, missing_file), image::PNG)
			.map_err(|e| MapError::ImageError(e))?;

		Ok(Map {
			file_name: file_name.to_string(),
			image: image,
			missing_image: missing_image,
			bounds: bounds,
		})
	}
}
impl fmt::Debug for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Map{{image: <image from {}>, bounds: {:?}}}",
			self.file_name, self.bounds
		)
	}
}
