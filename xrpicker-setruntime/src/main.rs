// Copyright 2024, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![forbid(unsafe_code)]

use pico_args::Arguments;
use xrpicker::platform::PlatformRuntime;

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

fn parse_runtime() -> Result<impl PlatformRuntime, anyhow::Error> {
    let mut args = Arguments::from_env();

    let orig_path = args.value_from_os_str("--orig-path", parse_path)?;
    let canonical_path = args.opt_value_from_os_str("--canonical", parse_path)?;

    let canonical_path = canonical_path
        .or_else(|| orig_path.canonicalize().ok())
        .ok_or_else(|| pico_args::Error::ArgumentParsingFailed {
            cause: "Failed to canonicalize the orig path".to_owned(),
        })?;
    let runtime = xrpicker::linux::LinuxRuntime::new(&orig_path, &canonical_path)?;
    Ok(runtime)
}

fn main() -> Result<(), pico_args::Error> {
    println!("Hello, world!");
    Ok(())
}
