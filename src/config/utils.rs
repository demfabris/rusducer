use std::path::PathBuf;

use xdg::BaseDirectories;

pub mod default_filenames {
    pub const CONFIG_FILE: &str = "config.yaml";
    pub const CONFIG_PREFIX: &str = "rusducer";
}

pub fn default_config_xdg_dir() -> Option<PathBuf> {
    let xdg = BaseDirectories::with_prefix(default_filenames::CONFIG_PREFIX)
        .expect("Failed to find config directory");

    xdg.find_config_file(default_filenames::CONFIG_FILE)
}
