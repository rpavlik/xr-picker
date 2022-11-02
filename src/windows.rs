// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use special_folder::SpecialFolder;

use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey, RegValue};

use crate::{
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    Error, OPENXR_MAJOR_VERSION,
};
use std::{path::{Path, PathBuf}, collections::{HashMap, HashSet}};

#[derive(Debug, Clone)]
pub struct WindowsRuntime {
    base: Option<BaseRuntime>,
    base_narrow: Option<BaseRuntime>,
}

const WINMR_JSON_NAME: &str = "MixedRealityRuntime.json";

fn system_dir_native() -> Option<PathBuf> {
    SpecialFolder::System.get()
}

#[cfg(target_pointer_width = "64")]
fn system_dir_narrow() -> Option<PathBuf> {
    SpecialFolder::SystemX86.get()
}

#[cfg(target_pointer_width = "32")]
fn system_dir_narrow() -> Option<PathBuf> {
    None
}

fn winmr_native() -> Option<PathBuf> {
    system_dir_native().map(|d| d.join(WINMR_JSON_NAME))
}

struct BitPair<T>(T, T);

#[cfg(target_arch = "x86_64")]
fn system_dirs() -> BitPair<Option<PathBuf>> {
    BitPair(SpecialFolder::SystemX86.get(), SpecialFolder::System.get())
}
#[cfg(target_arch = "x86")]
fn system_dirs() -> BitPair<Option<PathBuf>> {
    BitPair(SpecialFolder::System.get(), None)
}

fn make_prefix_key_native() -> PathBuf {
    Path::new("Software")
        .join("Khronos")
        .join(&OPENXR_MAJOR_VERSION.to_string())
}

#[cfg(target_arch = "x86_64")]
fn make_prefix_key_narrow() -> Option<PathBuf> {
    Some(Path::new("Software")
        .join("WOW6432Node")
        .join("Khronos")
        .join(&OPENXR_MAJOR_VERSION.to_string()))
}
#[cfg(not(target_arch = "x86_64"))]
fn make_prefix_key_narrow() -> Option<PathBuf> {
    None
}

// fn make_available_runtimes_key() -> PathBuf {
//         .join("AvailableRuntimes")
// }

impl WindowsRuntime {
    fn new(path: Option<&Path>, narrow_path: Option<&Path>) -> Result<Self, Error> {
        let base = path.map(|p| BaseRuntime::new(p)).transpose()?;
        let base_narrow = narrow_path.map(|p| BaseRuntime::new(p)).transpose()?;
        Ok(WindowsRuntime { base, base_narrow })
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


pub struct WindowsPlatform;

impl WindowsPlatform {
    fn new() -> Self {
        Self
    }
}

const AVAILABLE_RUNTIMES: &str = "AvailableRuntimes";

fn make_available_runtimes_key() -> PathBuf {
    Path::new("Software")
        .join("Khronos")
        .join(&OPENXR_MAJOR_VERSION.to_string())
        .join("AvailableRuntimes")
}

fn maybe_runtime(regkey: &RegKey, kv: (String, RegValue)) -> Option<PathBuf> {
    let (val_name, _) = kv;
    let v: u32 = regkey.get_value(&val_name).ok()?;
    if v == 0 {
        return Some(Path::new(&val_name).to_owned());
    }
    None
}

fn enumerate_reg_runtimes(base_key: &Path) -> Result<Vec<PathBuf>, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let avail = hklm
        .open_subkey(base_key)
        .map_err(|e| Error::EnumerationError(format!("Registry read error: {}", e)))?;

    let manifest_files = avail.enum_values().filter_map(|x| {
        let x = x.ok()?;
        maybe_runtime(&avail, x)
    });
    Ok(manifest_files.collect())
}

fn make_winmr() -> (Option<PathBuf>, Option<PathBuf>) {
    system_dir_native().map(|d| d.join(WINMR_JSON_NAME)), system_dir_narrow().map(|d| d.join(WINMR_JSON_NAME))
}


impl Platform for WindowsPlatform {
    type PlatformRuntimeType = WindowsRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {

        let native = enumerate_reg_runtimes(&make_prefix_key_native().join(AVAILABLE_RUNTIMES))?;
        let narrow = make_prefix_key_narrow().map(|k| enumerate_reg_runtimes(&k)).transpose()?.unwrap_or_default();
        
        let narrow_by_parent_dir = HashMap::from_iter( narrow.iter().filter_map(|p| p.parent().map(|parent| (parent, p))));

        let mut used_narrow = HashSet::new();

        let mut result = vec![];
        for path in native.iter() {
            let parent = path.parent().expect("every file has a parent");
            let narrow_ver = narrow_by_parent_dir.get(parent);
            result.append(WindowsRuntime::new(Some(path), narrow_ver)?);
            if let Some(narrow_version) = narrow_by_parent_dir.get(parent) {
            }
        }
        let winmr = system_dir_native().map(|d| d.join(WINMR_JSON_NAME))

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let avail = hklm
            .open_subkey(make_available_runtimes_key())
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

        Ok(manifest_files)
    }
}

pub fn make_platform() -> WindowsPlatform {
    WindowsPlatform::new()
}
