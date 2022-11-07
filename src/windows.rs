// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    manifest::GenericManifest,
    platform::{Platform, PlatformRuntime},
    runtime::BaseRuntime,
    ActiveState, Error, ManifestError, OPENXR, OPENXR_MAJOR_VERSION,
};
use itertools::Itertools;
use special_folder::SpecialFolder;
use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    path::{Path, PathBuf},
};
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE, KEY_READ, KEY_WRITE},
    RegKey, RegValue,
};

#[derive(Debug, Clone)]
pub struct WindowsRuntime {
    base64: Option<BaseRuntime>,
    base32: Option<BaseRuntime>,
}

const WINMR_JSON_NAME: &str = "MixedRealityRuntime.json";

const AVAILABLE_RUNTIMES: &str = "AvailableRuntimes";
const ACTIVE_RUNTIME: &str = "ActiveRuntime";

#[cfg(target_pointer_width = "64")]
fn system_dir_64() -> Option<PathBuf> {
    SpecialFolder::System.get()
}

#[cfg(target_pointer_width = "32")]
fn system_dir_64() -> Option<PathBuf> {
    use iswow64::iswow64;

    if iswow64() {
        SpecialFolder::System.get().map(|p| {
            p.parent()
                .expect("system dir has a parent")
                .join("sysnative")
        })
    } else {
        None
    }
}

#[cfg(target_pointer_width = "64")]
fn system_dir_32() -> Option<PathBuf> {
    SpecialFolder::SystemX86.get()
}

#[cfg(target_pointer_width = "32")]
fn system_dir_32() -> Option<PathBuf> {
    SpecialFolder::System.get()
}

fn make_prefix_key() -> PathBuf {
    Path::new("Software")
        .join("Khronos")
        .join(OPENXR)
        .join(&OPENXR_MAJOR_VERSION.to_string())
}

#[cfg(target_pointer_width = "64")]
fn make_prefix_key_flags_64() -> Option<u32> {
    use winreg::enums::KEY_WOW64_64KEY;

    Some(KEY_WOW64_64KEY)
}

#[cfg(target_pointer_width = "64")]
fn make_prefix_key_flags_32() -> Option<u32> {
    use winreg::enums::KEY_WOW64_32KEY;

    Some(KEY_WOW64_32KEY)
}

#[cfg(target_pointer_width = "32")]
fn make_prefix_key_flags_64() -> Option<u32> {
    use iswow64::iswow64;
    use winreg::enums::KEY_WOW64_64KEY;
    if iswow64() {
        Some(KEY_WOW64_64KEY)
    } else {
        None
    }
}

#[cfg(target_pointer_width = "32")]
fn make_prefix_key_flags_32() -> Option<u32> {
    use winreg::enums::KEY_WOW64_32KEY;
    Some(KEY_WOW64_32KEY)
}

fn get_active_runtime_manifest_path(prefix: &Path, reg_flags: Option<u32>) -> Option<PathBuf> {
    let reg_flags = reg_flags?;
    let base = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(prefix, reg_flags | KEY_READ | KEY_QUERY_VALUE)
        .ok()?;
    let val: String = base.get_value(ACTIVE_RUNTIME).ok()?;
    Some(Path::new(&val).to_path_buf())
}

impl WindowsRuntime {
    fn new(path64: Option<&Path>, path32: Option<&Path>) -> Result<Self, Error> {
        let base64 = path64.map(BaseRuntime::new).transpose()?;
        let base32 = path32.map(BaseRuntime::new).transpose()?;
        Ok(WindowsRuntime { base64, base32 })
    }

    fn runtimes(&self) -> impl Iterator<Item = &BaseRuntime> {
        self.base64.iter().chain(self.base32.iter())
    }
}

