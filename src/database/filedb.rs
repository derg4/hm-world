use crate::entities::{LatLong, Map, MapBounds};
use crate::world::{Database, DatabaseError, WorldState};

use std::fs::File;
use std::io::Read;
use std::path::Path;

use cgmath::Deg;
use toml::Value; // XXX BAD!

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
}
impl Database for FileDatabase {
	fn load(&self) -> Result<WorldState, DatabaseError> {
		let value = self.load_config()?;
		info!("FileDB read config: {:?}", value);

		// TODO: implement a wrapper around toml, so when writing save code we don't have to
		// duplicate the format in code there?

		// World table info
		let world_table = value
			.get("world")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let name_val = world_table
			.get("name")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let name = name_val
			.as_str()
			.ok_or(DatabaseError::ConfigValueWrongType)?;

		// Map table info
		let map_table = value.get("map").ok_or(DatabaseError::ConfigMissingValue)?;
		let map_filename_val = map_table
			.get("filename")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let map_filename = map_filename_val
			.as_str()
			.ok_or(DatabaseError::ConfigValueWrongType)?;
		let map_missing_val = map_table
			.get("missing")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let map_missing = map_missing_val
			.as_str()
			.ok_or(DatabaseError::ConfigValueWrongType)?;

		let min_lat_val = map_table
			.get("min_lat")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let min_lat = min_lat_val
			.as_float()
			.ok_or(DatabaseError::ConfigValueWrongType)?;
		let max_lat_val = map_table
			.get("max_lat")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let max_lat = max_lat_val
			.as_float()
			.ok_or(DatabaseError::ConfigValueWrongType)?;

		let min_long_val = map_table
			.get("min_long")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let min_long = min_long_val
			.as_float()
			.ok_or(DatabaseError::ConfigValueWrongType)?;
		let max_long_val = map_table
			.get("max_long")
			.ok_or(DatabaseError::ConfigMissingValue)?;
		let max_long = max_long_val
			.as_float()
			.ok_or(DatabaseError::ConfigValueWrongType)?;

		//TODO check degrees/radians
		let map_bounds = MapBounds {
			min: LatLong::new(Deg(min_lat), Deg(min_long)),
			max: LatLong::new(Deg(max_lat), Deg(max_long)),
		};
		let map_file_path = Path::new(&self.config_file)
			.with_file_name(map_filename)
			.to_str()
			.ok_or(DatabaseError::IOError(std::io::Error::new(
				std::io::ErrorKind::NotFound,
				"Parent dir of config",
			)))?
			.to_string();
		let missing_file_path = Path::new(&self.config_file)
			.with_file_name(map_missing)
			.to_str()
			.ok_or(DatabaseError::IOError(std::io::Error::new(
				std::io::ErrorKind::NotFound,
				"Parent dir of config",
			)))?
			.to_string();
		let map = Map::new(&map_file_path, &missing_file_path, map_bounds)
			.map_err(|e| DatabaseError::MapError(e))?;

		Ok(WorldState {
			name: name.to_string(),
			map: Box::new(map),
		})
	}

	fn save(&self) {
		error!("Database save unimplemented!");
	}
}
