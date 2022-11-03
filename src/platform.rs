// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use crate::{ActiveState, Error};

pub(crate) trait Runtime {}

pub trait PlatformRuntime {
    /// Is this runtime marked as active?
    ///
    /// Some platforms might have separate 32-bit and 64-bit active runtime settings,
    /// which makes this more complex than a bool.
    fn get_active_state(&self) -> ActiveState;

    /// Attempt to make this runtime active.
    fn make_active(&self) -> Result<(), Error>;

    /// Get a name for the runtime, preferably the self-declared one.
    ///
    /// Not promised to be unique, though!
    fn get_runtime_name(&self) -> String;
}

pub trait Platform {
    type PlatformRuntimeType: PlatformRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error>;

    fn get_active_runtime_manifests(&self) -> Vec<PathBuf>;
}
