use crate::errors::DispatchError;
use std::ffi::OsString;

enum Kind {
    Move,
    Copy,
    Delete,
}

pub struct Action {
    files: Vec<OsString>,
    destination: OsString,
    kind: Kind,
    replace: bool,
}

pub trait Dispatchable {
    fn dispatch(&self) -> Result<u64, DispatchError<OsString>>;
}

impl Dispatchable for Action {
    fn dispatch(&self) -> Result<u64, DispatchError<OsString>> {
        match self.kind {
            Kind::Move => {
                let mut options = fs_extra::file::CopyOptions::new();
                options.overwrite = self.replace;

                for file in &self.files {
                    match fs_extra::file::move_file(file, &self.destination, &options) {
                        Ok(size) => println!("Moved {}", size),
                        Err(error) => println!("Failed {}", error),
                    }
                }

                Ok(0)
            }
            Kind::Copy => todo!(),
            Kind::Delete => todo!(),
        }
    }
}
