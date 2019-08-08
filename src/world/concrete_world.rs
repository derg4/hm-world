use super::{Database, DatabaseError, World, WorldState};

extern crate log;

// =============================================================================

pub struct ConcreteWorld {
	database: Box<Database>,
	state: WorldState,
}

impl ConcreteWorld {
	pub fn new(database: Box<Database>) -> Result<ConcreteWorld, DatabaseError> {
		let state = database.load()?;
		info!("World loaded");

		Ok(ConcreteWorld {
			database: database,
			state: state,
		})
	}
}
impl World for ConcreteWorld {
	fn get_state(&self) -> &WorldState {
		&self.state
	}

	fn save(&self) {
		self.database.save();
	}
}
