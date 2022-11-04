// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::Path;

use serde::Deserialize;

pub(crate) trait GenericManifest {
    /// Get the library path as stored in the manifest
    fn library_path(&self) -> &str;

    /// Check the file format version
    fn is_file_format_version_ok(&self) -> bool;

    /// Does the library path use the system shared library search path?
    fn uses_search_path(&self) -> bool {
        !self.library_path().contains('/') && !self.library_path().contains('\\')
    }

    /// Should the library be searched for relative to the manifest?
    fn library_relative_to_manifest(&self) -> bool {
        let path = self.library_path();
        !self.uses_search_path()
            && !path.starts_with('/')
            && !path.starts_with('\\')
            && path.chars().nth(1) != Some(':')
    }

    /// Describe this manifest by using the manifest path and library path
    fn describe_manifest(&self, manifest_path: &Path) -> String {
        let manifest = manifest_path.display();
        if self.uses_search_path() {
            format!(
                "{} -> {} in the dynamic library search path",
                manifest,
                self.library_path()
            )
        } else if self.library_relative_to_manifest() {
            format!(
                "{} -> {} relative to the manifest",
                manifest,
                self.library_path()
            )
        } else {
            format!("{} -> {}", manifest, self.library_path())
        }
    }
}

/// Non-top-level objects in a runtime manifest
pub(crate) mod json_subobjects {
    use serde::Deserialize;

    /// The optional table of function symbol renaming in a runtime manifest
    #[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
    pub(crate) struct RuntimeFunctions {
        #[serde(rename = "xrNegotiateLoaderRuntimeInterface")]
        pub(crate) xr_negotiate_loader_runtime_interface: Option<String>,
    }

    /// The main object in a runtime manifest
    #[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
    pub(crate) struct Runtime {
        pub(crate) library_path: String,
        pub(crate) name: Option<String>,
        pub(crate) functions: Option<RuntimeFunctions>,
    }
}

/// Top level structure corresponding to a runtime manifest
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct RuntimeManifest {
    file_format_version: String,
    pub(crate) runtime: json_subobjects::Runtime,
}

impl GenericManifest for RuntimeManifest {
    fn library_path(&self) -> &str {
        &self.runtime.library_path
    }
    fn is_file_format_version_ok(&self) -> bool {
        self.file_format_version == "1.0.0"
    }
}
