// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use xrpicker::{make_platform, platform::PlatformRuntime, Platform};

fn main() {
    println!("Hello, world!");
    let platform = make_platform();
    let active_data = platform.get_active_data();
    for runtime in platform.find_available_runtimes().unwrap() {
        println!(
            "- {}: {:?} - {:?}",
            runtime.get_runtime_name(),
            platform.get_runtime_active_state(&runtime, &active_data),
            runtime
        );
    }

    println!("Active runtime manifest path(s)");

    for path in platform.get_active_runtime_manifests() {
        println!("- {}", path.display());
    }
}
