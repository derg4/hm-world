use crate::entities::Map;

#[derive(Clone, Debug)]
pub struct WorldState {
	pub name: String,
	pub map: Box<Map>,
}

pub trait World {
	fn get_state(&self) -> &WorldState;
	fn save(&self);
}
