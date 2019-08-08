extern crate cgmath;
extern crate glium;
extern crate image;

#[macro_use]
extern crate log;
use log::{Record, Level, LevelFilter, Metadata};

extern crate rand;
extern crate toml;

mod presenter;
use presenter::GLPresenter;

mod view;
use view::GLView;

mod world;
use world::ConcreteWorld;

mod database;
use database::FileDatabase;

mod entities;

static LOGGER: SimpleLogger = SimpleLogger;
struct SimpleLogger;
impl log::Log for SimpleLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			println!("{} - {}", record.level(), record.args());
		}
	}

	fn flush(&self) {}
}

fn main() {
	log::set_logger(&LOGGER).unwrap();
	log::set_max_level(LevelFilter::Info);

	let database = FileDatabase::new("worlds/tellene/config.toml");

	let world = match ConcreteWorld::new(Box::new(database)) {
		Ok(world) => world,
		Err(err) => {
			error!("Main: Error loading world: {}", err);
			return;
		}
	};

	let view = match GLView::new() {
		Ok(view) => view,
		Err(err) => {
			error!("Main: Error initializing view: {}", err);
			return;
		},
	};

	let mut presenter = GLPresenter::new(Box::new(view), Box::new(world));

	presenter.event_loop();

	info!("Main is returning");
}
