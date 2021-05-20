use serde_yaml::{from_slice, Error, Value};
use std::fs::read;
use std::path::PathBuf;

use crate::config::Action;

pub fn parse_yaml_config(path: &PathBuf) {
    let buffer = read(path).expect("Couldn't read file");

    let yaml: Result<Value, Error> = from_slice(&buffer);
    println!("{:?}", yaml);
    todo!();
}
