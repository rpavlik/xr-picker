// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::{Path, PathBuf};

use crate::RuntimeManifest;

pub(crate) struct BaseRuntime {
    manifest_path: PathBuf,
    manifest: RuntimeManifest,
}
