// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use xdg::{BaseDirectories, BaseDirectoriesError};

use crate::{
    manifest::GenericManifest,
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    ActiveState, Error, ACTIVE_RUNTIME_FILENAME, OPENXR, OPENXR_MAJOR_VERSION,
};
use std::{
    fs,
    iter::once,
    os::unix,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const ETC: &str = "/etc";

fn make_path_suffix() -> PathBuf {
    Path::new(OPENXR).join(OPENXR_MAJOR_VERSION.to_string())
}

fn make_sysconfdir(suffix: &Path) -> PathBuf {
    Path::new(ETC).join(suffix)
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinuxRuntime {
    base: BaseRuntime,
    orig_path: PathBuf,
}

impl LinuxRuntime {
    fn new(orig_path: &Path, canonical_path: &Path) -> Result<Self, Error> {
        let base = BaseRuntime::new(canonical_path)?;
        Ok(LinuxRuntime {
            base,
            orig_path: orig_path.to_owned(),
        })
    }
}

impl PlatformRuntime for LinuxRuntime {
    fn make_active(&self) -> Result<(), Error> {
        fn convert_err(e: BaseDirectoriesError) -> Error {
            Error::SetActiveError(e.to_string())
        }
        let dirs = BaseDirectories::new().map_err(convert_err)?;
        let suffix = make_path_suffix();
        let path = dirs.place_config_file(suffix.join(ACTIVE_RUNTIME_FILENAME))?;

        // First move the old file out of the way, if any.
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let move_target =
            dirs.place_config_file(suffix.join(format!("old_active_runtime{}.json", timestamp)))?;

        match fs::rename(&path, &move_target) {
            Ok(_) => {
                // Only keep our renamed file if it wasn't a symlink
                if let Ok(m) = move_target.symlink_metadata() {
                    if m.is_symlink() && fs::remove_file(&move_target).is_err() {
                        // that's ok
                        eprintln!(
                            "Got an error trying to remove an apparently-symlink {}",
                            move_target.display()
                        )
                    }
                }
            }
            Err(e) => {
                // ignore and hope it meant there was just nothing to move
                eprintln!(
                    "Got an error trying to rename {} to {}: {}",
                    path.display(),
                    move_target.display(),
                    e
                );
            }
        }
        unix::fs::symlink(self.base.get_manifest_path(), &path)?;
        Ok(())
    }

    fn get_runtime_name(&self) -> String {
        self.base.get_runtime_name()
    }

    fn get_manifests(&self) -> Vec<&Path> {
        vec![self.base.get_manifest_path()]
    }

    fn get_libraries(&self) -> Vec<PathBuf> {
        let path = self.base.resolve_library_path();
        vec![path]
    }

    fn describe(&self) -> String {
        let description = self.base.describe_manifest(self.base.get_manifest_path());
        if self.orig_path != self.base.get_manifest_path() {
            format!("{} -> {}", self.orig_path.display(), description)
        } else {
            description
        }
    }
}

pub struct LinuxPlatform {
    path_suffix: PathBuf,
}

impl LinuxPlatform {
    fn new() -> Self {
        let path_suffix = make_path_suffix();
        Self { path_suffix }
    }
}

fn find_potential_manifests_xdg(suffix: &Path) -> impl Iterator<Item = PathBuf> {
    let suffix = suffix.to_owned();
    BaseDirectories::new()
        .ok()
        .into_iter()
        .flat_map(move |xdg_dirs| xdg_dirs.list_config_files(&suffix).into_iter())
}

fn find_potential_manifests_sysconfdir(suffix: &Path) -> impl Iterator<Item = PathBuf> {
    make_sysconfdir(suffix)
        .read_dir()
        .into_iter()
        .flat_map(|dir_contents| {
            dir_contents
                .into_iter()
                .filter_map(|r| r.ok())
                .filter_map(|entry| {
                    if let Ok(m) = entry.metadata() {
                        if m.is_file() || m.is_symlink() {
                            return Some(entry.path());
                        }
                    }
                    None
                })
        })
}

pub struct LinuxActiveRuntimeData(Option<PathBuf>);

impl LinuxActiveRuntimeData {
    fn new() -> Self {
        let suffix = (&make_path_suffix()).join(ACTIVE_RUNTIME_FILENAME);
        let xdg_iter = BaseDirectories::new()
            .ok()
            .into_iter()
            .flat_map(|d| d.find_config_files(&suffix));

        let path = xdg_iter
            .chain(once(make_sysconfdir(&suffix)))
            .filter(|p| {
                p.metadata()
                    .map(|m| m.is_file() || m.is_symlink())
                    .ok()
                    .unwrap_or_default()
            })
            .filter_map(|p| p.canonicalize().ok())
            .next();

        LinuxActiveRuntimeData(path)
    }

    fn check_runtime(&self, runtime: &LinuxRuntime) -> ActiveState {
        let is_active = self
            .0
            .as_ref()
            .map(|active_path| runtime.base.get_manifest_path() == active_path)
            .unwrap_or_default();

        match is_active {
            true => ActiveState::ActiveIndependentRuntime,
            false => ActiveState::NotActive,
        }
    }
}

impl Platform for LinuxPlatform {
    type PlatformRuntimeType = LinuxRuntime;
    type PlatformActiveData = LinuxActiveRuntimeData;

    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {
        let manifest_files = find_potential_manifests_xdg(&self.path_suffix)
            .chain(find_potential_manifests_sysconfdir(&self.path_suffix))
            .filter_map(|p| p.canonicalize().ok().map(|canonical| (p, canonical)))
            .filter_map(
                |(orig, canonical)| match LinuxRuntime::new(&orig, &canonical) {
                    Ok(r) => Some(r),
                    Err(e) => {
                        eprintln!(
                            "Error when trying to load {} -> {}: {}",
                            orig.display(),
                            canonical.display(),
                            e
                        );
                        None
                    }
                },
            )
            // .unique_by(|r| r.base.get_manifest_path().to_owned())
            // .sorted_by_cached_key(|r| r.base.resolve_library_path())
            .collect();
        Ok(manifest_files)
    }

    fn get_active_runtime_manifests(&self) -> Vec<PathBuf> {
        LinuxActiveRuntimeData::new().0.into_iter().collect()
    }

    fn get_active_data(&self) -> Self::PlatformActiveData {
        LinuxActiveRuntimeData::new()
    }

    fn get_runtime_active_state(
        &self,
        runtime: &Self::PlatformRuntimeType,
        active_data: &Self::PlatformActiveData,
    ) -> ActiveState {
        active_data.check_runtime(runtime)
    }
}

pub fn make_platform() -> LinuxPlatform {
    LinuxPlatform::new()
}
