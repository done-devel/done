use serde::{Deserialize, Serialize};

use super::appearance::ColorScheme;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Preferences {
	pub color_scheme: ColorScheme,
	pub extended: bool,
}

impl Preferences {
	pub async fn new() -> Self {
		Self {
			color_scheme: ColorScheme::Default,
			extended: true,
		}
	}
}
