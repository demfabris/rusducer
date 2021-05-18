extern crate clap;
extern crate serde;
extern crate serde_yaml;

mod app;
use app::Runtime;

fn main() {
    let matches = Runtime.build().get_matches();

    println!("{:?}", matches);

    todo!()
}
