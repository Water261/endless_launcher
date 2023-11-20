use std::{fs::File, fmt::Display};
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Result, bail};
use directories::ProjectDirs;
use serde::Deserialize;
use tracing::info;

use crate::APP_IDENTIFIER;

#[derive(Clone, Debug)]
pub enum ManifestError {
	/// Could not retrieve a valid home path
	ProjectDirectory
}

impl Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			ManifestError::ProjectDirectory => writeln!(f, "Could not retrieve a valid home path")
		}
    }
}

pub fn get_manifest() -> Result<Manifest> {
	info!("Retrieving the projet directory");
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
	version: u32,
	source_types: SourceTypesVersion,
	manifest: ManifestVersion,
}

impl Manifest {
	//* This is set up for future-proofing
	#[allow(unreachable_patterns)]
	pub fn check_version(&self) -> bool {
		match self.version {
			1 => {
				let source_types_correct = match self.source_types {
					SourceTypesVersion::Version1(_) => true,
					_ => false,
				};

				let manifest_correct = match self.manifest {
					ManifestVersion::Version1(_) => true,
					_ => false
				};

				return source_types_correct && manifest_correct
			},
			_ => todo!("Version number is not recognised")
		}
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ManifestVersion {
	Version1(Vec<ManifestVersion1>)
}

#[derive(Clone, Debug, Deserialize)]
pub struct ManifestVersion1 {
	name: String,
	latest_version: String,
	versions: Vec<String>,
	source: String,
	source_type: String
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum SourceTypesVersion {
	Version1(Vec<SourceTypesVersion1>)
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceTypesVersion1 {
	name: String,
	archive_path: String,
	archive_type: String,
}
