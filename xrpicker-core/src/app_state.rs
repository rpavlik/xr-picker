// Copyright 2022-2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{iter, path::PathBuf};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{platform::PlatformRuntime, Error, ManifestError, Platform};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersistentAppState {
    /// The extra paths provided by the user: the only thing we really want to serialize
    pub extra_paths: Vec<PathBuf>,
}

impl PersistentAppState {
    pub fn append_new_extra_paths(&mut self, new_extra_paths: Vec<PathBuf>) {
        if !new_extra_paths.is_empty() {
            let old_extra_paths = std::mem::take(&mut self.extra_paths);
            self.extra_paths.extend(
                old_extra_paths
                    .into_iter()
                    .chain(new_extra_paths.into_iter())
                    .unique(),
            );
        }
    }
}

trait IterateExtraPaths {
    fn iterate_extra_paths(&self) -> Box<dyn '_ + Iterator<Item = PathBuf>>;
}

impl IterateExtraPaths for PersistentAppState {
    fn iterate_extra_paths(&self) -> Box<dyn '_ + Iterator<Item = PathBuf>> {
        Box::new(self.extra_paths.iter().cloned())
    }
}
impl IterateExtraPaths for Option<&PersistentAppState> {
    fn iterate_extra_paths(&self) -> Box<dyn '_ + Iterator<Item = PathBuf>> {
        match self {
            Some(state) => Box::new(state.extra_paths.iter().cloned()),
            None => Box::new(iter::empty()),
        }
    }
}
impl IterateExtraPaths for Option<PersistentAppState> {
    fn iterate_extra_paths(&self) -> Box<dyn '_ + Iterator<Item = PathBuf>> {
        match self {
            Some(state) => Box::new(state.extra_paths.iter().cloned()),
            None => Box::new(iter::empty()),
        }
    }
}

/// Generic state data for the app in a "non-error" state
///
/// GUI code will likely implement new traits for this,
/// as well as wrap it in a struct (probably one that owns a Platform implementation too)
pub struct AppState<T: Platform> {
    pub runtimes: Vec<T::PlatformRuntimeType>,
    pub nonfatal_errors: Vec<ManifestError>,
    pub active_data: T::PlatformActiveData,
}

impl<T: Platform> AppState<T> {
    /// Try creating state from scratch
    pub fn new(platform: &T) -> Result<Self, Error> {
        let (runtimes, nonfatal_errors) =
            platform.find_available_runtimes(Box::new(iter::empty()))?;
        let active_data = platform.get_active_data();
        Ok(Self {
            runtimes,
            nonfatal_errors,
            active_data,
        })
    }

    pub fn new_with_persistent_state(
        platform: &T,
        persistent_state: &PersistentAppState,
    ) -> Result<Self, Error> {
        let (runtimes, nonfatal_errors) =
            platform.find_available_runtimes(persistent_state.iterate_extra_paths())?;
        let active_data = platform.get_active_data();
        Ok(Self {
            runtimes,
            nonfatal_errors,
            active_data,
        })
    }

    /// "refresh" existing state: we don't re-create if we can avoid it,
    /// to preserve the order of existing entries.
    pub fn refresh(
        self,
        platform: &T,
        persistent_state: Option<&PersistentAppState>,
    ) -> Result<Self, Error> {
        let (new_runtimes, new_nonfatal_errors) =
            platform.find_available_runtimes(persistent_state.iterate_extra_paths())?;

        let active_data = platform.get_active_data();

        // start with existing runtimes
        let runtimes = self
            .runtimes
            .into_iter()
            // chain on the new ones
            .chain(new_runtimes.into_iter())
            // only keep the unique ones, preferring the earlier ones
            .unique_by(|r| {
                // compare by the list of manifests used
                r.get_manifests()
                    .into_iter()
                    .map(|p| p.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(Self {
            runtimes,
            nonfatal_errors: new_nonfatal_errors,
            active_data,
        })
    }
}
