use image::DynamicImage;

use std::fmt;

use super::LatLong;

#[derive(Clone, Debug)]
pub struct MapBounds {
	pub min: LatLong,
	pub max: LatLong,
}

#[derive(Clone)]
pub struct Map {
	world_name: String,
	pub image: DynamicImage,
	pub missing_image: DynamicImage,
	pub bounds: MapBounds,
}
impl Map {
	pub fn new(world_name: &str,
	           large_image: DynamicImage,
	           missing_image: DynamicImage,
	           bounds: MapBounds)
	           -> Map {

		Map {
			world_name: world_name.to_string(),
			image: large_image,
			missing_image: missing_image,
			bounds: bounds,
		}
	}
}
impl fmt::Debug for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
		       "Map{{image: <image from {}>, bounds: {:?}}}",
		       self.world_name,
		       self.bounds)
	}
}
