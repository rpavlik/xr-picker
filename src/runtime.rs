// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{path::{Path, PathBuf}, fs};

use crate::{Error, RuntimeManifest};

pub(crate) struct BaseRuntime {
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
}
