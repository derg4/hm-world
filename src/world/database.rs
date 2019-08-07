use crate::entities::MapError;

use super::WorldState;

#[derive(Debug)]
pub enum DatabaseError {
	IOError(std::io::Error),
	ConfigParseError(Box<std::error::Error>),
	ConfigMissingValue, // { value: String },
	ConfigValueWrongType, // { value: String, expected: String },
	MapError(MapError),
}

pub trait Database {
	fn load(&self) -> Result<WorldState, DatabaseError>;
}
