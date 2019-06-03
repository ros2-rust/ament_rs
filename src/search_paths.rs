//! Contains functions to retrieve ament resource index prefixes.
//!
//! Prefixes are system paths separated by a colon.

use crate::AMENT_PREFIX_PATH_ENV_VAR;

/// Returns the list of prefixes defined in the `AMENT_PREFIX_PATH` environment variable.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn get_search_paths() -> Result<Vec<String>, std::env::VarError> {
    get_search_paths_from_var(AMENT_PREFIX_PATH_ENV_VAR)
}

/// Returns the list of prefixes defined in the given environment variable.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the given environment variable is not set.
pub fn get_search_paths_from_var(
    env_var: impl AsRef<str>,
) -> Result<Vec<String>, std::env::VarError> {
    Ok(get_search_paths_from(std::env::var(env_var.as_ref())?))
}

/// Returns the list of prefixes defined in the given string.
pub fn get_search_paths_from(prefixes_list: impl AsRef<str>) -> Vec<String> {
    prefixes_list
        .as_ref()
        .split(':')
        .map(str::to_string)
        .collect()
}
