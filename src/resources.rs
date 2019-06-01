use crate::get_search_paths;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;

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
            .filter_entry(|e| {
                e.file_type().is_file()
                    && e.file_name()
                        .to_str()
                        .map(|s| !s.starts_with('.'))
                        .unwrap_or(true)
            })
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

pub fn get_resources_with_prefixes_from(
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> HashMap<String, Vec<String>> {
    list_all_prefixes_of_resources(resource_type, prefixes)
}

pub fn get_resource_prefix_from(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<String> {
    list_all_prefixes_of_resource(resource_name, resource_type, prefixes).nth(0)
}

pub fn get_resource_from(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<Result<(Vec<u8>, String), std::io::Error>> {
    list_all_prefixes(resource_type, prefixes)
        .map(|(prefix, path)| (prefix, path.join(resource_name.as_ref())))
        .filter(|(_, path)| path.is_file())
        .map(|(prefix, path)| {
            let mut buffer = vec![];
            std::fs::File::open(path)
                .and_then(|mut file| file.read_to_end(&mut buffer))
                .map(|_| (buffer, prefix))
        })
        .nth(0)
}

pub fn get_resources_from(
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> HashMap<String, String> {
    list_prefix_of_resources(resource_type, prefixes).collect()
}

pub fn find_resource_from(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<String> {
    list_all_prefixes(resource_type, prefixes)
        .map(|(prefix, path)| (prefix, path.join(resource_name.as_ref())))
        .filter(|(_, path)| path.is_file())
        .map(|(prefix, _)| prefix)
        .nth(0)
}

pub fn has_resource_from(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
    prefixes: impl IntoIterator<Item = impl AsRef<str>>,
) -> bool {
    find_resource_from(resource_name, resource_type, prefixes).is_some()
}

pub enum GetResourceError {
    VarError(std::env::VarError),
    IoError(std::io::Error),
    ResourceNotFound,
}

pub fn get_resource(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
) -> Result<(Vec<u8>, String), GetResourceError> {
    get_resource_from(
        resource_name,
        resource_type,
        get_search_paths().map_err(GetResourceError::VarError)?,
    )
    .ok_or(GetResourceError::ResourceNotFound)
    .and_then(|result| result.map_err(GetResourceError::IoError))
}

pub fn get_resources(
    resource_type: impl AsRef<str>,
) -> Result<HashMap<String, String>, std::env::VarError> {
    Ok(get_resources_from(resource_type, get_search_paths()?))
}

pub fn find_resource(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
) -> Result<Option<String>, std::env::VarError> {
    Ok(find_resource_from(
        resource_name,
        resource_type,
        get_search_paths()?,
    ))
}

pub fn has_resource(
    resource_name: impl AsRef<str>,
    resource_type: impl AsRef<str>,
) -> Result<bool, std::env::VarError> {
    Ok(has_resource_from(
        resource_name,
        resource_type,
        get_search_paths()?,
    ))
}
