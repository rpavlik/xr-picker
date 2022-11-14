// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io;

#[cfg(windows)]
use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)]
    {
        let mut res = WindowsResource::new();
        res.set_icon("assets/icon/icon48.ico");
        // .set_manifest_file("assets/manifest.xml")
        
        res.compile()?;
    }
    Ok(())
}
