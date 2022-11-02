// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

// use registry::{Hive, Security, iter::keys::KeyRef};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey, RegValue};

use crate::{
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    Error, ACTIVE_RUNTIME_FILENAME, OPENXR, OPENXR_MAJOR_VERSION,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct WindowsRuntime {
    base: BaseRuntime,
}

impl WindowsRuntime {
    fn new(path: &Path) -> Result<Self, Error> {
        let base = BaseRuntime::new(path)?;
        Ok(WindowsRuntime { base })
    }
}

impl PlatformRuntime for WindowsRuntime {
    fn is_active(&self) -> bool {
        todo!()
    }

    fn make_active(&self) {
        todo!()
    }
}

impl Into<BaseRuntime> for WindowsRuntime {
    fn into(self) -> BaseRuntime {
        self.base
    }
}

pub struct WindowsPlatform;

impl WindowsPlatform {
    fn new() -> Self {
        Self
    }
}

fn make_available_runtimes_key() -> String {
    r"Software\Khronos Group\".to_owned()
        + &OPENXR_MAJOR_VERSION.to_string()
        + r"\AvailableRuntimes"
}

// fn try_reg_key(k: KeyRef) -> Option<PathBuf> {
//     let k = k.open(Security::Read).ok()?;
//     k.
// }

fn maybe_runtime(regkey: &RegKey, kv: (String, RegValue)) -> Option<PathBuf> {
    let (val_name, _) = kv;
    let v: u32 = regkey.get_value(&val_name).ok()?;
    if v == 0 {
        return Some(Path::new(&val_name).to_owned());
    }
    None
}

impl Platform for WindowsPlatform {
    type PlatformRuntimeType = WindowsRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let avail = hklm
            .open_subkey(format!(
                "Software\\Khronos\\OpenXR\\{}\\AvailableRuntimes",
                OPENXR_MAJOR_VERSION
            ))
            .map_err(|e| Error::EnumerationError(format!("Registry read error: {}", e)))?;

        let manifest_files = avail
            .enum_values()
            .filter_map(|x| {
                let x = x.ok()?;
                maybe_runtime(&avail, x)
            })
            .filter_map(|p| match WindowsRuntime::new(&p) {
                Ok(r) => Some(r),
                Err(e) => {
                    eprintln!("Error when trying to load {}: {}", p.display(), e);
                    None
                }
            })
            .collect();
        //         let regkey = Hive::LocalMachine
        //             .open(make_available_runtimes_key(), Security::Read)
        //             .map_err(|e| Error::EnumerationError(format!("Registry read error: {}", e)))?;
        // regkey.keys().filter_map(|r| r.map(|k| k.))
        //         let manifest_files = find_potential_manifests_xdg(&self.path_suffix)
        //             .chain(find_potential_manifests_sysconfdir(&self.path_suffix))
        //             .filter_map(|p| p.canonicalize().ok().map(|canonical| (p, canonical)))
        //             .filter_map(
        //                 |(orig, canonical)| match WindowsRuntime::new(&orig, &canonical) {
        //                     Ok(r) => Some(r),
        //                     Err(e) => {
        //                         eprintln!(
        //                             "Error when trying to load {} -> {}: {}",
        //                             orig.display(),
        //                             canonical.display(),
        //                             e
        //                         );
        //                         None
        //                     }
        //                 },
        //             )
        //             .collect();
        Ok(manifest_files)
    }
}

pub fn make_platform() -> WindowsPlatform {
    WindowsPlatform::new()
}
