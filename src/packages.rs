//! Contains functions to retrieve packages registered as ament resources.

use crate::resources::*;
use std::path::PathBuf;

/// Returns the `share` directory of the package or `None` is the package was not found.
pub fn get_package_share_directory(
    package_name: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<PathBuf> {
    Some(
        [
            &get_resource_prefix(package_name.as_ref(), "packages", prefixes)?,
            "share",
            package_name.as_ref(),
        ]
        .iter()
        .collect(),
    )
}