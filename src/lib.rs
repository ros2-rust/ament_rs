pub const AMENT_PREFIX_PATH_ENV_VAR: &str = "AMENT_PREFIX_PATH";

mod packages;
mod resources;
mod search_paths;

pub use packages::{get_package_prefix, get_package_share_directory, get_packages_with_prefixes};
pub use resources::{
    find_resource, find_resource_from, get_resource, get_resource_from, get_resource_prefix_from,
    get_resources, get_resources_from, get_resources_with_prefixes_from, has_resource,
    has_resource_from, list_all_prefixes_of_resources, list_prefix_of_resources,
};
pub use search_paths::get_search_paths;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
