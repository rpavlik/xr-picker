// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    Error, ACTIVE_RUNTIME_FILENAME, OPENXR, OPENXR_MAJOR_VERSION,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
    fn is_active(&self) -> bool {
        todo!()
    }

    fn make_active(&self) {
        todo!()
    }
}

pub struct LinuxPlatform {
    path_suffix: PathBuf,
}

const ETC: &str = "/etc";
fn make_path_suffix() -> PathBuf {
    Path::new(OPENXR).join(OPENXR_MAJOR_VERSION.to_string())
}

fn make_sysconfdir(suffix: &Path) -> PathBuf {
    Path::new(ETC).join(suffix)
}

fn find_xdg_active_runtime(suffix: &Path) -> Option<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::new().ok()?;
    let xdg_iter = xdg_dirs
        .list_config_files_once(suffix.join(ACTIVE_RUNTIME_FILENAME))
        .into_iter();
    xdg_iter
        .filter(|p| {
            p.metadata()
                .map(|m| m.is_file() || m.is_symlink())
                .ok()
                .unwrap_or(false)
        })
        .next()
}

impl LinuxPlatform {
    fn new() -> Self {
        let path_suffix = make_path_suffix();
        // let sys_conf_dir = make_sysconfdir(&path_suffix);

        // let xdg_dirs = xdg::BaseDirectories::new().ok();
        // let xdg_iter = xdg_dirs.list_config_files(&self.path_suffix).into_iter();
        Self { path_suffix }
    }
}

fn find_potential_manifests_xdg(suffix: &Path) -> impl Iterator<Item = PathBuf> {
    let suffix = suffix.to_owned();
    xdg::BaseDirectories::new()
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
impl Platform for LinuxPlatform {
    type PlatformRuntimeType = LinuxRuntime;
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
            .collect();
        Ok(manifest_files)
    }
}

pub fn make_platform() -> LinuxPlatform {
    LinuxPlatform::new()
}
