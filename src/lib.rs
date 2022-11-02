// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

pub const OPENXR_MAJOR_VERSION: i32 = 1;

pub const ACTIVE_RUNTIME_FILENAME: &str = "active_runtime.json";
/// Directory used in constructing paths
pub const OPENXR: &str = "openxr";

pub(crate) mod manifest;
pub mod platform;
pub(crate) mod runtime;

use std::io;

pub(crate) use manifest::RuntimeManifest;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failure while attempting to enumerate available runtimes: {0}")]
    EnumerationError(String),

    #[error("IO error during manifest read")]
    ManifestReadError(#[from] io::Error),

    #[error("JSON parsing error")]
    JsonParseError(#[from] serde_json::Error)
}

mod linux;
pub use linux::make_platform;
pub use platform::Platform;
