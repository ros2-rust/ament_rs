use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;

pub(crate) fn filter_path(path: impl AsRef<std::path::Path>) -> bool {
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

pub(crate) fn list_all_prefixes<'a>(
    resource_type: impl AsRef<str> + 'a,
    prefixes: impl IntoIterator<Item = impl AsRef<str> + 'a> + 'a,
) -> impl Iterator<Item = (String, PathBuf)> + 'a {
    prefixes.into_iter().map(move |prefix| {
        (
            prefix.as_ref().to_string(),
            [
                prefix.as_ref(),
                "share",
                "ament_index",
                "resource_index",
                resource_type.as_ref(),
            ]
            .iter()
            .collect(),
        )
    })
}

pub(crate) fn list_all_prefixes_of_resources_disjointly<'a>(
    resource_type: impl AsRef<str> + 'a,
    prefixes: impl IntoIterator<Item = impl AsRef<str> + 'a> + 'a,
) -> impl Iterator<Item = (String, String)> + 'a {
    list_all_prefixes(resource_type, prefixes).flat_map(|(prefix, path)| {
        WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| filter_path(&e.path()))
            .filter_map(Result::ok)
            .map(move |entry| {
                (
                    entry.file_name().to_string_lossy().to_string(),
                    prefix.clone(),
                )
            })
    })
}

pub fn list_prefix_of_resources<'a>(
    resource_type: impl AsRef<str> + 'a,
    prefixes: impl IntoIterator<Item = impl AsRef<str> + 'a> + 'a,
) -> impl Iterator<Item = (String, String)> + 'a {
    list_all_prefixes_of_resources_disjointly(resource_type, prefixes)
        .unique_by(|(resource_name, _)| resource_name.clone())
}

pub fn list_all_prefixes_of_resources(
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> HashMap<String, Vec<String>> {
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

pub fn list_all_prefixes_of_resource<'a>(
    resource_name: impl AsRef<str> + 'a,
    resource_type: impl AsRef<str> + 'a,
    prefixes: impl IntoIterator<Item = impl AsRef<str> + 'a> + 'a,
) -> impl Iterator<Item = String> + 'a {
    list_all_prefixes_of_resources_disjointly(resource_type, prefixes)
        .filter(move |(found_name, _)| found_name == resource_name.as_ref())
        .map(|(_, prefix)| prefix)
}

pub fn get_resources_prefixes(
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> HashMap<String, Vec<String>> {
    list_all_prefixes_of_resources(resource_type, prefixes)
}

pub fn get_resource_prefix(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<String> {
    list_all_prefixes_of_resource(resource_name, resource_type, prefixes).next()
}

pub fn get_resource(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<(std::io::Result<Vec<u8>>, String)> {
    list_all_prefixes(resource_type, prefixes)
        .map(|(prefix, path)| (prefix, path.join(resource_name.as_ref())))
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
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> HashMap<String, String> {
    list_prefix_of_resources(resource_type, prefixes).collect()
}

pub fn find_resource(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<String> {
    list_all_prefixes(resource_type, prefixes)
        .map(|(prefix, path)| (prefix, path.join(resource_name.as_ref())))
        .filter(|(_, path)| filter_path(path))
        .map(|(prefix, _)| prefix)
        .next()
}

pub fn has_resource(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> bool {
    find_resource(resource_name, resource_type, prefixes).is_some()
}
