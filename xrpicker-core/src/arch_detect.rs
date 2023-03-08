// Copyright 2022-2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{runtime::BaseRuntime, Error, ManifestError};
use object::{self, read::Object};
use std::{fs, path::Path};

pub(crate) enum RuntimeBitness {
    /// Uses shared library search path to find the right binary per arch
    Universal,
    /// Points to a 32-bit runtime
    BitWidth32,
    /// Points to a 64-bit runtime
    BitWidth64,
}

pub(crate) fn get_runtime_bitness(manifest_path: &Path) -> Result<RuntimeBitness, ManifestError> {
    let runtime =
        BaseRuntime::new(manifest_path).map_err(|e| ManifestError(manifest_path.to_owned(), e))?;
    let library_path = runtime.resolve_library_path();
    if !library_path.is_absolute() {
        // If we can't resolve it, it must be universal
        return Ok(RuntimeBitness::Universal);
    }

    let make_err = || {
        ManifestError(
            library_path.clone(),
            Error::RuntimeBinaryLoadError(library_path.display().to_string()),
        )
    };

    let bin_data = fs::read(&library_path).map_err(|_| make_err())?;
    let obj_file = object::File::parse(&*bin_data).map_err(|_| make_err())?;
    if obj_file.is_64() {
        Ok(RuntimeBitness::BitWidth64)
    } else {
        Ok(RuntimeBitness::BitWidth32)
    }
}

pub(crate) trait PushUnique<T> {
    fn push_unique(&mut self, val: T);
}

impl<T> PushUnique<T> for Vec<T>
where
    T: Eq,
{
    fn push_unique(&mut self, val: T) {
        let contains = self.contains(&val);
        if !contains {
            self.push(val);
        }
    }
}
