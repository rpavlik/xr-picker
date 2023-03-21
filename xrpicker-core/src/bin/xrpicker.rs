// Copyright 2022-2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::iter;

use xrpicker::{make_platform, platform::PlatformRuntime, Platform};

fn main() {
    let platform = make_platform();
    let active_data = platform.get_active_data();
    let (runtimes, nonfatal_errors) = platform
        .find_available_runtimes(Box::new(iter::empty()))
        .unwrap();
    println!("\nRuntimes:");
    for runtime in runtimes {
        println!(
            "- {}: {:?} - {:?}",
            runtime.get_runtime_name(),
            platform.get_runtime_active_state(&runtime, &active_data),
            runtime
        );
    }

    if !nonfatal_errors.is_empty() {
        println!("\nNon-fatal errors:");
        for e in nonfatal_errors {
            println!("- Manifest: {} - Error: {:?}", e.0.display(), e.1);
        }
    }

    println!("\nActive runtime manifest path(s):");

    for path in platform.get_active_runtime_manifests() {
        println!("- {}", path.display());
    }
}
