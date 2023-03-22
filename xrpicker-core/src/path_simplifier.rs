// Copyright 2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    borrow::Cow,
    iter,
    path::{Component, Path, PathBuf},
};

/// Replace the literal home directory path with ~ in a Path and possibly similar things in the future.
pub(crate) struct PathSimplifier {
    home_dir: Option<PathBuf>,
}

impl PathSimplifier {
    pub(crate) fn new() -> Self {
        Self {
            home_dir: dirs::home_dir(),
        }
    }

    pub(crate) fn simplify<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        if let Some(home_dir) = &self.home_dir {
            if path.starts_with(home_dir) {
                let simplified: PathBuf = iter::once(Component::Normal("~".as_ref()))
                    .chain(path.components().skip(home_dir.components().count()))
                    .collect();
                return Cow::Owned(simplified);
            }
        }
        Cow::Borrowed(path)
    }
}
