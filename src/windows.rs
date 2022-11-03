// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    ActiveState, Error, OPENXR, OPENXR_MAJOR_VERSION,
};
use special_folder::SpecialFolder;
use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    path::{Path, PathBuf},
};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey, RegValue};

#[derive(Debug, Clone)]
pub struct WindowsRuntime {
    base: Option<BaseRuntime>,
    base_narrow: Option<BaseRuntime>,
}

const WINMR_JSON_NAME: &str = "MixedRealityRuntime.json";

const AVAILABLE_RUNTIMES: &str = "AvailableRuntimes";
const ACTIVE_RUNTIME: &str = "ActiveRuntime";
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

fn make_prefix_key_native() -> PathBuf {
    Path::new("Software")
        .join("Khronos")
        .join(OPENXR)
        .join(&OPENXR_MAJOR_VERSION.to_string())
}

#[cfg(target_arch = "x86_64")]
fn make_prefix_key_narrow() -> Option<PathBuf> {
    Some(
        Path::new("Software")
            .join("WOW6432Node")
            .join("Khronos")
            .join(OPENXR)
            .join(&OPENXR_MAJOR_VERSION.to_string()),
    )
}

#[cfg(not(target_arch = "x86_64"))]
fn make_prefix_key_narrow() -> Option<PathBuf> {
    None
}

fn get_active_runtime_manifest_path(prefix: PathBuf) -> Option<PathBuf> {
    let base = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(&prefix)
        .ok()?;
    let val: String = base.get_value(ACTIVE_RUNTIME).ok()?;
    Some(Path::new(&val).to_path_buf())
}

impl WindowsRuntime {
    fn new(path: Option<&Path>, narrow_path: Option<&Path>) -> Result<Self, Error> {
        let base = path.map(|p| BaseRuntime::new(p)).transpose()?;
        let base_narrow = narrow_path.map(|p| BaseRuntime::new(p)).transpose()?;
        Ok(WindowsRuntime { base, base_narrow })
    }
}

fn check_active(active_runtime_manifest: Option<&Path>, runtime: Option<&BaseRuntime>) -> bool {
    match active_runtime_manifest {
        Some(active_manifest) => match runtime {
            Some(r) => r.get_manifest_path() == active_manifest,
            None => false,
        },
        None => false,
    }
}
impl PlatformRuntime for WindowsRuntime {
    fn get_active_state(&self) -> ActiveState {
        let native_active = get_active_runtime_manifest_path(make_prefix_key_native());
        let narrow_active = make_prefix_key_narrow()
            .into_iter()
            .filter_map(|p| get_active_runtime_manifest_path(p))
            .next();

        let is_native_active = check_active(
            native_active.as_ref().map(|p| p.as_path()),
            self.base.as_ref(),
        );

        let is_narrow_active = check_active(
            narrow_active.as_ref().map(|p| p.as_path()),
            self.base_narrow.as_ref(),
        );
        ActiveState::from_native_and_narrow_activity(is_native_active, is_narrow_active)
    }

    fn make_active(&self) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Default)]
struct RuntimeCollection {
    runtimes: Vec<WindowsRuntime>,
    used_manifests: HashSet<PathBuf>,
}

impl RuntimeCollection {
    fn try_add(&mut self, path: Option<&Path>, path_narrow: Option<&Path>) -> Result<(), Error> {
        let mut has_path = false;
        if let Some(p) = path {
            has_path = true;
            if self.used_manifests.contains(p) {
                return Ok(());
            }
        }
        if let Some(p) = path_narrow {
            has_path = true;
            if self.used_manifests.contains(p) {
                return Ok(());
            }
        }
        if !has_path {
            return Err(Error::EnumerationError(
                "Tried to add a runtime with no manifest paths!".to_string(),
            ));
        }
        let runtime = WindowsRuntime::new(path, path_narrow)?;
        self.runtimes.push(runtime);
        if let Some(p) = path {
            self.used_manifests.insert(p.to_owned());
        }
        if let Some(p) = path_narrow {
            self.used_manifests.insert(p.to_owned());
        }
        Ok(())
    }
}

impl Into<Vec<WindowsRuntime>> for RuntimeCollection {
    fn into(self) -> Vec<WindowsRuntime> {
        self.runtimes
    }
}

pub struct WindowsPlatform;

impl WindowsPlatform {
    fn new() -> Self {
        Self
    }
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
        .open_subkey(base_key.to_str().unwrap())
        .map_err(|e| Error::EnumerationError(format!("Registry read error: {}", e)))?;

    let manifest_files = avail.enum_values().filter_map(|x| {
        let x = x.ok()?;
        maybe_runtime(&avail, x)
    });
    Ok(manifest_files.collect())
}
impl Platform for WindowsPlatform {
    type PlatformRuntimeType = WindowsRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {
        let mut collection = RuntimeCollection::default();

        // Manually add winmr because it will be some revisions of windows before they can put it in AvailableRuntimes
        if collection
            .try_add(
                system_dir_native()
                    .map(|d| d.join(WINMR_JSON_NAME))
                    .as_deref(),
                system_dir_narrow()
                    .map(|d| d.join(WINMR_JSON_NAME))
                    .as_deref(),
            )
            .is_err()
        {
            // this is fine if it's not there
        }

        let native = enumerate_reg_runtimes(&make_prefix_key_native().join(AVAILABLE_RUNTIMES))?;
        let narrow = make_prefix_key_narrow()
            .map(|k| enumerate_reg_runtimes(&k))
            .transpose()?
            .unwrap_or_default();

        let narrow_by_parent_dir: HashMap<&Path, &Path, RandomState> = HashMap::from_iter(
            narrow
                .iter()
                .filter_map(|p| p.parent().map(|parent| (parent, p.as_path()))),
        );

        // Handle all native-width runtimes, matching with a narrow one if applicable
        for path in native.iter() {
            let parent = path.parent().expect("every file has a parent");
            let narrow_ver = narrow_by_parent_dir.get(parent);
            if let Err(e) = collection.try_add(Some(path), narrow_ver.map(|p| p.as_ref())) {
                eprintln!(
                    "Error creating runtime object for runtime with manifest {}: {}",
                    path.display(),
                    e
                );
            }
        }
        Ok(collection.into())
    }
}

pub fn make_platform() -> WindowsPlatform {
    WindowsPlatform::new()
}
