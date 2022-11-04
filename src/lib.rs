// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

pub const OPENXR_MAJOR_VERSION: i32 = 1;

pub const ACTIVE_RUNTIME_FILENAME: &str = "active_runtime.json";
/// Directory used in constructing paths
pub const OPENXR: &str = "openxr";

pub(crate) mod manifest;
pub mod platform;
pub(crate) mod runtime;

use std::{fmt::Display, io};

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

#[derive(Debug, Clone, Copy)]
pub enum ActiveState {
    NotActive,
    ActiveIndependentRuntime,
    ActiveNativeRuntime,
    ActiveNarrowRuntime,
    ActiveNativeAndNarrowRuntime,
}

#[cfg(target_pointer_width = "64")]
impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveState::NotActive => write!(f, ""),
            ActiveState::ActiveIndependentRuntime => write!(f, "Active"),
            ActiveState::ActiveNativeRuntime => write!(f, "Active - 64-bit only"),
            ActiveState::ActiveNarrowRuntime => write!(f, "Active - 32-bit only"),
            ActiveState::ActiveNativeAndNarrowRuntime => write!(f, "Active"),
        }
    }
}
#[cfg(target_pointer_width = "32")]
impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveState::NotActive => write!(f, ""),
            ActiveState::ActiveIndependentRuntime => write!(f, "Active"),
            ActiveState::ActiveNativeRuntime => write!(f, "Active"),
            ActiveState::ActiveNarrowRuntime => panic!("Should be unreachable"),
            ActiveState::ActiveNativeAndNarrowRuntime => panic!("Should be unreachable"),
        }
    }
}

impl ActiveState {
    /// Turn a pair of booleans (one for native, one for narrow) into an active state enum.
    #[cfg(windows)]
    pub(crate) fn from_native_and_narrow_activity(
        is_native_active: bool,
        is_narrow_active: bool,
    ) -> Self {
        match (is_native_active, is_narrow_active) {
            (true, true) => Self::ActiveNativeAndNarrowRuntime,
            (true, false) => Self::ActiveNativeRuntime,
            (false, true) => Self::ActiveNarrowRuntime,
            (false, false) => Self::NotActive,
        }
    }

    /// Is this state at least somewhat inactive, such that we should offer to make it active?
    pub fn should_provide_make_active_button(&self) -> bool {
        match self {
            ActiveState::NotActive => true,
            ActiveState::ActiveIndependentRuntime => false,
            ActiveState::ActiveNativeRuntime => true,
            ActiveState::ActiveNarrowRuntime => true,
            ActiveState::ActiveNativeAndNarrowRuntime => false,
        }
    }
}

#[cfg(unix)]
mod linux;
#[cfg(unix)]
pub use linux::make_platform;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::make_platform;

pub use platform::Platform;
