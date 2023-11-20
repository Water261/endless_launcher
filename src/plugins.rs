use std::{sync::RwLock, rc::Rc, fmt::Display, path::PathBuf, fs::File, io::BufReader};

use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{manifest::Manifest, APP_IDENTIFIER};

#[derive(Clone, Debug)]
pub enum PluginsError {
	/// Could not retrieve a valid home path
	ProjectDirectory,
}

impl Display for PluginsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			PluginsError::ProjectDirectory => writeln!(f, "Could not retrieve a valid home path")
		}
    }
}

pub fn get_installed_plugins() -> Result<Box<PluginsFile>> {
	info!("Retrieving the projet directory");
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
pub struct Plugin {
	name: String,
	version: String
}

pub struct PluginManager {
	plugins_file: RwLock<Box<PluginsFile>>,
	manifest_file: Rc<Box<Manifest>>
}

impl PluginManager {
	pub fn new(manifest_file: Rc<Box<Manifest>>, plugins_file: RwLock<Box<PluginsFile>>) -> Self {
		Self {
			manifest_file,
			plugins_file
		}
	}

	pub fn restore_plugins(&self) -> Result<()> {
		todo!("Implement plugin restore functionality")
	}

	pub fn install_plugin(&self, plugin: &str) -> Result<()> {
		todo!("Implement plugin install functionality")
	}

	pub fn update_plugin(&self, plugin: &str) -> Result<()> {
		todo!("Implement plugin update functionality")
	}

	pub fn uninstall_plugin(&self, plugin: &str) -> Result<()> {
		todo!("Implement plugin uninstall functionality")
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct PluginsFile {
	version: u32,
	plugins: PluginsFileVersion
}

impl PluginsFile {
	//* This is set up for future-proofing
	#[allow(unreachable_patterns)]
	pub fn check_version(&self) -> bool {
		match self.version {
			1 => {
				let plugins_version = match self.plugins {
					PluginsFileVersion::Version1(_) => true,
					_ => false,
				};

				return plugins_version;
			}
			_ => todo!("Version number not recognised"),
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PluginsFileVersion {
	Version1(Vec<Plugin>)
}
