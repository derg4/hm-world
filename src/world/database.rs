use super::WorldState;

use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
	ConfigParseError(Box<std::error::Error>),
	ConfigMissingValue,
	ConfigValueWrongType,
	ImageError(image::ImageError),
	IOError(std::io::Error),
}
impl fmt::Display for DatabaseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
		       "DatabaseError({})",
		       match self {
			       DatabaseError::ConfigParseError(e) => format!("{}", e),
			       DatabaseError::ConfigMissingValue => "ConfigMissingValue".to_string(),
			       DatabaseError::ConfigValueWrongType => "ConfigValueWrongType".to_string(),
			       DatabaseError::ImageError(e) => format!("{}", e),
			       DatabaseError::IOError(e) => format!("{}", e),
		       })
	}
}

pub trait Database {
	fn load(&self) -> Result<WorldState, DatabaseError>;
	fn save(&self);
}
