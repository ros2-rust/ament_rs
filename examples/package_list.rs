use ament_rs::*;

fn main() {
    if let Ok(ament) = Ament::new() {
        println!("{:#?}", ament.get_packages_prefixes())
    } else {
        eprintln!(
            "environment variable '{}' is not set or empty",
            AMENT_PREFIX_PATH_ENV_VAR
        );
    }
}
