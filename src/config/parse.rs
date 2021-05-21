use serde_yaml::{from_slice, Error};
use std::fs::read;
use std::path::PathBuf;

use crate::config::Configuration;

pub fn parse_yaml_config(path: &PathBuf) {
    let buffer = read(path).expect("Couldn't read file");

    let yaml: Result<Configuration, Error> = from_slice(&buffer);
    println!("{:?}", yaml.unwrap());

    todo!();
}