impl PlatformRuntime for WindowsRuntime {
    fn make_active(&self) -> Result<(), Error> {
        fn try_set_active(
            reg_path: &Path,
            runtime: &Option<BaseRuntime>,
            flags: Option<u32>,
        ) -> Result<(), Error> {
            if let (Some(runtime), Some(flags)) = (runtime, flags) {
                let key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
                    reg_path,
                    flags | KEY_WRITE | KEY_READ | KEY_QUERY_VALUE,
                )?;
                key.set_value(ACTIVE_RUNTIME, &runtime.get_manifest_path().as_os_str())?;
            }
            Ok(())
        }
        let key = make_prefix_key();
        try_set_active(&key, &self.base64, make_prefix_key_flags_64())?;
        try_set_active(&key, &self.base32, make_prefix_key_flags_32())?;
        Ok(())
    }

    fn get_runtime_name(&self) -> String {
        self.runtimes()
            .map(|r| r.get_runtime_name())
            .next()
            .expect("At least one of the runtimes will be Some")
    }

    fn get_manifests(&self) -> Vec<&Path> {
        self.runtimes().map(|r| r.get_manifest_path()).collect()
    }

    fn get_libraries(&self) -> Vec<PathBuf> {
        self.runtimes().map(|r| r.resolve_library_path()).collect()
    }

    fn describe(&self) -> String {
        self.runtimes()
            .map(|r| r.describe_manifest(r.get_manifest_path()))
            .join("\n")
    }
}

/// Little helper for accumulating runtimes and coalescing their different bitnesses.
#[derive(Default)]
struct RuntimeCollection {
    runtimes: Vec<WindowsRuntime>,
    used_manifests: HashSet<PathBuf>,
}

impl RuntimeCollection {
    fn try_add(&mut self, path64: Option<&Path>, path32: Option<&Path>) -> Result<(), Error> {
        let mut has_path = false;
        if let Some(p) = path64 {
            has_path = true;
            if self.used_manifests.contains(p) {
                return Ok(());
            }
        }
        if let Some(p) = path32 {
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
        let runtime = WindowsRuntime::new(path64, path32)?;
        self.runtimes.push(runtime);
        if let Some(p) = path64 {
            self.used_manifests.insert(p.to_owned());
        }
        if let Some(p) = path32 {
            self.used_manifests.insert(p.to_owned());
        }
        Ok(())
    }

    fn try_add_varjo(&mut self) -> Result<(), ManifestError> {
        if !cfg!(target_pointer_width = "64") {
            return Ok(());
        }
        let path = SpecialFolder::ProgramFiles.get().map(|p| {
            p.join("Varjo")
                .join("varjo-openxr")
                .join("VarjoOpenXR.json")
        });
        let path = path.as_deref().filter(|&p| p.exists());
        if let Some(path) = path {
            self.try_add(Some(path), None)
                .map_err(|e| ManifestError(path.to_owned(), e))
        } else {
            Ok(())
        }
    }

    fn try_add_winmr(&mut self) -> Result<(), ManifestError> {
        // Manually add winmr because it will be some revisions of windows before they can put it in AvailableRuntimes
        let (winmr64, winmr32) = (
            system_dir_64().map(|d| d.join(WINMR_JSON_NAME)),
            system_dir_32().map(|d| d.join(WINMR_JSON_NAME)),
        );

        // Only use paths that exist
        let (winmr64, winmr32) = (
            winmr64.as_deref().filter(|&p| p.exists()),
            winmr32.as_deref().filter(|&p| p.exists()),
        );

        if winmr64.is_some() || winmr32.is_some() {
            self.try_add(winmr64, winmr32).map_err(|e| {
                ManifestError(winmr64.unwrap_or_else(|| winmr32.unwrap()).to_owned(), e)
            })
        } else {
            Ok(())
        }
    }
}

impl From<RuntimeCollection> for Vec<WindowsRuntime> {
    fn from(val: RuntimeCollection) -> Self {
        val.runtimes
    }
}

pub struct WindowsActiveRuntimeData {
    active_64: Option<PathBuf>,
    active_32: Option<PathBuf>,
}

fn check_active(active_runtime_manifest: &Option<PathBuf>, runtime: &Option<BaseRuntime>) -> bool {
    match (active_runtime_manifest.as_deref(), runtime) {
        (Some(active_manifest), Some(r)) => r.get_manifest_path() == active_manifest,
        _ => false,
    }
}

impl WindowsActiveRuntimeData {
    fn new() -> Self {
        let reg_prefix = make_prefix_key();
        let active_64 = get_active_runtime_manifest_path(&reg_prefix, make_prefix_key_flags_64());
        let active_32 = get_active_runtime_manifest_path(&reg_prefix, make_prefix_key_flags_32());
        Self {
            active_64,
            active_32,
        }
    }

