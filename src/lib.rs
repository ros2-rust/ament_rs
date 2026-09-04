//! `ament_rs` provides utilities for querying the ament resource index used by ROS 2 packages.
//!
//! # Examples
//!
//! ```
//! # fn main() -> Result<(), std::env::VarError> {
//! # std::env::set_var("AMENT_PREFIX_PATH", "");
//! println!("{:#?}", ament_rs::packages()?);
//! # Ok(())
//! # }
//! ```
//!
//! This snippet will print a list of packages with the prefixes they were found in, depending of the value of the `AMENT_PREFIX_PATH` environment variable on your system.
//!
//! ```text
//! {
//!     "ros_core": [
//!         "/opt/ros/rolling",
//!     ],
//!     "rcl_interfaces": [
//!         "/your/workspace/install/rcl_interfaces",
//!         "/opt/ros/rolling",
//!     ],
//!     ...
//! }
//! ```
//!

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// This constant defines the name of the environment variable containing the list of ament resource index prefixes, which is `AMENT_PREFIX_PATH`
pub const AMENT_PREFIX_PATH_ENV_VAR: &str = "AMENT_PREFIX_PATH";

/// Returns the list of prefixes defined in the `AMENT_PREFIX_PATH` environment variable.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn prefixes() -> Result<Vec<PathBuf>, std::env::VarError> {
    prefixes_from_env_var(AMENT_PREFIX_PATH_ENV_VAR)
}

/// Returns the list of prefixes defined in the given environment variable.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the given environment variable is not set.
pub fn prefixes_from_env_var(env_var: &str) -> Result<Vec<PathBuf>, std::env::VarError> {
    Ok(std::env::split_paths(&std::env::var(env_var)?).collect())
}

/// Returns all resources of the given type found under the provided ament prefixes.
///
/// Each resource name is mapped to the list of prefixes that contain a matching
/// resource marker file.
pub fn resources_in(resource_type: &str, prefixes: &[PathBuf]) -> HashMap<String, Vec<PathBuf>> {
    prefixes.iter().fold(
        HashMap::new(),
        |mut map: HashMap<String, Vec<PathBuf>>, path| {
            let resource_index_path = path
                .join("share")
                .join("ament_index")
                .join("resource_index")
                .join(resource_type);

            if let Ok(dirs) = fs::read_dir(resource_index_path) {
                dirs.into_iter()
                    .flatten()
                    .filter(|entry| entry.path().is_file())
                    .for_each(|e| {
                        map.entry(e.file_name().to_string_lossy().into_owned())
                            .or_default()
                            .push(path.to_owned())
                    });
            }

            map
        },
    )
}

/// Returns all resources of the given type found in the prefixes from `AMENT_PREFIX_PATH`.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn resources(resource_type: &str) -> Result<HashMap<String, Vec<PathBuf>>, std::env::VarError> {
    Ok(resources_in(resource_type, &prefixes()?))
}

/// Finds a resource by name and type under the provided ament prefixes.
///
/// Returns the list of prefixes containing the resource, or `None` if no matching
/// resource marker file is found.
pub fn find_resource_in(
    resource_name: &str,
    resource_type: &str,
    prefixes: &[PathBuf],
) -> Option<Vec<PathBuf>> {
    resources_in(resource_type, prefixes)
        .get(resource_name)
        .cloned()
}

/// Finds a resource by name and type using the prefixes from `AMENT_PREFIX_PATH`.
///
/// Returns the list of prefixes containing the resource, or `None` if no matching
/// resource marker file is found.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn find_resource(resource_name: &str, resource_type: &str) -> Result<Option<Vec<PathBuf>>, std::env::VarError> {
    Ok(find_resource_in(resource_name, resource_type, &prefixes()?))
}

/// Returns all packages found under the provided ament prefixes.
pub fn packages_in(prefixes: &[PathBuf]) -> HashMap<String, Vec<PathBuf>> {
    resources_in("packages", prefixes)
}

/// Returns all packages found in the prefixes from `AMENT_PREFIX_PATH`.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn packages() -> Result<HashMap<String, Vec<PathBuf>>, std::env::VarError> {
    Ok(packages_in(&prefixes()?))
}

