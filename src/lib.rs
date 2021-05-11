//! This crate is a client for ament which is a system for cataloging and referencing resources distributed by software packages used by ROS2.
//!
//! # Examples
//!
//! ```
//! use ament_rs::*;
//! # fn main() -> Result<(), std::env::VarError> {
//! # std::env::set_var("AMENT_PREFIX_PATH", "");
//! println!("{:#?}", Ament::new()?.get_packages_prefixes());
//! # Ok(())
//! # }
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

pub mod ament;
pub mod packages;
pub mod resources;
pub mod search_paths;

pub use ament::Ament;
