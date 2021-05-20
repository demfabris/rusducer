use serde_derive::Deserialize;
use std::ffi::OsString;

mod parse;
mod utils;

use parse::parse_yaml_config;
use utils::default_config_xdg_dir;

#[derive(Deserialize, Debug)]
pub struct Action {
    id: Option<String>,
    description: Option<String>,
    files: Option<Vec<String>>,
    destination: Option<Vec<String>>,
    replace: Option<bool>,
    command: Option<String>,
    side_effect: Option<String>,
}

pub struct Configuration {
    actions: Vec<Action>,
}

impl Configuration {
    pub fn from_file() -> Configuration {
        todo!()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        match default_config_xdg_dir() {
            Some(path) => parse_yaml_config(&path),
            _ => (),
        }

        todo!()
    }
}

const DEFAULT_CONFIG: &str = r#"
---
- action:
    id: MOVE_MY_FILES
    description: Put files from place A to place B
    files:
      - "~/file"
      - "~/another_file"
    move:
      destination:
        - "~/Documents"
        - "~/Pictures"
      replace: true
    command: "sh -c custom_script.sh"
    side_effect: "..."
"#;
