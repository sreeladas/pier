use super::{home, pier_err, xdg_config_home, PierError, PierResult};
use dirs;
use std::path::PathBuf;
pub const FALLBACK_QUERY_DISPLAY_WIDTH: usize = 100;

pub fn fallback_path() -> PierResult<PathBuf> {
    let default_config_paths: Vec<Option<PathBuf>> = vec![
        Some(PathBuf::from("pier.toml")),
        xdg_config_home!("pier/config.toml"),
        xdg_config_home!("pier/config"),
        xdg_config_home!("pier.toml"),
        home!(".pier.toml"),
        home!(".pier"), // Kept the .pier path for backwards compatibility
    ];

    // Loops for a vector of possible paths and tries to generate config from the first
    // default path that exists.
    for config_path in default_config_paths {
        if let Some(path) = config_path {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    pier_err!(PierError::NoConfigFile)
}
