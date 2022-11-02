// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use xrpicker::{make_platform, Platform};

fn main() {
    println!("Hello, world!");
    for runtime in make_platform()
        .find_available_runtimes()
        .unwrap()
        .into_iter()
    {
        eprintln!("ASDF {:?}", runtime);
    }
}
