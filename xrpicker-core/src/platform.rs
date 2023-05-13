// Copyright 2022-2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::{Path, PathBuf};

use crate::{ActiveState, Error, ManifestError};

/// Trait for platform-specific interaction with a runtime.
pub trait PlatformRuntime {
    /// Attempt to make this runtime active.
    fn make_active(&self) -> Result<(), Error>;

    /// Get a name for the runtime, preferably the self-declared one.
    ///
    /// Not promised to be unique, though!
    fn get_runtime_name(&self) -> String;

    fn get_manifests(&self) -> Vec<&Path>;
    fn get_libraries(&self) -> Vec<PathBuf>;

    /// Describe this specific instance of a runtime: usually using the manifest(s) and library
    fn describe(&self) -> String;
}

/// Trait abstracting over the underlying system/platform type.
/// For any given build, only a single implementation of this trait
/// will be available. Having this as a trait is probably overkill
/// but keeps the interface constrained?
pub trait Platform {
    /// Platform-specific type for a runtime, must implement `PlatformType`
    type PlatformRuntimeType: PlatformRuntime;

    /// Platform-specific data describing the currently active runtime(s).
    /// Meant to be opaque and just used in `get_runtime_active_state()`
    type PlatformActiveData;

    /// Enumerate all available runtimes we might be aware of.
    fn find_available_runtimes(
        &self,
        extra_paths: Box<dyn '_ + Iterator<Item = PathBuf>>,
    ) -> Result<(Vec<Self::PlatformRuntimeType>, Vec<ManifestError>), Error>;

    /// Get the paths of all active runtime manifests. (There may be one per architecture.)
    fn get_active_runtime_manifests(&self) -> Vec<PathBuf>;

    /// Get a snapshot of what the active runtime(s) is/are,
    /// to use when checking if a runtime we know about is active.
    /// Returns a relatively opaque type used to pass into `get_runtime_active_state()`
    fn get_active_data(&self) -> Self::PlatformActiveData;

    /// Is the given runtime marked as active?
    ///
    /// Some platforms might have separate 32-bit and 64-bit active runtime settings,
    /// which makes this more complex than a bool.
    fn get_runtime_active_state(
        &self,
        runtime: &Self::PlatformRuntimeType,
        active_data: &Self::PlatformActiveData,
    ) -> ActiveState;
}
