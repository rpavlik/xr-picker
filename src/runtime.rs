// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{Error, RuntimeManifest};

#[derive(Debug, Clone)]
pub struct BaseRuntime {
    manifest_path: PathBuf,
    manifest: RuntimeManifest,
}

impl BaseRuntime {
    pub(crate) fn new(manifest_path: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(manifest_path)?;
        let manifest: RuntimeManifest = serde_json::from_str(&contents)?;
        Ok(BaseRuntime {
            manifest_path: manifest_path.to_owned(),
            manifest,
        })
    }

    pub(crate) fn get_manifest_path(&self) -> &Path {
        &self.manifest_path
    }
    pub(crate) fn get_manifest_data(&self) -> &RuntimeManifest {
        &self.manifest
    }
}
