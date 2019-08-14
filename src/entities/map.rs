use image::imageops;
use image::{DynamicImage, FilterType, GenericImage, GenericImageView};

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct MapBounds {
	pub min_lat: f64, // All in degrees
	pub max_lat: f64,
	pub min_long: f64,
	pub max_long: f64,
}
impl MapBounds {
	pub fn new(min_lat: f64, max_lat: f64, min_long: f64, max_long: f64) -> MapBounds {
		MapBounds {
			min_lat: min_lat,
			max_lat: max_lat,
			min_long: min_long,
			max_long: max_long,
		}
	}
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct MapPieceKey {
	pub min_lat: i32, // In degrees
	pub min_long: i32,
}

#[derive(Clone)]
pub struct Map {
	world_name: String,
	pub image: DynamicImage,
	pub missing_image: DynamicImage,
	pub bounds: MapBounds,
}
impl Map {
	pub fn new(
		world_name: &str,
		large_image: DynamicImage,
		missing_image: DynamicImage,
		bounds: MapBounds,
	) -> Map {
		Map {
			world_name: world_name.to_string(),
			image: large_image,
			missing_image: missing_image,
			bounds: bounds,
		}
	}

	fn get_img_x_by_long(&self, long_deg: f64) -> i32 {
		let px_per_deg_long =
			(self.image.width() as f64 - 1_f64) / (self.bounds.max_long - self.bounds.min_long);
		((long_deg - self.bounds.min_long) * px_per_deg_long).round() as i32
	}

	fn get_img_y_by_lat(&self, lat_deg: f64) -> i32 {
		let px_per_deg_lat =
			(self.image.height() as f64 - 1_f64) / (self.bounds.max_lat - self.bounds.min_lat);
		self.image.height() as i32
			- 1_i32 - ((lat_deg - self.bounds.min_lat) * px_per_deg_lat).round() as i32
	}

	pub fn generate_textures(&mut self, tex_size_deg: u32) -> HashMap<MapPieceKey, DynamicImage> {
		// "Increments" of tex_size_deg, set to hold whole image.
		// E.g. if self.image goes from long -13.7 to 12.0, with tex_size of 2 deg,
		// Min long incr would be -7, max would be 6
		let min_long_incr = (self.bounds.min_long / tex_size_deg as f64).floor() as i32;
		let max_long_incr = (self.bounds.max_long / tex_size_deg as f64).ceil() as i32 - 1_i32;
		let min_lat_incr = (self.bounds.min_lat / tex_size_deg as f64).floor() as i32;
		let max_lat_incr = (self.bounds.max_lat / tex_size_deg as f64).ceil() as i32 - 1_i32;

		let mut textures: HashMap<MapPieceKey, DynamicImage> = HashMap::new();

		// Goal in each iteration of the inner loop is to get a texture which is
		// tex_size_deg degrees wide and high, the size of self.missing_image
		debug!("Image w={}, h={}", self.image.width(), self.image.height());
		info!("Generating textures (may take a while)");
		for long_incr in min_long_incr..=max_long_incr {
			// Get x dimensions (in image-space pixels) for the subimage we'll grab
			let subimage_min_long = (long_incr * tex_size_deg as i32) as f64;
			// If min x coord would be < 0, set to 0 and store the offset
			let (subimage_min_x, offset_min_x) = {
				let min = self.get_img_x_by_long(subimage_min_long);
				if min < 0 {
					(0_u32, (-min) as u32)
				} else {
					(min as u32, 0_u32)
				}
			};

			let subimage_max_long = subimage_min_long + tex_size_deg as f64;
			// Max coords non-inclusive
			// If max x coord would be > width, set to width and store the offset
			let (subimage_max_x, offset_max_x) = {
				let max = self.get_img_x_by_long(subimage_max_long);
				if max > self.image.width() as i32 {
					(self.image.width(), (max as u32 - self.image.width()))
				} else {
					(max as u32, 0_u32)
				}
			};
			let subimage_width_px = subimage_max_x - subimage_min_x;
			debug!(
				"long incr={}, minlong x={}+{}, maxlong x={}, subimage w={}, img width={}",
				long_incr,
				subimage_min_x,
				offset_min_x,
				subimage_max_long,
				subimage_width_px,
				self.image.width()
			);

			for lat_incr in min_lat_incr..=max_lat_incr {
				// Get y dimensions ...
				let subimage_min_lat = (lat_incr * tex_size_deg as i32) as f64;
				// If min y coord would be < 0, set to 0 and store the offset
				// Also, min lat -> max y, because image coords are weird!
				let (subimage_max_y, offset_max_y) = {
					let max = self.get_img_y_by_lat(subimage_min_lat);
					if max > self.image.height() as i32 {
						(self.image.height(), (max as u32 - self.image.height()))
					} else {
						(max as u32, 0_u32)
					}
				};

				let subimage_max_lat = subimage_min_lat + tex_size_deg as f64;
				// If max y coord would be > height, set to height and store the offset
				// Also, max lat -> min y, because image coords are weird!
				let (subimage_min_y, offset_min_y) = {
					let min = self.get_img_y_by_lat(subimage_max_lat);
					if min < 0 {
						(0_u32, (-min) as u32)
					} else {
						(min as u32, 0_u32)
					}
				};
				let subimage_height_px = subimage_max_y - subimage_min_y;

				// Grab a view into image based on dimensions above
				debug!(
					"Subimage: minx={}, miny={}, w={}, h={}",
					subimage_min_x, subimage_min_y, subimage_width_px, subimage_height_px
				);
				let subimage = self.image.sub_image(
					subimage_min_x,
					subimage_min_y,
					subimage_width_px,
					subimage_height_px,
				);

				// Set up missing_img (bottom layer)
				let mut missing_img = self.missing_image.clone();

				// Get the multipliers needed to bring subimage up to missing_img's size
				let resize_x_mult = missing_img.width() as f64
					/ (subimage_width_px + offset_min_x + offset_max_x) as f64;
				let resize_y_mult = missing_img.height() as f64
					/ (subimage_height_px + offset_min_y + offset_max_y) as f64;

				// Resize subimage
				let resize_to_x = (subimage_width_px as f64 * resize_x_mult).round() as u32;
				let resize_to_y = (subimage_height_px as f64 * resize_y_mult).round() as u32;
				let resized_subimage =
					imageops::resize(&subimage, resize_to_x, resize_to_y, FilterType::Lanczos3);

				// Get offsets of subimage and overlay
				let tex_x_offset = (offset_min_x as f64 * resize_x_mult).round() as u32;
				let tex_y_offset = (offset_min_y as f64 * resize_y_mult).round() as u32;
				imageops::overlay(
					&mut missing_img,
					&resized_subimage,
					tex_x_offset,
					tex_y_offset,
				);

				// Store it and continue!
				let map_piece_key = MapPieceKey {
					min_lat: lat_incr * tex_size_deg as i32,
					min_long: long_incr * tex_size_deg as i32,
				};
				textures.insert(map_piece_key, missing_img);
			}
		}
		textures
	}
}
impl fmt::Debug for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Map{{image: <image from {}>, bounds: {:?}}}",
			self.world_name, self.bounds
		)
	}
}
