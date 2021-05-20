extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;
extern crate xdg;

mod app;
mod config;

use crate::config::Configuration;
use app::Runtime;

fn main() {
    let matches = Runtime.build().get_matches();
    let conf = Configuration::default();

    println!("{:?}", matches);

    todo!()
}
