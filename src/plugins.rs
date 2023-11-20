use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::info;
use thiserror::Error;

use crate::{manifest::Manifest, APP_IDENTIFIER};

#[derive(Clone, Debug, Error)]
pub enum PluginsError {
	#[error("Could not retrieve a valid home path")]
	ProjectDirectory,
	#[error("Version specified does not match rest of file")]
	VersionMismatch
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

#[derive(Clone, Debug, Deserialize)]
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

	pub fn install_plugin(&self, _plugin: &str) -> Result<()> {
		todo!("Implement plugin install functionality")
	}

	pub fn update_plugin(&self, _plugin: &str) -> Result<()> {
		todo!("Implement plugin update functionality")
	}

	pub fn uninstall_plugin(&self, _plugin: &str) -> Result<()> {
		todo!("Implement plugin uninstall functionality")
	}
}
