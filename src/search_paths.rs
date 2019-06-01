// pub fn get_search_paths() {
// "Environment variable '{}' is not set or empty".format(AMENT_PREFIX_PATH_ENV_VAR))
// }

use crate::AMENT_PREFIX_PATH_ENV_VAR;

pub fn get_search_paths() -> Result<Vec<String>, std::env::VarError> {
    Ok(std::env::var(AMENT_PREFIX_PATH_ENV_VAR)?
        .split(':')
        .map(str::to_string)
        .collect())
}
