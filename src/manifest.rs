use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Result, bail};
use directories::ProjectDirs;
use serde::Deserialize;
use tracing::info;
use thiserror::Error;

use crate::APP_IDENTIFIER;

#[derive(Clone, Debug, Error)]
pub enum ManifestError {
	#[error("Could not retrieve a valid home path")]
	ProjectDirectory
}

pub fn get_manifest() -> Result<Manifest> {
	info!("Retrieving the project directory");
    let project_dirs = match ProjectDirs::from(APP_IDENTIFIER[0], APP_IDENTIFIER[1], APP_IDENTIFIER[2]) {
		Some(dir) => dir,
		None => bail!(ManifestError::ProjectDirectory)
	};
    let cache_dir = project_dirs.cache_dir();

	info!("Reading and parsing the manifest file from cache");
    let mut manifest_file_path = PathBuf::from(cache_dir);
    manifest_file_path.push("manifest.json");

    let manifest_file = File::open(manifest_file_path)?;

	let manifest_reader = BufReader::new(manifest_file);

	let manifest: Manifest = serde_json::from_reader(manifest_reader)?;

	Ok(manifest)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Manifest {
	// This enforces what version the fields should be
	pub version: u32,
	pub source_types: Vec<SourceType>,
	pub manifest: Vec<ManifestPlugin>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ManifestPlugin {
	pub name: String,
	pub latest_version: String,
	pub versions: Vec<String>,
	pub source: String,
	pub source_type: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceType {
	pub name: String,
	pub archive_path: String,
	pub archive_type: String,
}
