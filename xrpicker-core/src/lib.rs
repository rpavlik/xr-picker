// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

pub const OPENXR_MAJOR_VERSION: i32 = 1;

pub const ACTIVE_RUNTIME_FILENAME: &str = "active_runtime.json";
/// Directory used in constructing paths
pub const OPENXR: &str = "openxr";

mod app_state;
pub(crate) mod manifest;
pub mod platform;
pub(crate) mod runtime;

pub use app_state::AppState;

use std::{fmt::Display, io, path::PathBuf};

pub(crate) use manifest::RuntimeManifest;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failure while attempting to enumerate available runtimes: {0}")]
    EnumerationError(String),

    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("JSON parsing error")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Manifest file format version mismatch")]
    ManifestVersionMismatch,

    #[error("Error when trying to set active runtime: {0}")]
    SetActiveError(String),
}

#[derive(Debug)]
pub struct ManifestError(pub PathBuf, pub Error);

#[derive(Debug, Clone, Copy)]
pub enum ActiveState {
    NotActive,
    ActiveIndependentRuntime,
    Active64,
    Active32,
    Active64and32,
}

impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveState::NotActive => write!(f, ""),
            ActiveState::ActiveIndependentRuntime => write!(f, "Active"),
            ActiveState::Active64 => write!(f, "Active - 64-bit only"),
            ActiveState::Active32 => write!(f, "Active - 32-bit only"),
            ActiveState::Active64and32 => write!(f, "Active"),
        }
    }
}

impl ActiveState {
    /// Turn a pair of booleans (one for 64 bit, one for 32) into an active state enum.
    #[cfg(windows)]
    pub(crate) fn from_active_64_and_32(active_64: bool, active_32: bool) -> Self {
        match (active_64, active_32) {
            (true, true) => Self::Active64and32,
            (true, false) => Self::Active64,
            (false, true) => Self::Active32,
            (false, false) => Self::NotActive,
        }
    }

    /// Is this state at least somewhat inactive, such that we should offer to make it active?
    pub fn should_provide_make_active_button(&self) -> bool {
        match self {
            ActiveState::NotActive => true,
            ActiveState::ActiveIndependentRuntime => false,
            ActiveState::Active64 => true,
            ActiveState::Active32 => true,
            ActiveState::Active64and32 => false,
        }
    }
}

#[cfg(unix)]
mod linux;
#[cfg(unix)]
pub use linux::make_platform;
#[cfg(unix)]
pub type ConcretePlatform = linux::LinuxPlatform;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::make_platform;
#[cfg(windows)]
pub type ConcretePlatform = windows::WindowsPlatform;

pub use platform::Platform;
