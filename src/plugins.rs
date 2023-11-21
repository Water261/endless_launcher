use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::info;
use thiserror::Error;

use crate::{manifest::{Manifest, ManifestPlugin}, APP_IDENTIFIER};

#[derive(Clone, Debug, Error)]
pub enum PluginsError {
	#[error("Could not retrieve a valid home path")]
	ProjectDirectory,
	#[error("Version specified does not match rest of file")]
	VersionMismatch,
	#[error("Could not find the specified plugin")]
	PluginMissing
}

pub fn get_installed_plugins() -> Result<Box<PluginsFile>> {
	info!("Retrieving the project directory");
	let projects_dir = match ProjectDirs::from(APP_IDENTIFIER[0], APP_IDENTIFIER[1], APP_IDENTIFIER[2]) {
		Some(dir) => dir,
		None => bail!(PluginsError::ProjectDirectory)
	};
	let data_dir = projects_dir.data_dir();

	info!("Reading and parsing the plugins file");
	let mut plugins_path = PathBuf::from(data_dir);
	plugins_path.push("plugins.json");

	let plugins_file = File::open(plugins_path)?;
	let plugins_reader = BufReader::new(plugins_file);
	let plugins_file: PluginsFile = serde_json::from_reader(plugins_reader)?;

	Ok(Box::new(plugins_file))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PluginsFile {
	pub version: u32,
	pub plugins: Vec<Plugin>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plugin {
	pub name: String,
	pub version: String
}

pub struct PluginManager {
	plugins_file: RwLock<Box<PluginsFile>>,
	manifest_file: Box<Manifest>
}

impl PluginManager {
	pub fn new(manifest_file: Box<Manifest>, plugins_file: RwLock<Box<PluginsFile>>) -> Self {
		Self {
			manifest_file,
			plugins_file
		}
	}

	pub fn restore_plugins(&self) -> Result<()> {
		todo!("Implement plugin restore functionality")
	}

	// TODO: Actually download the plugin
	pub fn install_plugin(&self, plugin: &str) -> Result<()> {
		let plugin = match self.get_manifest_plugin(plugin) {
			Some(plugin) => plugin,
			None => bail!(PluginsError::PluginMissing)
		};

		let installed_plugin = Plugin {
			name: plugin.name,
			version: plugin.latest_version
		};

		let mut write_guard = match self.plugins_file.try_write() {
			Ok(guard) => guard,
			Err(e) => bail!("{:?}", e)
		};

		write_guard.plugins.push(installed_plugin);
		
		Ok(())
	}

	pub fn update_plugin(&self, _plugin: &str) -> Result<()> {
		todo!("Implement plugin update functionality")
	}

	pub fn uninstall_plugin(&self, plugin: &str) -> Result<()> {
		let mut write_guard = match self.plugins_file.try_write() {
			Ok(guard) => guard,
			Err(e) => bail!("{:?}", e)
		};

		let index = match write_guard.plugins.iter().position(|p| &(p.name) == plugin) {
			Some(index) => index,
			None => bail!(PluginsError::PluginMissing)
		};

		write_guard.plugins.remove(index);

		Ok(())
	}

	pub fn get_manifest_plugin(&self, plugin: &str) -> Option<ManifestPlugin> {
		let index_opt = self.manifest_file.manifest.iter().position(|p| &(p.name) == plugin);

		match index_opt {
			Some(index) => self.manifest_file.manifest.get(index).cloned(),
			None => None
		}
	}

	pub fn get_installed_plugin(&self, plugin: &str) -> Result<Option<Plugin>> {
		let read_guard = match self.plugins_file.try_read() {
			Ok(guard) => guard,
			Err(e) => bail!("{:?}", e)
		};

		let index_opt = read_guard.plugins.iter().position(|p| &(p.name) == plugin);

		Ok(match index_opt {
			Some(index) => read_guard.plugins.get(index).cloned(),
			None => None
		})
	}
}
