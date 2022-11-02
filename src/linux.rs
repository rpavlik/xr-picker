// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    Error, OPENXR_MAJOR_VERSION,
};
use std::path::{Path, PathBuf};

pub struct LinuxRuntime {
    base: BaseRuntime,
}
impl PlatformRuntime for LinuxRuntime {
    fn is_active(&self) -> bool {
        todo!()
    }

    fn make_active(&self) {
        todo!()
    }
}

pub struct LinuxPlatform;

// fn normalize_path(p: Path) ->  Result<PathBuf, io::Error> {
//     let metadata = p.symlink_metadata()?;
//     if metadata.is_symlink() {
//         metadata.
//     }
// }
impl Platform for LinuxPlatform {
    type PlatformRuntimeType = LinuxRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {
        let path_suffix = Path::new("openxr").join(OPENXR_MAJOR_VERSION.to_string());

        let xdg_dirs =
            xdg::BaseDirectories::new().map_err(|e| Error::EnumerationError(e.to_string()))?;
        let xdg_iter = xdg_dirs.list_config_files(&path_suffix).into_iter();
        let sysconfdir_iter= Path::new("/etc")
            .join(&path_suffix)
            .read_dir()
            .map(|dir_contents| {
                dir_contents.filter_map(|f| {
                    f.ok()
                        .filter(|entry| {
                            if let Ok(m) = entry.metadata() {
                                m.is_file() || m.is_symlink()
                            } else {
                                false
                            }
                        })
                        .map(|entry| entry.path())
                })
            })
            .ok();

        let manifest_files = sysconfdir_iter
            .into_iter()
            .chain(xdg_iter)
            .filter_map(|p| p.canonicalize().ok().map(|canonical| (p, canonical)))
            .for_each(|e| println!("readlink path {}", e.display()));
        Ok(vec![])
    }
}

pub fn make_platform() -> LinuxPlatform {
    LinuxPlatform
}