    fn check_runtime(&self, runtime: &WindowsRuntime) -> ActiveState {
        let active_64 = check_active(&self.active_64, &runtime.base64);
        let active_32 = check_active(&self.active_32, &runtime.base32);

        ActiveState::from_active_64_and_32(active_64, active_32)
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

fn enumerate_reg_runtimes(base_key: &Path, reg_flags: u32) -> Vec<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey_with_flags(
        base_key.to_str().unwrap(),
        reg_flags | KEY_READ | KEY_QUERY_VALUE,
    )
    .map(|avail| {
        let manifest_files = avail.enum_values().filter_map(|x| {
            let x = x.ok()?;
            maybe_runtime(&avail, x)
        });
        manifest_files.collect()
    })
    .unwrap_or_default()
}

/// Returns any non-fatal errors
fn manually_add_runtimes(collection: &mut RuntimeCollection) -> Vec<ManifestError> {
    let mut nonfatal_errors = vec![];

    if let Err(e) = collection.try_add_varjo() {
        nonfatal_errors.push(e);
    }
    if let Err(e) = collection.try_add_winmr() {
        nonfatal_errors.push(e);
    }
    nonfatal_errors
}

impl Platform for WindowsPlatform {
    type PlatformRuntimeType = WindowsRuntime;
    fn find_available_runtimes(
        &self,
    ) -> Result<(Vec<Self::PlatformRuntimeType>, Vec<ManifestError>), Error> {
        let mut collection = RuntimeCollection::default();

        let mut nonfatal_errors = vec![];

        let avail_runtimes_key_path = make_prefix_key().join(AVAILABLE_RUNTIMES);

        let manifests64 = match make_prefix_key_flags_64() {
            Some(flags) => enumerate_reg_runtimes(&avail_runtimes_key_path, flags),
            None => Default::default(),
        };

        let manifests32 = match make_prefix_key_flags_32() {
            Some(flags) => enumerate_reg_runtimes(&avail_runtimes_key_path, flags),
            None => Default::default(),
        };

        let manifest_32_by_parent_dir: HashMap<&Path, &Path, RandomState> = HashMap::from_iter(
            manifests32
                .iter()
                .filter_map(|p| p.parent().map(|parent| (parent, p.as_path()))),
        );

        // Handle all 64-bit runtimes, matching with a 32-bit one if applicable
        for path in manifests64.iter() {
            let parent = path.parent().expect("every file has a parent");
            let counterpart_32 = manifest_32_by_parent_dir.get(parent);
            if let Err(e) = collection.try_add(Some(path), counterpart_32.map(|p| p.as_ref())) {
                eprintln!(
                    "Error creating runtime object for runtime with manifest {}: {}",
                    path.display(),
                    e
                );
                nonfatal_errors.push(ManifestError(path.to_owned(), e));
            }
        }
        // Handle remaining 32-bit ones
        for path in manifests32.iter() {
            // we don't care about errors right now
            if let Err(e) = collection.try_add(None, Some(path)) {
                eprintln!(
                    "Error creating runtime object for runtime with manifest {}: {}",
                    path.display(),
                    e
                );
                nonfatal_errors.push(ManifestError(path.to_owned(), e));
            }
        }

        // Finally, try adding ones we might not see otherwise
        nonfatal_errors.extend(manually_add_runtimes(&mut collection));

        Ok((collection.into(), nonfatal_errors))
    }

    type PlatformActiveData = WindowsActiveRuntimeData;

    fn get_active_runtime_manifests(&self) -> Vec<PathBuf> {
        let data = WindowsActiveRuntimeData::new();
        // OK to move out of data because we just created it for this purpose
        data.active_64
            .into_iter()
            .chain(data.active_32.into_iter())
            .collect()
    }

    fn get_active_data(&self) -> Self::PlatformActiveData {
        WindowsActiveRuntimeData::new()
    }

    fn get_runtime_active_state(
        &self,
        runtime: &Self::PlatformRuntimeType,
        active_data: &Self::PlatformActiveData,
    ) -> ActiveState {
        active_data.check_runtime(runtime)
    }
}

pub fn make_platform() -> WindowsPlatform {
    WindowsPlatform::new()
}
