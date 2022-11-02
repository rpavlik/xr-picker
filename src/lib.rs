// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

pub const OPENXR_MAJOR_VERSION: i32 = 1;

pub(crate) mod manifest;
pub mod platform;
pub(crate) mod runtime;


pub(crate) use manifest::RuntimeManifest;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failure while attempting to enumerate available runtimes: {0}")]
    EnumerationError(String),
}

mod linux;
pub use linux::make_platform;
pub use platform::Platform;
