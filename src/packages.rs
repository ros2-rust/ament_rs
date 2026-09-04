//! Contains functions to retrieve packages registered as ament resources.

use crate::resources::*;
use std::path::{Path, PathBuf};

/// Returns the `share` directory of the package or `None` is the package was not found.
pub fn get_package_share_directory(
    package_name: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> Option<PathBuf> {
    Some(
        get_resource_prefix(package_name, "packages", prefixes)?
            .join("share")
            .join(package_name),
    )
}
