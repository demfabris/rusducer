use std::{ffi::OsString, fs::metadata, path::Path};

enum Kind {
    Move,
    Copy,
    Delete,
}

pub struct Action {
    files: Vec<OsString>,
    destination: OsString,
    kind: Kind,
}

pub trait Dispatchable {
    fn dispatch(&self);
}

impl Dispatchable for Action {
    fn dispatch(&self) {
        match self.kind {
            Kind::Move => todo!(),
            Kind::Copy => {}
            Kind::Delete => todo!(),
        }
    }
}

fn try_execute<O: AsRef<Path>, D: AsRef<Path>>(
    origin: O,
    destination: D,
    replace: bool,
) -> Result<_, _> {
    if !replace {
        metadata(destination)?;
    }

    todo!()
}
