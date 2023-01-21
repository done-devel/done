use anyhow::Result;
use directories::ProjectDirs;
use libset::format::FileFormat;
use libset::project::Project;
use proto_rust::provider::provider_client::ProviderClient;
use proto_rust::provider::{Empty, List};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt};
use tonic::transport::Channel;

pub const PLUGINS_URL: &str = "https://raw.githubusercontent.com/done-devs/done/main/dev.edfloreshz.Done.Plugins.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Plugin {
	#[serde(rename = "pluginId")]
	pub id: String,
	#[serde(rename = "pluginName")]
	pub name: String,
	#[serde(rename = "pluginDescription")]
	pub description: String,
	#[serde(rename = "pluginIcon")]
	pub icon: String,
	#[serde(rename = "pluginPort")]
	pub port: u32,
	#[serde(rename = "pluginVersion")]
	pub version: String,
	#[serde(rename = "pluginDownloadUrl")]
	pub download_url: String,
	#[serde(rename = "pluginProcessName")]
	pub process_name: String,
	#[serde(skip)]
	pub lists: Vec<List>,
}

impl Plugin {
	pub async fn fetch_plugins() -> Result<Vec<Plugin>> {
		let response = relm4::spawn(async move {
			reqwest::get(PLUGINS_URL)
				.await
				.unwrap()
				.text()
				.await
				.unwrap()
		})
		.await?;
		let plugins: Vec<Plugin> = serde_json::from_str(&response)?;
		Ok(plugins)
	}

	pub fn get_plugins() -> Result<Vec<Plugin>> {
		let plugins = Project::open("dev", "edfloreshz", "done")?
			.get_file_as::<Vec<Plugin>>(
				"dev.edfloreshz.Done.Plugins",
				FileFormat::JSON,
			)?;
		Ok(plugins)
	}

	pub fn get_by_id(id: &str) -> Result<Plugin> {
		let plugins = Self::get_plugins()?;
		let plugin = plugins
			.into_iter()
			.find(|plugin| plugin.id == id)
			.ok_or_else(|| anyhow::anyhow!("Plugin not found."))?;
		Ok(plugin)
	}

	pub fn start(&self) -> Result<u32, std::io::Error> {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();
		let mut command = Command::new(format!("./{}", &self.process_name));
		command.current_dir(project.data_dir().join("bin"));
		match command.spawn() {
			Ok(child) => Ok(child.id()),
			Err(err) => Err(err),
		}
	}

	pub fn stop(&self) {
		let processes = System::new_all();
		if let Some(process) =
			processes.processes_by_exact_name(&self.process_name).next()
		{
			if process.kill() {
				tracing::info!("Process was killed.");
			} else {
				tracing::error!("Failed to kill process.");
			}
		} else {
			tracing::info!("Process is not running.");
		};
	}

	pub fn is_running(&self) -> bool {
		let processes = System::new_all();
		let is_running = processes
			.processes_by_exact_name(&self.process_name)
			.next()
			.is_some();
		is_running
	}

	pub async fn install(&self) -> Result<PathBuf> {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();
		let path = download_file(
			&self.download_url,
			project.data_dir().join("bin").join(&self.process_name),
		)
		.await?;
		Ok(path)
	}

	pub fn is_installed(&self) -> bool {
		let project = ProjectDirs::from("dev", "edfloreshz", "done").unwrap();
		project
			.data_dir()
			.join("bin")
			.join(&self.process_name)
			.exists()
	}

	pub async fn connect(&self) -> Result<ProviderClient<Channel>> {
		let port = self.port;
		loop {
			match relm4::spawn(async move {
				ProviderClient::connect(format!("http://[::1]:{}", port))
					.await
					.unwrap()
			})
			.await
			{
				Ok(client) => return Ok(client),
				Err(err) => tracing::error!("Failed to connect to plugin: {err}"),
			}
		}
	}

	pub async fn lists(self) -> Result<Vec<String>> {
		let response = relm4::spawn(async move {
			let mut connector = self.connect().await.unwrap();
			connector
				.read_all_list_ids(Empty {})
				.await
				.unwrap()
				.into_inner()
		})
		.await?;
		Ok(response.lists)
	}
}

// Download a file from a URL and save it to a file
async fn download_file(url: &str, path: PathBuf) -> Result<PathBuf> {
	let client = Client::new();
	let url = url.to_string();
	let response =
		relm4::spawn(async move { client.get(url).send().await.unwrap() }).await?;
	let status = response.status();
	if status == 200 {
		let bytes =
			relm4::spawn(async move { response.bytes().await.unwrap() }).await?;
		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		let mut file = std::fs::File::create(path.clone()).unwrap();
		file
			.set_permissions(std::fs::Permissions::from_mode(0o755))
			.unwrap();
		file.write_all(&bytes).unwrap();
		Ok(path)
	} else {
		Err(anyhow::anyhow!("This service is unavailable."))
	}
}
