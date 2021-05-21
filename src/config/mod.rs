use serde_derive::Deserialize;
use std::{ffi::OsString, path::PathBuf};

mod parse;
mod utils;

use parse::parse_yaml_config;
use utils::default_config_xdg_dir;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub actions: Option<Vec<Action>>,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    pub action: Option<String>,
    pub description: Option<String>,
    pub files: Option<Vec<String>>,
    pub destination: Option<String>,
    pub replace: Option<bool>,
    pub command: Option<String>,
    pub side_effect: Option<String>,
}

impl Configuration {
    pub fn with_none() -> Self {
        Self { actions: None }
    }

    pub fn from_file(path_str: &str) -> Self {
        let path: PathBuf = path_str.into();
        parse_yaml_config(&path).expect("Couldn't read custom config file")
    }
}

impl Default for Configuration {
    fn default() -> Self {
        match default_config_xdg_dir() {
            Some(path) => parse_yaml_config(&path).expect("Couldn't read config file"),
            _ => Self::with_none(),
        }
    }
}
