#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::Result;
use application::localize::load_localization;
use diesel_migrations::embed_migrations;
use once_cell::sync::Lazy;
use relm4::adw::prelude::ApplicationExt;
use relm4::gtk::prelude::Cast;
use relm4::{adw, gtk, gtk::gio, RelmApp};

use crate::adw::Application;
use crate::application::resources::load_resources;
use widgets::app::AppModel;

use crate::application::application::DoneApplication;
use crate::application::utilities::{load_css, verify_data_integrity};

mod application;
mod schema;
mod widgets;
mod core;

embed_migrations!("migrations");

fn main() -> Result<()> {
	pretty_env_logger::init();
	load_localization();
	load_resources()?;
	let application = DoneApplication::new();
	application.connect_startup(|_| load_css());
	verify_data_integrity()?;
	let app: RelmApp<AppModel> =
		RelmApp::with_app(application.upcast::<Application>());
	app.run(None);
	Ok(())
}
