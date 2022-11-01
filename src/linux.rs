// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    Error,
};

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

impl Platform for LinuxPlatform {
    type PlatformRuntimeType = LinuxRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("openxr/1")
            .map_err(|e| Error::EnumerationError(e.to_string()))?;
        xdg_dirs
            .list_config_files(".")
            .into_iter()
            .for_each(|e| println!("path {}", e.to_str().unwrap_or("INVALID UNICODE")));
        Ok(vec![])
    }
}

pub fn make_platform() -> LinuxPlatform {
    LinuxPlatform
}
