// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use xrpicker::{Error, make_platform, Platform};

fn main() {
    println!("Hello, world!");
    make_platform().find_available_runtimes();
}
