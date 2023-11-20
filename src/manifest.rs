use serde::Deserialize;

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
