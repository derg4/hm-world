extern crate log;

use super::{Vertex, View};
use crate::world::{World, WorldState};

const VERT_SHADER: &str = include_str!["vertex.glsl"];
const FRAG_SHADER: &str = include_str!["fragment.glsl"];

pub struct GLPresenter {
	view: Box<View>,
	world: Box<World>,
}

impl GLPresenter {
	pub fn new(view: Box<View>, world: Box<World>) -> GLPresenter {
		let mut presenter = GLPresenter{
			view: view,
			world: world,
		};

		let world_state = (*presenter.update_from_world()).clone();
		//let world_state = presenter.update_from_world()
		presenter.init_view(&world_state);
		presenter
	}

	pub fn event_loop(&mut self) {
		std::thread::sleep(std::time::Duration::from_secs(10));
		warn!("presenter is dying");
	}

	fn update_from_world(&mut self) -> &WorldState {
		self.world.get_state()
	}

	/// For setting the view up from scratch
	fn init_view(&mut self, state: &WorldState) {
		//Empty events? self.view.poll_events();
		self.view.set_shaders(VERT_SHADER, FRAG_SHADER);
		self.view.set_title(&format!("Viewing the world of {}", state.name));
	}
}
