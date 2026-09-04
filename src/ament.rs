//! Defines the `Ament` struct storing a list of prefixes in which resources will be searched.
//!
//! This struct implements methods to search for resources and packages inside the stored prefixes.

use crate::{
    packages::get_package_share_directory,
    resources::{
        find_resource, get_resource, get_resources_prefix, has_resource,
        list_all_prefixes_of_resources,
    },
    search_paths::{get_search_paths, get_search_paths_from_var},
};
use std::{collections::HashMap, path::PathBuf};

/// Stores a list of prefixes in which resources will be searched.
#[derive(Clone, Debug, PartialEq)]
pub struct Ament {
    /// The list of prefixes in which resources will be searched.
    pub prefixes: Vec<PathBuf>,
}

impl Ament {
    /// Constructs a new `Ament` struct with prefixes read from the `AMENT_PREFIX_PATH` environment variable.
    ///
    /// # Errors
    ///
    /// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
    pub fn new() -> Result<Self, std::env::VarError> {
        Ok(Self {
            prefixes: get_search_paths()?,
        })
    }

    /// Constructs a new `Ament` struct with prefixes read from the given environment variable.
    ///
    /// # Errors
    ///
    /// A `std::env::VarError` is returned if the given environment variable is not set.
    pub fn from_var(env_var: &str) -> Result<Self, std::env::VarError> {
        Ok(Self {
            prefixes: get_search_paths_from_var(env_var)?,
        })
    }

    /// Returns a map containing packages name for key and the first prefix in which each package was found for value.
    pub fn get_packages_prefix(&self) -> HashMap<String, PathBuf> {
        self.get_resources_prefix("packages")
    }

    /// Returns a map containing packages name for key and the list of prefixes in which each package was found for value.
    pub fn get_packages_prefixes(&self) -> HashMap<String, Vec<PathBuf>> {
        self.get_resources_prefixes("packages")
    }

    /// Returns the `share` directory of the package or `None` is the package was not found.
    pub fn get_package_share_directory(&self, package_name: &str) -> Option<PathBuf> {
        get_package_share_directory(package_name, &self.prefixes)
    }

    /// Returns the prefix in which the given package has been found or `None` if the package has not been found.
    pub fn find_package(&self, package_name: &str) -> Option<PathBuf> {
        self.find_resource(package_name, "packages")
    }

    /// Returns true if the given package exists in the ament resource index, returns false otherwise.
    pub fn has_package(&self, package_name: &str) -> bool {
        self.has_resource(package_name, "packages")
    }

    /// Returns the content of the resource if the given resource exists in the ament resource index, or `None` if the resource was not found.
    pub fn get_resource(
        &self,
        resource_name: &str,
        resource_type: &str,
    ) -> Option<(std::io::Result<Vec<u8>>, PathBuf)> {
        get_resource(resource_name, resource_type, &self.prefixes)
    }

    /// Returns a map containing resources name for key and the first prefix in which each resource was found for value.
    pub fn get_resources_prefix(&self, resource_type: &str) -> HashMap<String, PathBuf> {
        get_resources_prefix(resource_type, &self.prefixes)
    }

    /// Returns a map containing resources name for key and the list of prefixes in which each resource was found for value.
    pub fn get_resources_prefixes(
        &self,
        resource_type: &str,
    ) -> HashMap<String, Vec<PathBuf>> {
        list_all_prefixes_of_resources(resource_type, &self.prefixes)
    }

    /// Returns the prefix in which the given resource was found or `None` if the resource was not found.
    pub fn find_resource(
        &self,
        resource_name: &str,
        resource_type: &str,
    ) -> Option<PathBuf> {
        find_resource(resource_name, resource_type, &self.prefixes)
    }

    /// Returns true if the given resource exists in the ament resource index, returns false otherwise.
    pub fn has_resource(
        &self,
        resource_name: &str,
        resource_type: &str,
    ) -> bool {
        has_resource(resource_name, resource_type, &self.prefixes)
    }
}
