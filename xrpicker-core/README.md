# xrpicker crate

<!--
Copyright 2023, Collabora, Ltd.
SPDX-License-Identifier: CC-BY-4.0
-->

[![Crates.io](https://img.shields.io/crates/v/xrpicker)](https://crates.io/crates/xrpicker)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![REUSE status](https://api.reuse.software/badge/github.com/rpavlik/xr-picker)](https://api.reuse.software/info/github.com/rpavlik/xr-picker)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](../CODE_OF_CONDUCT.md)

This crate provides the core functionality for enumerating OpenXR runtimes,
identifying the active runtime, and changing the active runtime, on Windows and
Linux. It contains features that assist in implementing a GUI frontend but does
not rely on or infer any particular GUI.

It includes a very minimal (for now) CLI tool that can only list the active
runtime and available runtimes. This will probably be upgraded eventually to be
able to set the active runtime, in part so that the Windows GUI build can invoke
it as administrator instead of having to run the whole GUI as administrator.

See the
[main XR Picker README](https://github.com/rpavlik/xr-picker/blob/main/README.md)
for more information.

## License

Licensed under either of the
[Apache License, Version 2.0](LICENSES/Apache-2.0.txt) or the
[MIT license](LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

This software conforms to the [REUSE specification](https://reuse.software).
