// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

// use crate::{
//     platform::{Platform, PlatformRuntime},
//     runtime::BaseRuntime,
//     Error, ACTIVE_RUNTIME_FILENAME, OPENXR, OPENXR_MAJOR_VERSION,
// };
// use std::{
//     fs,
//     path::{Path, PathBuf},
// };

// pub struct WindowsRuntime {
//     base: BaseRuntime,
// }

// impl Runtime {
//     fn new(path: &Path) -> Result<Self, Error> {
//         let base = BaseRuntime::new(path)?;
//         Ok(WindowsRuntime { base })
//     }
// }

// impl PlatformRuntime for WindowsRuntime {
//     fn is_active(&self) -> bool {
//         todo!()
//     }

//     fn make_active(&self) {
//         todo!()
//     }
// }

// pub struct WindowsPlatform {}

// impl WindowsPlatform {
//     fn new() -> Self {
//         Self
//     }
// }

// impl Platform for WindowsPlatform {
//     type PlatformRuntimeType = WindowsRuntime;
//     fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error> {
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
//         Ok(manifest_files)
//     }
// }

// pub fn make_platform() -> WindowsPlatform {
//     WindowsPlatform::new()
// }
