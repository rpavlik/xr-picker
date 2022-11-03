// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{manifest::GenericManifest, Error, RuntimeManifest};

#[derive(Debug, Clone)]
pub struct BaseRuntime {
    manifest_path: PathBuf,
    manifest: RuntimeManifest,
}

impl BaseRuntime {
    pub(crate) fn new(manifest_path: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(manifest_path)?;
        let manifest: RuntimeManifest = serde_json::from_str(&contents)?;
        if !manifest.is_file_format_version_ok() {
            return Err(Error::ManifestVersionMismatch);
        }
        Ok(BaseRuntime {
            manifest_path: manifest_path.to_owned(),
            manifest,
        })
    }

    pub(crate) fn get_manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    /// Get a name for the runtime, preferably the self-declared one.
    ///
    /// Not promised to be unique, though!
    pub(crate) fn get_runtime_name(&self) -> String {
        // Prefer the runtime's advertised name if it has one
        if let Some(s) = &self.manifest.runtime.name {
            return s.clone();
        }

        // Heuristics go here, for manifests that lack the name
        if self.manifest.library_path().contains("MixedRealityRuntime") {
            return "Windows Mixed Reality".to_owned();
        }
        if self.manifest.library_path().contains("monado") {
            return "Monado".to_owned();
        }

        // Fallback to manifest path or library path
        self.manifest_path
            .to_str()
            .unwrap_or_else(|| self.manifest.library_path())
            .to_owned()
    }

    pub(crate) fn resolve_library_path(&self) -> PathBuf {
        let notcanon = self
            .manifest_path
            .parent()
            .expect("files always have parents")
            .join(self.manifest.library_path());
        notcanon.canonicalize().unwrap_or(notcanon)
    }
}

impl GenericManifest for BaseRuntime {
    fn library_path(&self) -> &str {
        self.manifest.library_path()
    }

    fn is_file_format_version_ok(&self) -> bool {
        self.manifest.is_file_format_version_ok()
    }
}
