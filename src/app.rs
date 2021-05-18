use clap::{App, Arg};

pub struct Runtime;

impl Runtime {
    pub fn build(&self) -> App<'static, 'static> {
        App::new("Rusducer")
            .version("0.1.0")
            .author("Fabricio7p, <fabricio7p@protonmail.com>")
            .about("General purpose reducer-like application")
            .arg(
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .help("Display current version"),
            )
            .arg(
                Arg::with_name("silent")
                    .short("s")
                    .long("silent")
                    .help("Omit output"),
            )
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .help("Provide custom config file")
                    .value_name("CONFIG")
                    .takes_value(true),
            )
    }
}