/// Finds a package under the provided ament prefixes.
///
/// Returns the list of prefixes containing the package, or `None` if the package is not found.
pub fn find_package_in(package: &str, prefixes: &[PathBuf]) -> Option<Vec<PathBuf>> {
    find_resource_in(package, "packages", prefixes)
}

/// Finds a package using the prefixes from `AMENT_PREFIX_PATH`.
///
/// Returns `None` if the package is not found.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn find_package(package: &str) -> Result<Option<Vec<PathBuf>>, std::env::VarError> {
    Ok(find_package_in(package, &prefixes()?))
}

/// Returns the share directories for a package under the provided ament prefixes.
///
/// The returned paths are formed by appending `share/<package>` to each prefix
/// where the package was found.
pub fn package_share_dirs_in(package: &str, prefixes: &[PathBuf]) -> Option<Vec<PathBuf>> {
    find_package_in(package, prefixes).map(|paths| {
        paths
            .iter()
            .map(|path| path.join("share").join(package))
            .collect()
    })
}

/// Returns the share directories for a package using prefixes from `AMENT_PREFIX_PATH`.
///
/// Returns `None` if the package is not found.
///
/// # Errors
///
/// A `std::env::VarError` is returned if the `AMENT_PREFIX_PATH` environment variable is not set.
pub fn package_share_dirs(package: &str) -> Result<Option<Vec<PathBuf>>, std::env::VarError> {
    Ok(package_share_dirs_in(package, &prefixes()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use itertools::Itertools;

    struct AmentIndexTestFixture<'a> {
        manifest: Vec<(&'a str, Vec<&'a str>)>,
        pub fs: assert_fs::TempDir,
    }

    impl<'a> AmentIndexTestFixture<'a> {
        pub fn new(manifest: Vec<(&'a str, Vec<&'a str>)>) -> Self {
            let fs = assert_fs::TempDir::new().unwrap();

            for (package, paths) in &manifest {
                for path in paths {
                    fs.child(path)
                        .child("share")
                        .child("ament_index")
                        .child("resource_index")
                        .child("packages")
                        .create_dir_all()
                        .unwrap();

                    fs.child(path)
                        .child("share")
                        .child("ament_index")
                        .child("resource_index")
                        .child("packages")
                        .child(package)
                        .touch()
                        .unwrap();
                }
            }

            Self { manifest, fs }
        }

        pub fn prefixes(&self) -> Vec<PathBuf> {
            self.manifest
                .iter()
                .flat_map(|(_, paths)| paths)
                .map(|path| self.fs.path().join(path))
                .unique()
                .collect()
        }

        pub fn expected(&self) -> HashMap<String, Vec<PathBuf>> {
            self.manifest
                .iter()
                .map(|(package, paths)| {
                    (
                        package.to_string(),
                        paths.iter().map(|path| self.fs.path().join(path)).collect(),
                    )
                })
                .collect()
        }
    }

    #[test]
    fn empty_prefixes() {
        assert!(packages_in(&[]).is_empty());
    }

    #[test]
    fn multiple_packages_and_prefixes() {
        let fixture = AmentIndexTestFixture::new(vec![
            (
                "common_interfaces",
                vec!["my/workspace/install/common_interfaces", "opt/ros/rolling"],
            ),
            ("ros_core", vec!["opt/ros/rolling"]),
        ]);

        assert_eq!(packages_in(&fixture.prefixes()), fixture.expected());
    }

    #[test]
    fn package_prefixes_preserve_input_prefix_order() {
        let fixture = AmentIndexTestFixture::new(vec![(
            "shared_package",
            vec!["second_prefix", "first_prefix", "third_prefix"],
        )]);

        let packages = packages_in(&fixture.prefixes());

        let shared_package = packages.get("shared_package").unwrap();
        assert_eq!(
            shared_package[0].iter().last().unwrap().to_str(),
            Some("second_prefix")
        );
        assert_eq!(
            shared_package[1].iter().last().unwrap().to_str(),
            Some("first_prefix")
        );
        assert_eq!(
            shared_package[2].iter().last().unwrap().to_str(),
            Some("third_prefix")
        );
    }

    #[test]
    fn find_nonexistent_package() {
        assert_eq!(find_package_in("nonexistent_package", &[]), None);
    }

    #[test]
    fn find_real_package() {
        let fixture = AmentIndexTestFixture::new(vec![
            (
                "common_interfaces",
                vec!["my/workspace/install/common_interfaces", "opt/ros/rolling"],
            ),
            ("ros_core", vec!["opt/ros/rolling"]),
        ]);

        let packages = find_package_in("common_interfaces", &fixture.prefixes()).unwrap();

        let expected = fixture.expected().get("common_interfaces").unwrap().clone();

        assert_eq!(packages, expected);
    }

    #[test]
    fn find_nonexistent_package_share_dirs() {
        let fixture = AmentIndexTestFixture::new(vec![]);

        let share_dirs = package_share_dirs_in("common_interfaces", &fixture.prefixes());

        assert_eq!(share_dirs, None);
    }

    #[test]
    fn test_package_share_dirs() {
        let fixture = AmentIndexTestFixture::new(vec![(
            "common_interfaces",
            vec!["my/workspace/install/common_interfaces", "opt/ros/rolling"],
        )]);

        let share_dirs = package_share_dirs_in("common_interfaces", &fixture.prefixes()).unwrap();

        assert_eq!(
            share_dirs,
            vec![
                fixture
                    .fs
                    .join("my/workspace/install/common_interfaces/share/common_interfaces"),
                fixture.fs.join("opt/ros/rolling/share/common_interfaces"),
            ]
        );
    }

    #[test]
    fn find_real_resources() {
        let fixture = AmentIndexTestFixture::new(vec![
            (
                "common_interfaces",
                vec!["my/workspace/install/common_interfaces", "opt/ros/rolling"],
            ),
            ("ros_core", vec!["opt/ros/rolling"]),
        ]);

        let resources = resources_in("packages", &fixture.prefixes());

        assert_eq!(resources, fixture.expected());

        let common_interfaces =
            find_resource_in("common_interfaces", "packages", &fixture.prefixes()).unwrap();

        assert_eq!(
            common_interfaces,
            vec![
                fixture.fs.join("my/workspace/install/common_interfaces"),
                fixture.fs.join("opt/ros/rolling"),
            ]
        );
    }

    #[test]
    fn find_package_matches_find_package_in_with_prefixes_from_env() {
        let fixture = AmentIndexTestFixture::new(vec![(
            "rcl_interfaces",
            vec!["my/workspace/install/rcl_interfaces", "opt/ros/rolling"],
        )]);

        let prefixes = fixture.prefixes();
        std::env::set_var(AMENT_PREFIX_PATH_ENV_VAR, std::env::join_paths(&prefixes).unwrap());

        assert_eq!(
            find_package_in("rcl_interfaces", &prefixes),
            find_package("rcl_interfaces").unwrap()
        );
    }

    #[test]
    fn resources_ignore_directories() {
        let fs = assert_fs::TempDir::new().unwrap();

        fs.child("prefix")
            .child("share")
            .child("ament_index")
            .child("resource_index")
            .child("packages")
            .child("not_a_package")
            .create_dir_all()
            .unwrap();

        let prefixes = vec![fs.path().join("prefix")];

        assert!(packages_in(&prefixes).is_empty());
        assert_eq!(find_package_in("not_a_package", &prefixes), None);
    }

    #[test]
    fn resources_support_arbitrary_resource_types() {
        let fs = assert_fs::TempDir::new().unwrap();

        fs.child("prefix")
            .child("share")
            .child("ament_index")
            .child("resource_index")
            .child("plugins")
            .create_dir_all()
            .unwrap();

        fs.child("prefix")
            .child("share")
            .child("ament_index")
            .child("resource_index")
            .child("plugins")
            .child("my_plugin")
            .touch()
            .unwrap();

        let prefixes = vec![fs.path().join("prefix")];

        let mut expected = HashMap::new();
        expected.insert("my_plugin".to_string(), vec![fs.path().join("prefix")]);

        assert_eq!(resources_in("plugins", &prefixes), expected);
        assert_eq!(
            find_resource_in("my_plugin", "plugins", &prefixes),
            Some(vec![fs.path().join("prefix")])
        );
    }
}
