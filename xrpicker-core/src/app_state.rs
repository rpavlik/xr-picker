// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use itertools::Itertools;

use crate::{platform::PlatformRuntime, Error, ManifestError, Platform};

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
        let (runtimes, nonfatal_errors) = platform.find_available_runtimes()?;
        let active_data = platform.get_active_data();
        Ok(Self {
            runtimes,
            nonfatal_errors,
            active_data,
        })
    }

    /// "refresh" existing state: we don't re-create if we can avoid it,
    /// to preserve the order of existing entries.
    pub fn refresh(self, platform: &T) -> Result<Self, Error> {
        let (new_runtimes, new_nonfatal_errors) = platform.find_available_runtimes()?;
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
