// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Error;

pub(crate) trait Runtime {
    fn get_name(&self) -> &str;
}

pub trait PlatformRuntime {
    fn is_active(&self) -> bool;
    fn make_active(&self);
}

pub trait Platform {
    type PlatformRuntimeType: PlatformRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error>;
}
