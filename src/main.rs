extern crate cgmath;
extern crate glium;
extern crate image;
#[macro_use]
extern crate log;
use log::{Record, Level, LevelFilter, Metadata};
extern crate rand;
extern crate toml;

//use std::thread;
//use std::sync::mpsc;

mod presenter;
use presenter::GLPresenter;

mod view;
use view::GLView;

mod world;
use world::ConcreteWorld;

mod database;
use database::FileDatabase;

mod entities;

//mod world;
//use world::World;

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

	/*// Channels to communicate between view and presenter
	let (tx_v2p, rx_v2p) = mpsc::channel();
	let (tx_p2v, rx_p2v) = mpsc::channel();

	// Channels to communicate between presenter and world
	let (tx_p2w, rx_p2w) = mpsc::channel();
	let (tx_w2p, rx_w2p) = mpsc::channel();

	// View thread
	let view_handle = thread::spawn(move || {
		GLView::new((tx_v2p, rx_p2v)).event_loop();
	});

	// Presenter thread
	let presenter_handle = thread::spawn(move || {
		GLPresenter::new((tx_p2v, rx_v2p), (tx_p2w, rx_w2p)).event_loop();
	});

	// World thread
	let world_handle = thread::spawn(move || {
		World::new((tx_w2p, rx_p2w)).event_loop();
	});

	view_handle.join().unwrap();
	presenter_handle.join().unwrap();
	world_handle.join().unwrap();*/
	let database = FileDatabase::new("worlds/tellene/config.toml");
	let world = match ConcreteWorld::new(Box::new(database)) {
		Ok(world) => world,
		Err(err) => {
			error!("Main: Error loading world: {:?}", err);
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
	warn!("Main is dying");
}
