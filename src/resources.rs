use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) fn filter_path(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    path.is_file()
        && path
            .file_name()
            .map(|file_name| {
                file_name
                    .to_str()
                    .map(|s| !s.starts_with('.'))
                    .unwrap_or(true)
            })
            .unwrap_or(false)
}

pub(crate) fn list_all_prefixes(
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = (PathBuf, PathBuf)> {
    let resource_type = resource_type.to_owned();

    prefixes.into_iter().map(move |prefix| {
        let prefix = prefix.as_ref();
        let resource_index_path = prefix
            .join("share")
            .join("ament_index")
            .join("resource_index")
            .join(&resource_type);

        (prefix.to_path_buf(), resource_index_path)
    })
}

pub(crate) fn list_all_prefixes_of_resources_disjointly(
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = (String, PathBuf)> {
    list_all_prefixes(resource_type, prefixes).flat_map(|(prefix, path)| {
        WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| filter_path(e.path()))
            .filter_map(Result::ok)
            .map(move |entry| {
                (
                    entry.file_name().to_string_lossy().to_string(),
                    prefix.clone(),
                )
            })
    })
}

pub fn list_prefix_of_resources(
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = (String, PathBuf)> {
    list_all_prefixes_of_resources_disjointly(resource_type, prefixes)
        .unique_by(|(resource_name, _)| resource_name.clone())
}

pub fn list_all_prefixes_of_resources(
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> HashMap<String, Vec<PathBuf>> {
    list_all_prefixes_of_resources_disjointly(resource_type, prefixes).fold(
        HashMap::new(),
        |mut prefixes_of_resources, (resource_name, prefix)| {
            prefixes_of_resources
                .entry(resource_name)
                .or_default()
                .push(prefix);
            prefixes_of_resources
        },
    )
}

pub fn list_all_prefixes_of_resource(
    resource_name: &str,
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> impl Iterator<Item = PathBuf> {
    let resource_name = resource_name.to_owned();

    list_all_prefixes_of_resources_disjointly(resource_type, prefixes)
        .filter(move |(found_name, _)| found_name == &resource_name)
        .map(|(_, prefix)| prefix)
}

pub fn get_resources_prefixes(
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> HashMap<String, Vec<PathBuf>> {
    list_all_prefixes_of_resources(resource_type, prefixes)
}

pub fn get_resource_prefix(
    resource_name: &str,
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> Option<PathBuf> {
    list_all_prefixes_of_resource(resource_name, resource_type, prefixes).next()
}

pub fn get_resource(
    resource_name: &str,
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> Option<(std::io::Result<Vec<u8>>, PathBuf)> {
    list_all_prefixes(resource_type, prefixes)
        .map(|(prefix, path)| (prefix, path.join(resource_name)))
        .filter(|(_, path)| filter_path(path))
        .map(|(prefix, path)| {
            let mut buffer = vec![];
            (
                std::fs::File::open(path)
                    .and_then(|mut file| file.read_to_end(&mut buffer))
                    .map(|_| buffer),
                prefix,
            )
        })
        .next()
}

pub fn get_resources_prefix(
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> HashMap<String, PathBuf> {
    list_prefix_of_resources(resource_type, prefixes).collect()
}

pub fn find_resource(
    resource_name: &str,
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> Option<PathBuf> {
    list_all_prefixes(resource_type, prefixes)
        .map(|(prefix, path)| (prefix, path.join(resource_name)))
        .filter(|(_, path)| filter_path(path))
        .map(|(prefix, _)| prefix)
        .next()
}

pub fn has_resource(
    resource_name: &str,
    resource_type: &str,
    prefixes: impl IntoIterator<Item = impl AsRef<Path>>,
) -> bool {
    find_resource(resource_name, resource_type, prefixes).is_some()
}
