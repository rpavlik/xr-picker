# XR Runtime Picker for OpenXR

<!--
Copyright 2023, Collabora, Ltd.
SPDX-License-Identifier: CC-BY-4.0
-->

[![REUSE status](https://api.reuse.software/badge/github.com/rpavlik/xr-picker)](https://api.reuse.software/info/github.com/rpavlik/xr-picker)

This is a cross-platform tool to allow you to easily change your active OpenXR
runtime. (It also serves as a bit of a testbed for Rust GUI techniques, though I
use it "in production".)

Features include:

- Finding available runtimes
  - On Windows using the AvailableRuntimes registry key and a few hard-coded
    extras
  - On Linux by listing the files in the config directories
- Parsing runtime manifests for the runtime name, as well as adding names to
  select exceptions via heuristics.
- Working with additional runtimes manually added (by browsing to or
  drag-and-dropping a manifest) - useful for runtime developers.
- Remembering these extra runtimes between sessions.
- Identifying the active runtime (or runtimes in the case of Windows, 32 and 64
  bit).
- Setting the active runtime(s)
  - On Windows by setting the registry value/values
  - On Linux by setting a per-user symlink to the manifest.

## Development and Contribution

Contributions are gladly accepted. CI enforces
[REUSE](https://reuse.software/), [cargo-deny][], and simple build tests on
Linux and Windows. It is a bit hard to test even the core library because it
works with registry keys and the file system, and I have not yet investigated
how to mock these cleanly in Rust for automated testing.

[cargo-deny]: https://embarkstudios.github.io/cargo-deny/

## License

Licensed under either of the
[Apache License, Version 2.0](LICENSES/Apache-2.0.txt) or the
[MIT license](LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

This software conforms to the [REUSE specification](https://reuse.software).
