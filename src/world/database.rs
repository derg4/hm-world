use crate::entities::MapError;

use super::WorldState;

use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
	IOError(std::io::Error),
	ConfigParseError(Box<std::error::Error>),
	ConfigMissingValue, // { value: String },
	ConfigValueWrongType, // { value: String, expected: String },
	MapError(MapError),
}
impl fmt::Display for DatabaseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DatabaseError({})", match self {
			DatabaseError::IOError(e) => format!("{}", e),
			DatabaseError::ConfigParseError(e) => format!("{}", e),
			DatabaseError::ConfigMissingValue => "ConfigMissingValue".to_string(),
			DatabaseError::ConfigValueWrongType => "ConfigValueWrongType".to_string(),
			DatabaseError::MapError(e) => format!("{}", e),
		})
	}
}

pub trait Database {
	fn load(&self) -> Result<WorldState, DatabaseError>;
	fn save(&self);
}
