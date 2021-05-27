use crate::config::Configuration;
use crate::worker::Worker;
use clap::ArgMatches;

pub struct Core<'runtime> {
    flags: Option<&'runtime ArgMatches<'runtime>>,
    config: Option<Configuration>,
    dry: bool,
}

impl<'r> Core<'r> {
    pub fn new() -> Self {
        Self {
            config: None,
            flags: None,
            dry: false,
        }
    }

    pub fn from_config(self, config: Configuration) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn with_flags(self, flags: &'r ArgMatches) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn run(&self) {
        let mut worker = Worker::new();

        if let Some(flags) = self.flags {
            if flags.is_present("version") {
                println!("v0.1.0");
                worker.dry = true;
            }

            if flags.is_present("list") {
                worker.dry = true;
            }

            if flags.is_present("silent") {
                worker.silent = true;
            }

            if flags.is_present("delay") {
                let delay = flags.value_of("delay").expect("Invalid delay argument");
                worker.delay = delay.parse::<u32>().unwrap();
            }
        }

        println!("hi");
        todo!()
    }
}
