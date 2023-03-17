// Copyright 2022-2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(windows)]
use winres::WindowsResource;

use std::io;

fn main() -> io::Result<()> {
    #[cfg(windows)]
    {
        let mut res = WindowsResource::new();
        res.set_icon("../assets/icon/icon48.ico")
            .set_icon_with_id("../assets/icon/icon32.ico", "2")
            .set_manifest_file("../assets/manifest.xml")
            .set_language(0x0409) // US English
            .compile()?;
    }
    Ok(())
}
