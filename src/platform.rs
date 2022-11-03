// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{ActiveState, Error};

pub(crate) trait Runtime {
    fn get_human_readable_name(&self) -> String;
}

pub trait PlatformRuntime {
    fn get_active_state(&self) -> ActiveState;
    fn make_active(&self) -> Result<(), Error>;
}

pub trait Platform {
    type PlatformRuntimeType: PlatformRuntime;
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error>;
}
