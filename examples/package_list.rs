use ament_rs::*;

fn main() {
    match get_packages_with_prefixes() {
        Ok(package_list) => println!("{:#?}", package_list),
        Err(_) => eprintln!(
            "environment variable '{}' is not set or empty",
            AMENT_PREFIX_PATH_ENV_VAR
        ),
    }
}
