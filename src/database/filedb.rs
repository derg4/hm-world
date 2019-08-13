use crate::entities::{Map, MapBounds};
use crate::world::{Database, DatabaseError, WorldState};

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use image::DynamicImage;
use toml::Value;

fn get_image_from_file(file_name: &str) -> Result<DynamicImage, DatabaseError> {
	let file = File::open(file_name).map_err(|e| DatabaseError::IOError(e))?;
	Ok(image::load(BufReader::with_capacity(8192, file), image::PNG)
		.map_err(|e| DatabaseError::ImageError(e))?)
}

pub struct FileDatabase {
	config_file: String,
}
impl FileDatabase {
	pub fn new(config_file: &str) -> FileDatabase {
		FileDatabase {
			config_file: config_file.to_string(),
		}
	}

	fn load_config(&self) -> Result<Value, DatabaseError> {
		let mut file = File::open(&self.config_file).map_err(|e| DatabaseError::IOError(e))?;

		let mut config = String::new();
		file.read_to_string(&mut config)
			.map_err(|e| DatabaseError::IOError(e))?;

		config
			.parse::<Value>()
			.map_err(|e| DatabaseError::ConfigParseError(Box::new(e)))
	}

	fn value_get<'a>(value: &'a Value, key: &str) -> Result<&'a Value, DatabaseError> {
		Ok(value.get(key).ok_or(DatabaseError::ConfigMissingValue)?)
	}

	fn value_get_str<'a>(value: &'a Value, key: &str) -> Result<&'a str, DatabaseError> {
		Ok(Self::value_get(value, key)?.as_str().ok_or(DatabaseError::ConfigValueWrongType)?)
	}

	fn value_get_int(value: &Value, key: &str) -> Result<i64, DatabaseError> {
		Ok(Self::value_get(value, key)?.as_integer().ok_or(DatabaseError::ConfigValueWrongType)?)
	}

	fn value_get_float(value: &Value, key: &str) -> Result<f64, DatabaseError> {
		Ok(Self::value_get(value, key)?.as_float().ok_or(DatabaseError::ConfigValueWrongType)?)
	}

	fn value_get_bool(value: &Value, key: &str) -> Result<bool, DatabaseError> {
		Ok(Self::value_get(value, key)?.as_bool().ok_or(DatabaseError::ConfigValueWrongType)?)
	}
}
impl Database for FileDatabase {
	fn load(&self) -> Result<WorldState, DatabaseError> {
		let value = self.load_config()?;
		info!("FileDB read config: {:?}", value);

		// TODO: implement a wrapper around toml, so when writing save code we don't have to
		// duplicate the format in code there?

		// World table info
		let world = Self::value_get(&value, "world")?;
		let name = Self::value_get_str(&world, "name")?;

		// Map table info
		let map = Self::value_get(&value, "map")?;
		let map_filename = Self::value_get_str(&map, "filename")?;
		let min_lat = Self::value_get_float(&map, "min_lat")?;
		let max_lat = Self::value_get_float(&map, "max_lat")?;
		let min_long = Self::value_get_float(&map, "min_long")?;
		let max_long = Self::value_get_float(&map, "max_long")?;
		let missing_texture_file = Self::value_get_str(&map, "missing_texture")?;
		let texture_size_deg = Self::value_get_int(&map, "texture_size_deg")?;

		let map_bounds = MapBounds {
			min_lat: min_lat,
			max_lat: max_lat,
			min_long: min_long,
			max_long: max_long,
		};
		let config_path = Path::new(&self.config_file);
		let map_file_path = config_path.with_file_name(map_filename)
			.to_str()
			.ok_or(DatabaseError::IOError(std::io::Error::new(
				std::io::ErrorKind::NotFound,
				"Parent dir of config",
			)))?
			.to_string();
		let map_image = get_image_from_file(&map_file_path)?;
		let missing_file_path = config_path
			.with_file_name(missing_texture_file)
			.to_str()
			.ok_or(DatabaseError::IOError(std::io::Error::new(
				std::io::ErrorKind::NotFound,
				"Parent dir of config",
			)))?
			.to_string();
		let missing_image = get_image_from_file(&missing_file_path)?;
		let mut map = Map::new(&name, map_image, missing_image, map_bounds);
		info!("Loaded map!");

		let textures_dir = config_path.with_file_name(&format!("tex_{}", texture_size_deg));
		let dir_builder = std::fs::DirBuilder::new();
		if dir_builder.create(&textures_dir).is_err() {
			warn!("Dir builder couldn't create {:?}", textures_dir);
		}
		info!("foo");

		let map_textures = map.generate_textures(texture_size_deg as u32);
		for (map_piece_key, texture) in map_textures.iter() {
			let texture_filename = format!("{:+04}_{:+04}.png", map_piece_key.min_long, map_piece_key.min_lat);
			let texture_file_path = textures_dir.join(texture_filename);
			if texture.save_with_format(&texture_file_path, image::ImageFormat::PNG).is_err() {
				warn!("Texture {:?} could not be saved", texture_file_path);
			}
		}

		Ok(WorldState {
			name: name.to_string(),
			map: Box::new(map),
		})
	}

	fn save(&self) {
		error!("Database save unimplemented!");
	}
}
