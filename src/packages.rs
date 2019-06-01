use crate::get_search_paths;
use crate::resources::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_packages_with_prefixes_from(
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> HashMap<String, Vec<String>> {
    list_all_prefixes_of_resources("packages", prefixes)
}

pub fn get_package_prefix_from(
    package_name: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<String> {
    list_all_prefixes_of_resource(package_name, "packages", prefixes).nth(0)
}

pub fn get_package_share_directory_from(
    package_name: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<PathBuf> {
    Some(
        [
            &get_package_prefix_from(package_name.as_ref(), prefixes)?,
            "share",
            package_name.as_ref(),
        ]
        .iter()
        .collect(),
    )
}

pub fn get_packages_with_prefixes() -> Result<HashMap<String, Vec<String>>, std::env::VarError> {
    Ok(get_packages_with_prefixes_from(get_search_paths()?))
}

pub fn get_package_prefix(
    package_name: impl AsRef<str>,
) -> Result<Option<String>, std::env::VarError> {
    Ok(get_package_prefix_from(package_name, get_search_paths()?))
}

pub fn get_package_share_directory(
    package_name: impl AsRef<str>,
) -> Result<Option<PathBuf>, std::env::VarError> {
    Ok(get_package_share_directory_from(
        package_name,
        get_search_paths()?,
    ))
}
