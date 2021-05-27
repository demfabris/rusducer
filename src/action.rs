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
}

pub trait Dispatchable {
    fn dispatch(&self);
}

impl Dispatchable for Action {
    fn dispatch(&self) {
        match self.kind {
            Kind::Move => todo!(),
            Kind::Copy => todo!(),
            Kind::Delete => todo!(),
        }
    }
}
