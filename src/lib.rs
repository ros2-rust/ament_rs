//! This crate is a client for ament which is a system for cataloging and referencing resources distributed by software packages used by ROS2.
//!
//! # Naming conventions
//!
//! Functions with a name ending with `_from` take an additionnal `prefixes` argument.
//! These functions also have a variant named without the `_from` ending which use the prefixes retrieved from the `AMENT_PREFIX_PATH` environment variable.
//!
//! Functions starting with `list_` return an iterator instead of a collection.
//!
//! # Examples
//!
//! ```
//! use ament_rs::*;
//! println!("{:#?}", get_packages_with_prefixes());
//! ```
//!
//! This snippet will print a list of packages with the prefixes they were found in, depending of the value of the `AMENT_PREFIX_PATH` environment variable on your system.
//!
//! ```none
//! {
//!     "ros_core": [
//!         "/opt/ros/crystal",
//!         "/opt/ros/bouncy",
//!     ],
//!     "console_bridge_vendor": [
//!         "/opt/ros/dashing",
//!         "/opt/ros/crystal",
//!     ],
//!     "ament_cmake_export_interfaces": [
//!         "/opt/ros/dashing",
//!         "/opt/ros/crystal",
//!         "/opt/ros/bouncy",
//!     ],
//! }
//! ```
//!

/// This constant defines the name of the environment variable containing the list of ament resource index prefixes, which is `AMENT_PREFIX_PATH`
pub const AMENT_PREFIX_PATH_ENV_VAR: &str = "AMENT_PREFIX_PATH";

pub mod packages;
pub mod resources;
pub mod search_paths;

pub use packages::{
    find_package, get_package_prefix, get_package_share_directory, get_packages_with_prefixes,
    has_package,
};
pub use resources::{
    find_resource, get_resource, get_resources, has_resource, list_all_prefixes_of_resources,
    list_prefix_of_resources,
};
pub use search_paths::get_search_paths;
